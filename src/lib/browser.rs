use crate::station::Station;
use inquire::{error::InquireError, Text};
use radiobrowser::{blocking::RadioBrowserAPI, StationOrder};

pub fn get_station(name: String) -> Result<Station, InquireError> {
    let api = match RadioBrowserAPI::new() {
        Ok(r) => r,
        Err(_e) => panic!("Failed to create Radio Browser API!"),
    };

    match api
        .get_stations()
        .name(name)
        .order(StationOrder::Clickcount)
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

fn search_station(message: &str, placeholder: &str) -> Result<String, InquireError> {
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
                    .countrycode("ES")
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

pub fn prompt() -> Result<Station, InquireError> {
    let station = search_station("Search for a station: ", "Station name");

    match station {
        Ok(s) => get_station(s),
        Err(e) => Err(e),
    }
}
