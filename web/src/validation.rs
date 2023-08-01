use crate::messages::PlanMessage;

pub fn create_speed_validation(wind_speed: i64) -> impl Fn(i64) -> Option<PlanMessage> {
    move |value: i64| {
        if value <= 0 {
            Some(PlanMessage::SetMessage(
                "Speed cannot be zero or less".to_owned(),
            ))
        } else if value <= wind_speed {
            Some(PlanMessage::SetMessage(
                "Speed cannot less than the wind speed".to_owned(),
            ))
        } else {
            None
        }
    }
}

pub fn create_wind_speed_validation(air_speed: i64) -> impl Fn(i64) -> Option<PlanMessage> {
    move |value: i64| {
        if value <= 0 {
            Some(PlanMessage::SetMessage(
                "Wind speed cannot be zero or less".to_owned(),
            ))
        } else if air_speed <= value {
            Some(PlanMessage::SetMessage(
                "Wind speed cannot more than air speed".to_owned(),
            ))
        } else {
            None
        }
    }
}

pub fn nop_validation(_: i64) -> Option<PlanMessage> {
    None
}

pub fn validate_course(value: i64) -> Option<PlanMessage> {
    if value < 0 {
        Some(PlanMessage::SetMessage(
            "Course cannot be negative".to_owned(),
        ))
    } else {
        None
    }
}

pub fn validate_distance(value: i64) -> Option<PlanMessage> {
    if value <= 0 {
        Some(PlanMessage::SetMessage(
            "Distance cannot be zero or less".to_owned(),
        ))
    } else {
        None
    }
}

pub fn validate_wind_direction(value: i64) -> Option<PlanMessage> {
    if value < 0 {
        Some(PlanMessage::SetMessage(
            "Wind direction cannot be negative".to_owned(),
        ))
    } else {
        None
    }
}
