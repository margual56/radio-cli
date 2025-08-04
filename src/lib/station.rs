use std::fmt;

use radiobrowser::ApiStation;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, Visitor},
    ser::SerializeStruct,
};

use crate::{Config, ConfigError, ConfigErrorCode};

#[derive(Debug, Clone, PartialEq)]
pub struct Station(pub ApiStation);

impl From<ApiStation> for Station {
    fn from(api_station: ApiStation) -> Self {
        Station(api_station)
    }
}

impl Into<ApiStation> for Station {
    fn into(self) -> ApiStation {
        self.0.clone()
    }
}

impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

impl Station {
    pub fn new(name: String, url: String) -> Station {
        Station::from(ApiStation {
            name,
            url,
            changeuuid: String::default(),
            stationuuid: String::default(),
            serveruuid: None,
            url_resolved: String::default(),
            homepage: String::default(),
            favicon: String::default(),
            tags: String::default(),
            country: String::default(),
            countrycode: String::default(),
            iso_3166_2: None,
            state: String::default(),
            language: String::default(),
            languagecodes: None,
            votes: 0,
            lastchangetime_iso8601: None,
            codec: String::default(),
            bitrate: 0,
            hls: 0,
            lastcheckok: 0,
            lastchecktime_iso8601: None,
            lastcheckoktime_iso8601: None,
            lastlocalchecktime_iso8601: None,
            clicktimestamp_iso8601: None,
            clickcount: 0,
            clicktrend: 0,
            ssl_error: None,
            geo_lat: None,
            geo_long: None,
            has_extended_info: None,
        })
    }
}

impl Serialize for Station {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize only the name and url fields of the .0
        let mut tup = serializer.serialize_struct("Station", 2)?;
        tup.serialize_field("station", &self.0.name.clone())?;
        tup.serialize_field("url", &self.0.url.clone())?;
        tup.end()
    }
}

impl<'de> Deserialize<'de> for Station {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Station,
            Url,
        }

        struct StationVisitor;

        impl<'de> Visitor<'de> for StationVisitor {
            type Value = Station;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map with fields `station` and `url`")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut name = None;
                let mut url = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Station => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("station"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Url => {
                            if url.is_some() {
                                return Err(de::Error::duplicate_field("url"));
                            }
                            url = Some(map.next_value()?);
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("station"))?;
                let url = url.ok_or_else(|| de::Error::missing_field("url"))?;

                Ok(Station::new(name, url))
            }
        }

        deserializer.deserialize_map(StationVisitor)
    }
}

pub fn add_station(name: &String, url: &String, config: &Config) -> Result<Config, ConfigError> {
    if name.is_empty() || url.is_empty() {
        return Err(ConfigError {
            code: ConfigErrorCode::InvalidStation,
            message: String::from("Invalid station"),
            extra: String::from("Station name and URL cannot be empty"),
        });
    }

    if config.data.iter().any(|s| s.0.name.eq(name)) {
        return Err(ConfigError {
            code: ConfigErrorCode::DuplicateStation,
            message: String::from("Duplicate station"),
            extra: String::from("Station name already exists"),
        });
    }

    let station = Station::new(name.clone(), url.clone());
    let mut new_config = config.clone();
    new_config.data.push(station);

    new_config.save();

    Ok(new_config)
}

pub fn remove_station(station_name: String, config: &Config) {
    let mut new_config = config.clone();
    let index = new_config
        .data
        .iter()
        .position(|s| s.0.name == station_name);

    if let Some(index) = index {
        new_config.data.remove(index);
        new_config.save();
    }
}

// pub fn edit_station(index: usize, name: String, url: String, config: &Config) {
//     if index < config.data.len() {
//         let mut new_config = config.clone();
//         new_config.data[index] = Station::new(name, url);

//         new_config.save();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_serialize_station() {
        let station = Station::new("My Station".to_string(), "http://example.com".to_string());

        let expected_json = r#"{"station":"My Station","url":"http://example.com"}"#;
        let serialized_json = to_string(&station).unwrap();
        assert_eq!(serialized_json, expected_json);
    }
    #[test]
    fn test_deserialize_station() {
        let json_str = r#"{
            "station": "My Station",
            "url": "http://example.com"
        }"#;

        let expected_station =
            Station::new("My Station".to_string(), "http://example.com".to_string());

        let deserialized_station: Result<Station, _> = from_str(json_str);
        assert!(deserialized_station.is_ok());
        assert_eq!(deserialized_station.unwrap(), expected_station);
    }

    #[test]
    fn test_deserialize_station_missing_fields() {
        let json_str_missing_station = r#"{
            "url": "http://example.com"
        }"#;
        let deserialized_station_missing_station: Result<Station, _> =
            from_str(json_str_missing_station);
        assert!(deserialized_station_missing_station.is_err());
        assert_eq!(
            deserialized_station_missing_station
                .unwrap_err()
                .to_string(),
            "missing field `station`"
        );

        let json_str_missing_url = r#"{
            "station": "My Station"
        }"#;
        let deserialized_station_missing_url: Result<Station, _> = from_str(json_str_missing_url);
        assert!(deserialized_station_missing_url.is_err());
        assert_eq!(
            deserialized_station_missing_url.unwrap_err().to_string(),
            "missing field `url`"
        );
    }

    #[test]
    fn test_deserialize_station_extra_field() {
        let json_str_extra_field = r#"{
            "station": "My Station",
            "url": "http://example.com",
            "extra": "value"
        }"#;
        let deserialized_station_extra_field: Result<Station, _> = from_str(json_str_extra_field);
        assert!(deserialized_station_extra_field.is_ok());
        assert_eq!(
            deserialized_station_extra_field.unwrap(),
            Station::new("My Station".to_string(), "http://example.com".to_string(),)
        );
    }

    #[test]
    fn test_deserialize_station_duplicate_fields() {
        let json_str_duplicate_station = r#"{
            "station": "My Station",
            "station": "Another Station",
            "url": "http://example.com"
        }"#;
        let deserialized_station_duplicate_station: Result<Station, _> =
            from_str(json_str_duplicate_station);
        assert!(deserialized_station_duplicate_station.is_err());
        assert_eq!(
            deserialized_station_duplicate_station
                .unwrap_err()
                .to_string(),
            "duplicate field `station`"
        );

        let json_str_duplicate_url = r#"{
            "station": "My Station",
            "url": "http://example.com",
            "url": "http://anotherexample.com"
        }"#;
        let deserialized_station_duplicate_url: Result<Station, _> =
            from_str(json_str_duplicate_url);
        assert!(deserialized_station_duplicate_url.is_err());
        assert_eq!(
            deserialized_station_duplicate_url.unwrap_err().to_string(),
            "duplicate field `url`"
        );
    }
}
