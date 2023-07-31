use core::definition::{Detail, Diversion, FontType, Leg, Plan, Route, Velocity};
use std::io;

impl From<serde_json::Error> for KneeboardError {
    fn from(value: serde_json::Error) -> Self {
        KneeboardError::Json(value)
    }
}

impl From<serde_yaml::Error> for KneeboardError {
    fn from(value: serde_yaml::Error) -> Self {
        KneeboardError::Yaml(value)
    }
}

impl From<io::Error> for KneeboardError {
    fn from(value: io::Error) -> Self {
        KneeboardError::File(value)
    }
}

#[derive(Debug)]
pub enum KneeboardError {
    String(String),
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
    File(io::Error),
}

impl KneeboardError {
    pub fn to_err_string(&self) -> String {
        match self {
            KneeboardError::String(value) => value.to_owned(),
            KneeboardError::Json(value) => value.to_string(),
            KneeboardError::Yaml(value) => value.to_string(),
            KneeboardError::File(value) => value.to_string(),
        }
    }
}

pub fn create_template_plan() -> Plan {
    let detail = {
        let tail = Some("Registation".to_owned());
        let pic = Some("PIC name".to_owned());
        let call_sign = Some("Call sign".to_owned());

        Detail {
            tail,
            pic,
            call_sign,
        }
    };

    let diversions = {
        let diverion1 = {
            let angle = 190;
            let speed = 20;
            let variation = 1;

            let wind = Velocity { angle, speed };

            let aircraft_speed = 100;

            Diversion {
                wind,
                aircraft_speed,
                variation,
            }
        };

        let diverion2 = {
            let angle = 260;
            let speed = 10;
            let variation = 1;

            let wind = Velocity { angle, speed };

            let aircraft_speed = 90;

            Diversion {
                wind,
                aircraft_speed,
                variation,
            }
        };

        vec![diverion1, diverion2]
    };

    let routes = vec![create_template_route()];

    Plan {
        detail,
        diversions,
        routes,
    }
}

pub fn create_template_route() -> Route {
    let legs = {
        let leg1 = {
            let from = "Place 1".to_owned();
            let to = "Place 2".to_owned();
            let safe = "1.8".to_owned();
            let planned = "2.2".to_owned();
            let speed = 100;
            let course = 60;
            let distance = 15;
            let variation = 1;

            let wind_direction = 270;
            let wind_speed = 20;

            Leg {
                from,
                to,
                safe,
                planned,
                speed,
                course,
                distance,
                variation,
                wind_direction,
                wind_speed,
            }
        };

        let leg2 = {
            let from = "Place 2".to_owned();
            let to = "Place 3".to_owned();
            let safe = "1.8".to_owned();
            let planned = "2.2".to_owned();
            let speed = 100;
            let course = 70;
            let distance = 10;
            let variation = -1;

            let wind_direction = 265;
            let wind_speed = 25;

            Leg {
                from,
                to,
                safe,
                planned,
                speed,
                course,
                distance,
                variation,
                wind_direction,
                wind_speed,
            }
        };

        vec![leg1, leg2]
    };

    let notes = {
        let normal = FontType::Normal("Normal note".to_owned());
        let bold = FontType::Bold("Bold note".to_owned());
        let italic = FontType::Italics("Italic note".to_owned());
        let blank = FontType::Blank;

        vec![normal, bold, italic, blank]
    };

    Route { legs, notes }
}

pub fn create_template_leg() -> Leg {
    let from = "From".to_owned();
    let to = "To".to_owned();
    let safe = "1.8".to_owned();
    let planned = "2.2".to_owned();
    let speed = 100;
    let course = 0;
    let distance = 10;
    let variation = 0;

    let wind_direction = 270;
    let wind_speed = 20;

    Leg {
        from,
        to,
        safe,
        planned,
        speed,
        course,
        distance,
        variation,
        wind_direction,
        wind_speed,
    }
}

pub fn create_template_diversion() -> Diversion {
    let angle = 190;
    let speed = 20;
    let variation = 1;

    let wind = Velocity { angle, speed };

    let aircraft_speed = 100;

    Diversion {
        wind,
        aircraft_speed,
        variation,
    }
}
