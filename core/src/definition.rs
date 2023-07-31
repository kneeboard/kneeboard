use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Detail {
    pub tail: Option<String>,
    pub pic: Option<String>,
    pub call_sign: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Plan {
    pub detail: Detail,
    pub diversions: Vec<Diversion>,
    pub routes: Vec<Route>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Velocity {
    pub angle: i64,
    pub speed: i64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Diversion {
    pub wind: Velocity,
    pub aircraft_speed: i64,
    pub variation: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FontType {
    Bold(String),
    Normal(String),
    Italics(String),
    Blank,
}

impl FontType {
    pub fn string_value(&self) -> Option<&str> {
        match self {
            FontType::Bold(value) => Some(value),
            FontType::Italics(value) => Some(value),
            FontType::Normal(value) => Some(value),
            FontType::Blank => None,
        }
    }

    pub fn set_value(&self, value: String) -> FontType {
        match self {
            FontType::Bold(_) => FontType::Bold(value),
            FontType::Italics(_) => FontType::Italics(value),
            FontType::Normal(_) => FontType::Normal(value),
            FontType::Blank => {
                if value.trim().is_empty() {
                    FontType::Blank
                } else {
                    FontType::Normal(value)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Route {
    pub legs: Vec<Leg>,
    pub notes: Vec<FontType>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Leg {
    pub from: String,
    pub to: String,
    pub safe: String,
    pub planned: String,
    pub speed: i64,
    pub course: i64,
    pub distance: i64,
    pub variation: i64,

    pub wind_direction: i64,
    pub wind_speed: i64,
}
