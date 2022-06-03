use std::error::Error;

use crate::{station::Station, Config};
use inquire::{error::InquireError, Text};
use radiobrowser::{blocking::RadioBrowserAPI, ApiCountry, StationOrder};

pub fn get_countries() -> Result<Vec<ApiCountry>, Box<dyn Error>> {
    let api = match RadioBrowserAPI::new() {
        Ok(r) => r,
        Err(_e) => panic!("Failed to create Radio Browser API!"),
    };

    api.get_countries().send()
}

pub fn get_station(name: String, country_code: &str) -> Result<Station, InquireError> {
    let api = match RadioBrowserAPI::new() {
        Ok(r) => r,
        Err(_e) => panic!("Failed to create Radio Browser API!"),
    };

    match api
        .get_stations()
        .name(name)
        .countrycode(country_code)
        .send()
    {
        Ok(s) => match s.get(0) {
            Some(x) => Ok(Station {
                station: x.name.clone(),
                url: x.url.clone(),
            }),
            None => Err(InquireError::InvalidConfiguration(
                "Radio station does not exist".to_string(),
            )),
        },
        Err(_e) => Err(InquireError::OperationInterrupted),
    }
}

fn search_station(
    message: &str,
    placeholder: &str,
    country_code: &str,
) -> Result<String, InquireError> {
    let api = match RadioBrowserAPI::new() {
        Ok(r) => r,
        Err(_e) => panic!("Failed to create Radio Browser API!"),
    };

    Text::new(message)
        .with_placeholder(placeholder)
        .with_suggester(&|s: &str| {
            if s.len() > 3 {
                match api
                    .get_stations()
                    .countrycode(country_code)
                    .name(s)
                    .order(StationOrder::Clickcount)
                    .send()
                {
                    Ok(s) => s
                        .iter()
                        .map(|station| String::from(&station.name))
                        .collect(),
                    Err(_e) => Vec::new(),
                }
            } else {
                Vec::new()
            }
        })
        .prompt()
}

pub fn prompt(config: Config) -> Result<Station, InquireError> {
    let station = search_station(
        "Search for a station: ",
        "Station name",
        &config.country_code.clone(),
    );

    match station {
        Ok(s) => get_station(s, &config.country_code.clone()),
        Err(e) => Err(e),
    }
}
