use std::fs;
use yaml_rust::YamlLoader;

pub enum Action {
    CreateIncident,
    UpdateStatus,
}

pub struct CachetHQ {
    pub api_url: String,
    pub token: String,
    pub component_id: u16,
    pub metric_id: Option<i64>,
    pub action: Vec<Option<Action>>,
    pub public_incidents: bool,
    pub latency_unit: super::latency_unit::LatencyUnit,
}

pub struct MySQLMonitor {
    pub expectations: Vec<super::expectation::Expectation>,
    pub mysql_uri: String,
    pub query: String,
    pub timeout: f64,
    pub frequency: u16,
}

pub fn parse_config(path: &str) -> (CachetHQ, MySQLMonitor) {
    let docs =
        YamlLoader::load_from_str(&fs::read_to_string(path).expect("Failed to open config.yml"))
            .unwrap();
    let doc = &docs[0];

    (
        CachetHQ {
            api_url: doc["cachet"]["api_url"].as_str().unwrap().to_string(),
            token: doc["cachet"]["token"].as_str().unwrap().to_string(),
            component_id: doc["cachet"]["component_id"].as_i64().unwrap() as u16,
            metric_id: doc["cachet"]["metric_id"].as_i64(),
            action: Vec::new(),
            public_incidents: doc["cachet"]["public_incidents"].as_bool().unwrap(),
            latency_unit: super::latency_unit::parse_latency(doc),
        },
        MySQLMonitor {
            expectations: super::expectation::parse_expectations(doc),
            mysql_uri: doc["mysql"]["uri"].as_str().unwrap().to_string(),
            query: doc["mysql"]["query"].as_str().unwrap().to_string(),
            timeout: doc["mysql"]["timeout"].as_f64().unwrap(),
            frequency: doc["frequency"].as_i64().unwrap() as u16,
        },
    )
}
