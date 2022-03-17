use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;
use rand::Rng;

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vector)>;
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vector)> {
        match self {
            Material::Metal(m) => m.scatter(ray, hit_record),
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Dielectric(d) => d.scatter(ray, hit_record),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vector)> {
        let mut rng = rand::thread_rng();
        let attenuation = Vector::new(1.0, 1.0, 1.0, crate::vector::VectorType::Color);

        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r.direction.get_unit_vector();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>() {
            let direction = reflect(unit_direction, hit_record.normal);
            let scattered = Ray::new(hit_record.point, direction);
            Some((Some(scattered), attenuation))
        } else {
            let direction = refract(unit_direction, hit_record.normal, refraction_ratio);
            let scattered = Ray::new(hit_record.point, direction);
            Some((Some(scattered), attenuation))
        }
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vector)> {
        let reflected = reflect(ray.direction.get_unit_vector(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vector::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit_record.normal) > 0.0 {
            Some((Some(scattered), attenuation))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vector)> {
        let mut scatter_direction = hit_record.normal + Vector::random_in_unit_sphere();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;

        Some((Some(scattered), attenuation))
    }
}

fn reflect(v: Vector, n: Vector) -> Vector {
    return v - n * (2.0 * v.dot(&n));
}

fn refract(uv: Vector, n: Vector, etai_over_eatt: f64) -> Vector {
    let cos_theta: f64 = ((-uv).dot(&n)).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_eatt;
    let r_out_parallel = n * (-1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt());

    r_out_parallel + r_out_perp
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
