use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Detail {
    pub tail: Option<String>,
    pub pic: Option<String>,
    pub call_sign: Option<String>,
    pub field1: Option<String>,
    pub field2: Option<String>,
    pub field3: Option<String>,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Plan {
    #[serde(default)]
    pub detail: Detail,
    #[serde(default)]
    pub diversions: Vec<Diversion>,
    #[serde(default)]
    pub routes: Vec<Route>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holds: Vec<Hold>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aircraft_registrations: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pics: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub call_signs: Vec<String>,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Hold {
    pub description: String,
    pub right_hand: bool,
    pub in_bound_track: i64,
    pub wind: Velocity,
    pub aircraft_speed: i64,
    pub variation: i64,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Velocity {
    pub angle: i64,
    pub speed: i64,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Diversion {
    pub wind: Velocity,
    pub aircraft_speed: i64,
    pub variation: i64,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Route {
    #[serde(default)]
    pub name: String,
    pub legs: Vec<Leg>,
    pub notes: Vec<FontType>,
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ProfileConfig {
    pub aircraft_registrations: Vec<String>,
    pub pics: Vec<String>,
    pub call_signs: Vec<String>,
    pub saved_routes: Vec<SavedRoute>,
    #[serde(default)]
    pub saved_holds: Vec<SavedHold>,
    pub default_leg_values: DefaultLegValues,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SavedHold {
    pub name: String,
    pub description: String,
    pub right_hand: bool,
    pub in_bound_track: i64,
    pub aircraft_speed: i64,
    pub variation: i64,
    pub wind_angle: i64,
    pub wind_speed: i64,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SavedRoute {
    pub name: String,
    pub waypoints: String,
    pub legs: Vec<Leg>,
    pub notes: Vec<FontType>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DefaultLegValues {
    pub safe: String,
    pub planned: String,
    pub speed: i64,
    pub course: i64,
    pub distance: i64,
    pub variation: i64,
    pub wind_direction: i64,
    pub wind_speed: i64,
}

impl Default for DefaultLegValues {
    fn default() -> Self {
        DefaultLegValues {
            safe: "1.8".to_owned(),
            planned: "2.2".to_owned(),
            speed: 100,
            course: 0,
            distance: 10,
            variation: 0,
            wind_direction: 270,
            wind_speed: 20,
        }
    }
}
