use gtfs_rt::{FeedMessage, VehiclePosition};
use prost::Message;
use serde_json::{json, Value};

pub enum Line {
    Blue,
    G,
    Yellow,
    Number,
    Orange,
    Brown,
    L,
    SIR,
}

impl Line {
    pub fn from_str(line_name: &str) -> Option<Self> {
        match line_name.to_lowercase().as_str() { // Use lowercase for case-insensitive matching
            "blue" => Some(Line::Blue),
            "g" => Some(Line::G),
            "yellow" => Some(Line::Yellow),
            "number" => Some(Line::Number),
            "orange" => Some(Line::Orange),
            "brown" => Some(Line::Brown),
            "l" => Some(Line::L),
            "sir" => Some(Line::SIR),
            _ => None,
        }
    }

    pub fn api_endpoint(&self) -> &'static str {
        match self {
            Line::Blue => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-ace",
            Line::G => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-g",
            Line::Yellow => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-nqrw",
            Line::Number => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs",
            Line::Orange => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-bdfm",
            Line::Brown => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-jz",
            Line::L => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-l",
            Line::SIR => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-si",
        }
    }
}


pub struct MTA;

impl MTA {
    pub async fn get_line(line_name: &str) -> anyhow::Result<Value> {
        match Line::from_str(line_name) {
            Some(line) => {
                let endpoint = line.api_endpoint();
                let response_bytes = reqwest::get(endpoint).await?.bytes().await?;
                let feed = FeedMessage::decode(&*response_bytes)?;
                let mut vehicles: Vec<VehiclePosition> = vec![];

                for entity in feed.entity {
                    if let Some(vehicle) = entity.vehicle {
                        vehicles.push(vehicle);
                    }
                }

                Ok(json!(vehicles))
            },
            None => Err(anyhow::anyhow!("Invalid line name provided")), // Custom error handling
        }
    }
}

