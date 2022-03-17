use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range

        let mut root = (-half_b - discriminant_sqrt) / a;

        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;

            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = r.at(root);
        let normal = (point - self.center) / self.radius;
        let front_face = r.direction.dot(&normal) < 0.0;

        return Some(HitRecord {
            t: root,
            point,
            normal: if front_face { normal } else { -normal },
            material: &self.material,
            front_face,
        });
    }
}

pub fn hit_world<'material>(
    world: &'material Vec<Sphere>,
    r: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord<'material>> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}
