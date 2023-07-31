use std::ops::Add;

use crate::calc::Degree;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PolarVector {
    angle: Degree,
    magnitude: f64,
}

impl PolarVector {
    pub fn to_vector(self) -> Vector {
        self.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn unit(&self) -> Vector {
        let Vector { x, y } = self;
        let mag = self.magnitude();

        (x / mag, y / mag).into()
    }

    pub fn magnitude(&self) -> f64 {
        let Vector { x, y } = self;

        (x * x + y * y).sqrt()
    }

    pub fn to_polar(&self) -> PolarVector {
        let angle = (self.y / self.x).atan().to_degrees().into();
        let magnitude = self.magnitude();

        PolarVector { angle, magnitude }
    }
}

impl From<PolarVector> for Vector {
    fn from(val: PolarVector) -> Self {
        let PolarVector { angle, magnitude } = val;

        let (x, y) = (angle.cos() * magnitude, angle.sin() * magnitude);
        Vector { x, y }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        let Vector { x: x1, y: y1 } = self;
        let Vector { x: x2, y: y2 } = rhs;

        let (x, y) = (x1 + x2, y1 + y2);
        Vector { x, y }
    }
}

impl From<(Degree, f64)> for PolarVector {
    fn from(val: (Degree, f64)) -> Self {
        let (angle, magnitude) = val;
        PolarVector { angle, magnitude }
    }
}

impl From<(f64, f64)> for Vector {
    fn from(val: (f64, f64)) -> Self {
        let (x, y) = val;
        Vector { x, y }
    }
}

#[cfg(test)]
mod test {
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    use crate::calc::Degree;

    use super::PolarVector;

    impl Arbitrary for PolarVector {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let degree = Degree::new(i64::arbitrary(g) as f64);
            let mag = u64::arbitrary(g) as f64;

            (degree, mag).into()
        }
    }

    #[quickcheck]
    fn test_polar(polar_vector: PolarVector) {
        if polar_vector.magnitude == 0. {
            return;
        }

        let vector = polar_vector.to_vector();

        let ratio = vector.y / vector.x;
        let tan = polar_vector.angle.tan();

        assert!(
            (ratio - tan).abs() < 10e-10,
            "{:?} - ratio={ratio} tan:{tan}",
            polar_vector.angle
        );
    }

    #[quickcheck]
    fn test_polar_unit(polar_vector: PolarVector) {
        if polar_vector.magnitude == 0. {
            return;
        }

        let vector = polar_vector.to_vector().unit();

        let ratio = vector.y / vector.x;
        let tan = polar_vector.angle.tan();

        assert!(
            (ratio - tan).abs() < 10e-10,
            "{:?} - ratio={ratio} tan:{tan}",
            polar_vector.angle
        );
    }

    #[quickcheck]
    fn test_polar_mag(polar_vector: PolarVector) {
        if polar_vector.magnitude == 0. {
            return;
        }

        let vector = polar_vector.to_vector();

        let polar_mag = polar_vector.magnitude;
        let vector_mag = vector.magnitude();

        let percent = (polar_mag - vector_mag) / vector_mag;

        assert!(
            percent.abs() < 10e-10,
            "{:?} - polar_mag={polar_mag} vector_mag:{vector_mag}",
            polar_vector.angle
        );
    }
}
