use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct Station {
    pub station: String,
    pub url: String,
}

impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.station)
    }
}
