use std::error::Error;
use std::rc::Rc;

use crate::{station::Station, Config};
use inquire::{error::InquireError, Autocomplete, Text};
use radiobrowser::{blocking::RadioBrowserAPI, ApiCountry, ApiStation, StationOrder};

pub type StationCache = Rc<Vec<ApiStation>>;

#[derive(Debug, Clone)]
pub struct Stations {
    stations: StationCache,
}

impl Autocomplete for Stations {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let suggestions: Vec<String> = self
            .stations
            .iter()
            .filter_map(|station| {
                if station.name.to_lowercase().contains(&input.to_lowercase()) {
                    Some(station.name.clone())
                } else {
                    None
                }
            })
            .collect();
        Ok(suggestions)
    }

    fn get_completion(
        &mut self,
        _input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        match highlighted_suggestion {
            Some(suggestion) => Ok(Some(suggestion)),
            None => Err(inquire::CustomUserError::from("No suggestion available")),
        }
    }
}

pub struct Browser {
    api: RadioBrowserAPI,
    config: Rc<Config>,
    stations: StationCache,
}

impl Browser {
    pub fn new(
        config: Rc<Config>,
        cached_stations: Option<StationCache>,
    ) -> Result<(Browser, StationCache), Box<dyn Error>> {
        let api = match RadioBrowserAPI::new() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let stations = cached_stations.unwrap_or_else(|| {
            Rc::new(if let Some(code) = &config.country_code {
                match api
                    .get_stations()
                    .countrycode(code)
                    .order(StationOrder::Clickcount)
                    .send()
                {
                    Ok(s) => s,
                    Err(_e) => Vec::new(),
                }
            } else {
                match api.get_stations().order(StationOrder::Clickcount).send() {
                    Ok(s) => s,
                    Err(_e) => Vec::new(),
                }
            })
        });

        Ok((
            Browser {
                api,
                config,
                stations: stations.clone(),
            },
            stations,
        ))
    }

    pub fn get_countries() -> Result<Vec<ApiCountry>, Box<dyn Error>> {
        let api = match RadioBrowserAPI::new() {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        api.get_countries().send()
    }

    pub fn get_station(&self, name: String) -> Result<Station, InquireError> {
        if let Some(code) = self.config.country_code.clone() {
            return match self.api.get_stations().name(name).countrycode(code).send() {
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
            };
        } else {
            return match self.api.get_stations().name(name).send() {
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
            };
        }
    }

    fn search_station(&self, message: &str, placeholder: &str) -> Result<String, InquireError> {
        let max_lines = match self.config.max_lines {
            Some(x) => x,
            None => Text::DEFAULT_PAGE_SIZE,
        };

        Text::new(message)
            .with_placeholder(placeholder)
            // Deprecated: need to change to `with_autosuggester`
            // But for that, ApiStation needs to implement the Clone trait
            .with_autocomplete(Stations {
                stations: self.stations.clone(),
            })
            .with_page_size(max_lines)
            .prompt()
    }

    pub fn prompt(self) -> Result<Station, InquireError> {
        let station = self.search_station("Search for a station: ", "Names or keywords");

        match station {
            Ok(s) => self.get_station(s.to_string()),
            Err(e) => Err(e),
        }
    }
}
