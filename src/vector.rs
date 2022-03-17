use rand::Rng;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VectorType {
    Vector,
    Color,
    Point,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub data: (f64, f64, f64),
    pub data_type: VectorType,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64, data_type: VectorType) -> Self {
        Self {
            data: (x, y, z),
            data_type,
        }
    }

    pub fn len(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.data.0 * other.data.0 + self.data.1 * other.data.1 + self.data.2 * other.data.2
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.data.1 * other.data.2 - self.data.2 * other.data.1,
            self.data.2 * other.data.0 - self.data.0 * other.data.2,
            self.data.0 * other.data.1 - self.data.1 * other.data.0,
            self.data_type,
        )
    }

    pub fn get_unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn length_squared(&self) -> f64 {
        self.data.0 * self.data.0 + self.data.1 * self.data.1 + self.data.2 * self.data.2
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            VectorType::Vector,
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().get_unit_vector()
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Vector::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return -in_unit_sphere;
        }
    }

    pub fn near_zero(&self) -> bool {
        self.data.0.abs() < f64::EPSILON
            && self.data.1.abs() < f64::EPSILON
            && self.data.2.abs() < f64::EPSILON
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(
            self.data.0 + other.data.0,
            self.data.1 + other.data.1,
            self.data.2 + other.data.2,
            self.data_type,
        )
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(
            self.data.0 - other.data.0,
            self.data.1 - other.data.1,
            self.data.2 - other.data.2,
            self.data_type,
        )
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.data.0, -self.data.1, -self.data.2, self.data_type)
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(
            self.data.0 * other.data.0,
            self.data.1 * other.data.1,
            self.data.2 * other.data.2,
            self.data_type,
        )
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::new(
            self.data.0 * other,
            self.data.1 * other,
            self.data.2 * other,
            self.data_type,
        )
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        other.mul(self)
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self::new(
            self.data.0 / other.data.0,
            self.data.1 / other.data.1,
            self.data.2 / other.data.2,
            self.data_type,
        )
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self::new(
            self.data.0 / other,
            self.data.1 / other,
            self.data.2 / other,
            self.data_type,
        )
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        fuzzy_equal(self.data.0, other.data.0)
            && fuzzy_equal(self.data.1, other.data.1)
            && fuzzy_equal(self.data.2, other.data.2)
            && self.data_type == other.data_type
    }
}

pub fn fuzzy_equal(lhs: f64, rhs: f64) -> bool {
    let epsilon = 0.0001;
    (lhs - rhs).abs() < epsilon
}

mod tests {
    use super::*;

    #[test]
    fn create_vector() {
        let vector = Vector::new(0.0, 1.0, 2.0, VectorType::Vector);

        assert_eq!(vector.data.0, 0.0);
        assert_eq!(vector.data.1, 1.0);
        assert_eq!(vector.data.2, 2.0);
    }

    #[test]
    fn add_two_vectors() {
        let first = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);
        let second = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);

        let result = first + second;

        assert_eq!(result.data.0, 2.0);
        assert_eq!(result.data.1, 4.0);
        assert_eq!(result.data.2, 6.0);
        assert_eq!(result.data_type, VectorType::Vector);
    }

    #[test]
    fn sub_two_vectors() {
        let first = Vector::new(1.0, 2.0, 4.0, VectorType::Vector);
        let second = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);

        let result = first - second;

        assert_eq!(result.data.0, 0.0);
        assert_eq!(result.data.1, 0.0);
        assert_eq!(result.data.2, 1.0);
        assert_eq!(result.data_type, VectorType::Vector);
    }

    #[test]
    fn negate_vector() {
        let color = Vector::new(1.0, 2.0, 3.2, VectorType::Color);
        let color = -color;

        assert_eq!(color.data.0, -1.0);
        assert_eq!(color.data.1, -2.0);
        assert_eq!(color.data.2, -3.2);
        assert_eq!(color.data_type, VectorType::Color);
    }

    #[test]
    fn scale_vector_by_f64() {
        let point = Vector::new(1.0, 2.0, 3.6, VectorType::Point);

        let result = point * 2.0;

        assert_eq!(result.data.0, 2.0);
        assert_eq!(result.data.1, 4.0);
        assert_eq!(result.data.2, 7.2);
        assert_eq!(result.data_type, VectorType::Point);
    }

    #[test]
    fn multiply_two_vectors() {
        let first = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);
        let second = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);

        let result = first * second;
        let expected_result = Vector::new(1.0, 4.0, 9.0, VectorType::Vector);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn divide_two_vectors() {
        let first = Vector::new(1.6, 2.4, 1.2, VectorType::Vector);
        let second = Vector::new(2.0, 0.6, 0.2, VectorType::Vector);

        let result = first / second;
        let expected_result = Vector::new(0.8, 4.0, 6.0, VectorType::Vector);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn divide_vector_by_f64() {
        let vector = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);

        let result = vector / 2.0;

        assert_eq!(result.data.0, 0.5);
        assert_eq!(result.data.1, 1.0);
        assert_eq!(result.data.2, 1.5);
        assert_eq!(result.data_type, VectorType::Vector);
    }

    #[test]
    fn check_equality_of_f64() {
        assert!(fuzzy_equal(0.1 + 0.2, 0.3));
    }

    #[test]
    fn check_fuzzy_equal_on_vectors() {
        let vector = Vector::new(5.0, 0.9, 0.15, VectorType::Vector);
        let vector = vector / 3.0;
        let cpm_vector = Vector::new(1.666666666666, 0.3, 0.05, VectorType::Vector);

        assert_eq!(vector, cpm_vector);
    }

    #[test]
    fn different_types_does_not_equal() {
        let vector = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);
        let color = Vector::new(1.0, 2.0, 3.0, VectorType::Color);

        assert_ne!(vector, color);
    }

    #[test]
    fn length_of_vector() {
        let length = Vector::new(1.0, 2.0, 3.0, VectorType::Vector).len();

        assert!(fuzzy_equal(length, 3.741657));
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let first = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);
        let second = Vector::new(2.0, 3.0, 4.0, VectorType::Vector);

        let result = first.dot(&second);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let first = Vector::new(1.0, 2.0, 3.0, VectorType::Vector);
        let second = Vector::new(2.0, 3.0, 4.0, VectorType::Vector);

        let result = first.cross(&second);
        let expected_result = Vector::new(-1.0, 2.0, -1.0, VectorType::Vector);

        assert_eq!(result, expected_result)
    }

    #[test]
    fn get_unit_vector() {
        let result = Vector::new(1.0, 2.0, 3.0, VectorType::Vector).get_unit_vector();
        let expected_result = Vector::new(0.2672, 0.5345, 0.8017, VectorType::Vector);

        assert_eq!(result, expected_result)
    }
}
