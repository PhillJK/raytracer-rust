use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord<'material> {
    pub point: Vector,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub material: &'material Material,
}
