use serde::{Deserialize, Serialize};

/// GTFS-RT feed groups. Each feed covers a set of subway routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Feed {
    /// 1, 2, 3, 4, 5, 6, 7, S (42nd St Shuttle)
    NumberedLines,
    /// A, C, E, H (Rockaway Shuttle), FS (Franklin Shuttle)
    ACE,
    /// B, D, F, M
    BDFM,
    /// G
    G,
    /// J, Z
    JZ,
    /// L
    L,
    /// N, Q, R, W
    NQRW,
    /// Staten Island Railway
    SIR,
}

impl Feed {
    pub fn url(&self) -> &'static str {
        match self {
            Feed::NumberedLines => {
                "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs"
            }
            Feed::ACE => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-ace",
            Feed::BDFM => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-bdfm",
            Feed::G => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-g",
            Feed::JZ => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-jz",
            Feed::L => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-l",
            Feed::NQRW => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-nqrw",
            Feed::SIR => "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/nyct%2Fgtfs-si",
        }
    }

    pub fn all() -> &'static [Feed] {
        &[
            Feed::NumberedLines,
            Feed::ACE,
            Feed::BDFM,
            Feed::G,
            Feed::JZ,
            Feed::L,
            Feed::NQRW,
            Feed::SIR,
        ]
    }
}

pub const ALERTS_URL: &str =
    "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/camsys%2Fsubway-alerts";

/// Map a single route letter/number to its GTFS-RT feed.
pub fn route_to_feed(route: &str) -> Option<Feed> {
    match route.to_uppercase().as_str() {
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "S" => Some(Feed::NumberedLines),
        "A" | "C" | "E" | "H" => Some(Feed::ACE),
        "B" | "D" | "F" | "M" => Some(Feed::BDFM),
        "G" => Some(Feed::G),
        "J" | "Z" => Some(Feed::JZ),
        "L" => Some(Feed::L),
        "N" | "Q" | "R" | "W" => Some(Feed::NQRW),
        "SI" | "SIR" => Some(Feed::SIR),
        _ => None,
    }
}

/// Direction of travel (suffix on GTFS-RT stop_id).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
}

impl Direction {
    pub fn suffix(&self) -> &'static str {
        match self {
            Direction::North => "N",
            Direction::South => "S",
        }
    }

    pub fn from_stop_id_suffix(stop_id: &str) -> Option<Self> {
        if stop_id.ends_with('N') {
            Some(Direction::North)
        } else if stop_id.ends_with('S') {
            Some(Direction::South)
        } else {
            None
        }
    }
}
