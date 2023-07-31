use crate::definition::Velocity as JSonVelocity;
use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degree {
    degrees: f64,
}

pub struct Radian {
    radians: f64,
}

impl From<f64> for Degree {
    fn from(val: f64) -> Self {
        Degree::new(val)
    }
}

impl Degree {
    pub fn new(degrees: f64) -> Self {
        Self {
            degrees: rationalise_degree(degrees),
        }
    }

    pub fn to_radians(&self) -> Radian {
        Radian::new(self.degrees.to_radians())
    }

    pub fn cos(&self) -> f64 {
        self.degrees.to_radians().cos()
    }

    pub fn sin(&self) -> f64 {
        self.degrees.to_radians().sin()
    }

    pub fn tan(&self) -> f64 {
        self.degrees.to_radians().tan()
    }

    pub fn reciprocal(&self) -> Self {
        Degree::new(self.degrees + 180.)
    }

    pub fn as_heading(&self) -> String {
        let rounded = self.degrees.round() as i64;

        if rounded < 10 {
            format!("00{rounded}")
        } else if rounded < 100 {
            format!("0{rounded}")
        } else {
            format!("{rounded}")
        }
    }
}

impl Add for Degree {
    type Output = Degree;

    fn add(self, rhs: Degree) -> Self::Output {
        let result = self.degrees + rhs.degrees;
        Degree::new(result)
    }
}

impl Sub for Degree {
    type Output = Degree;

    fn sub(self, rhs: Self) -> Self::Output {
        let result = self.degrees - rhs.degrees;
        Degree::new(result)
    }
}

impl AddAssign for Degree {
    fn add_assign(&mut self, rhs: Self) {
        self.degrees = rationalise_degree(self.degrees + rhs.degrees);
    }
}

fn rationalise_degree(degree: f64) -> f64 {
    if !degree.is_finite() || degree.is_nan() {
        panic!("{}", degree)
    }

    let clamped = degree % 360.;
    if clamped < 0. {
        360. + clamped
    } else {
        clamped
    }
}

impl Radian {
    pub fn new(radians: f64) -> Self {
        Self { radians }
    }

    pub fn to_degrees(&self) -> Degree {
        Degree::new(self.radians.to_degrees())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub speed: f64,
    pub bearing: Degree,
}

pub fn convert_velocity(velocity: &JSonVelocity) -> Velocity {
    let bearing = Degree::new(velocity.angle as f64);
    let speed = velocity.speed as f64;

    Velocity { bearing, speed }
}

#[derive(Debug)]
pub struct Heading {
    pub destination_bearing: Degree,
    pub speed_overground: f64,
    pub correction_angle: Degree,
    pub heading: Degree,
    pub heading_magnetic: Degree,
}

pub fn calc_aircraft(
    air_speed: f64,
    destination_bearing: Degree,
    variation: Degree,
    wind: &Velocity,
) -> Heading {
    let speed_overground = ground_speed(air_speed, destination_bearing, wind);
    let correction_angle = correction(air_speed, destination_bearing, wind);

    let heading = correction_angle + destination_bearing;
    let heading_magnetic = heading + variation;
    Heading {
        destination_bearing,
        speed_overground,
        correction_angle,
        heading,
        heading_magnetic,
    }
}

fn ground_speed(air_speed: f64, destination_bearing: Degree, wind: &Velocity) -> f64 {
    let angle = wind.bearing - destination_bearing;
    let b = (-2.0) * wind.speed * (angle.cos());
    let c = (wind.speed * wind.speed) - (air_speed * air_speed);

    let (x1, x2) = quadratic(1.0, b, c);

    x1.max(x2)
}

fn correction(air_speed: f64, destination_bearing: Degree, wind: &Velocity) -> Degree {
    let ratio = wind.speed / air_speed;

    let sigma = destination_bearing - wind.bearing;

    (sigma.sin() * ratio).asin().to_degrees().into()
}

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let inner_sqrt = ((b * b) - (4.0 * a * c)).sqrt();
    let bottom = 2.0 * a;

    let x1 = ((-b) + inner_sqrt) / bottom;
    let x2 = ((-b) - inner_sqrt) / bottom;

    (x1, x2)
}

#[cfg(test)]
mod tests {
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    use crate::{
        calc::{calc_aircraft, rationalise_degree, Velocity},
        vector::{PolarVector, Vector},
    };

    use super::Degree;

    impl Arbitrary for Velocity {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let speed = (u32::arbitrary(g) % 40) as f64;
            let bearing = ((u32::arbitrary(g) % 360) as f64).into();

            Self { speed, bearing }
        }
    }

    #[test]
    fn test_heading1() {
        let air_speed = 100.;
        let destination_bearing = 45_f64.into();

        let wind = Velocity {
            speed: 30.,
            bearing: (180. + 10_f64).into(),
        };

        let variation = Degree::new(0.);
        let heading = calc_aircraft(air_speed, destination_bearing, variation, &wind);

        assert_eq!(heading.speed_overground, 73.9338599585306);
        assert_eq!(heading.heading, Degree::new(35.091634438291294));
    }

    #[test]
    fn test_heading2() {
        let air_speed = 80.;
        let destination_bearing = 303_f64.into();

        let wind = Velocity {
            speed: 38.,
            bearing: (180. + 1_f64).into(),
        };
        let variation = Degree::new(0.);
        let heading = calc_aircraft(air_speed, destination_bearing, variation, &wind);

        assert_eq!(heading.speed_overground, 53.08530523729222);
        assert_eq!(heading.heading, Degree::new(326.75476721509176));
    }

    #[derive(Clone, Copy, Debug)]
    struct AirSpeed {
        speed: f64,
    }

    impl Arbitrary for AirSpeed {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let speed = 80. + (u32::arbitrary(g) % 30) as f64;

            Self { speed }
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct DestBearing {
        bearing: Degree,
    }

    impl Arbitrary for DestBearing {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let bearing = ((u32::arbitrary(g) % 360) as f64).into();

            Self { bearing }
        }
    }

    #[quickcheck]
    fn check_rationalise_degree(degree: f64) {
        if degree.is_infinite() || degree.is_nan() {
            return;
        }

        let actual = rationalise_degree(degree);

        if degree.is_finite() {
            assert!(actual >= 0., "Degree: {} = {}", degree, actual);
            assert!(actual <= 360., "Degree: {} = {}", degree, actual)
        }
    }

    #[test]
    fn check_rationalise_degree_negtive() {
        let actual = rationalise_degree(-1. - 360.);
        assert_eq!(actual, 359.)
    }

    #[test]
    fn check_rationalise_degree_postive() {
        let actual = rationalise_degree(1. + 360.);
        assert_eq!(actual, 1.)
    }

    #[test]
    fn check_rationalise_degree_360() {
        let actual = rationalise_degree(360.);
        assert_eq!(actual, 0.)
    }

    #[quickcheck]
    fn check_heading(air_speed: AirSpeed, destination_bearing: DestBearing, wind: Velocity) {
        let variation = Degree::new(0.);
        let heading = calc_aircraft(
            air_speed.speed,
            destination_bearing.bearing,
            variation,
            &wind,
        );

        let bearing_to_destination = destination_bearing.bearing;
        let aircraft_speed = air_speed.speed;

        let corrected_heading = heading.heading;

        let desired: PolarVector = (bearing_to_destination, aircraft_speed).into();
        let wind_vector: PolarVector = (wind.bearing, wind.speed).into();
        let heading_vector: PolarVector = (corrected_heading, aircraft_speed).into();

        let result = wind_vector.to_vector() + heading_vector.to_vector();

        const EPSILON: f64 = 0.000000001;

        assert_vector(
            format!(
                "wind={:?}, bearing_to_destination={:?}, corrected_heading={:?}",
                wind_vector, bearing_to_destination, corrected_heading
            )
            .as_str(),
            &result,
            &desired.to_vector(),
            EPSILON,
        );

        let ground_speed = result.magnitude();

        assert_float(
            "Ground speed",
            ground_speed,
            heading.speed_overground,
            EPSILON,
        );
    }

    fn assert_float(msg: &str, v1: f64, v2: f64, epsilon: f64) {
        let diff = (v1 - v2).abs();
        assert!(diff < epsilon, "{}: {} vs {} = {}", msg, v1, v2, diff);
    }

    fn assert_vector(msg: &str, v1: &Vector, v2: &Vector, epsilon: f64) {
        let Vector { x: x1, y: y1 } = v1.unit();
        let Vector { x: x2, y: y2 } = v2.unit();

        assert_float(format!("x1 vs x2 - {msg}").as_str(), x1, x2, epsilon);
        assert_float(format!("y1 vs y2 - {msg}").as_str(), y1, y2, epsilon);
    }
}
