use crate::ray::Ray;
use crate::utils;
use crate::vector::Vector;
use crate::vector::VectorType;

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    lens_radius: f64,
    u: Vector,
    v: Vector,
    w: Vector,
}

impl Camera {
    pub fn new(
        look_from: Vector,
        look_at: Vector,
        vup: Vector,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = utils::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).get_unit_vector();
        let u = (vup.cross(&w)).get_unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            w,
            v,
            u,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * utils::random_in_unit_disk();
        let offset = self.u * rd.data.0 + self.v * rd.data.1;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
