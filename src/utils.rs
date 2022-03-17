use crate::material::Scatterable;
use crate::ray::Ray;
use crate::sphere::hit_world;
use crate::sphere::Sphere;
use crate::vector::Vector;
use crate::vector::VectorType;

use rand::Rng;

pub fn ray_color(r: &Ray, world: &Vec<Sphere>, depth: u64) -> Vector {
    if depth <= 0 {
        return Vector::new(0.0, 0.0, 0.0, VectorType::Color);
    }

    match hit_world(world, r, 0.0001, f64::INFINITY) {
        Some(hit_record) => {
            let scattered = hit_record.material.scatter(r, &hit_record);

            match scattered {
                Some((scattered_ray, albedo)) => match scattered_ray {
                    Some(sr) => {
                        return albedo * ray_color(&sr, world, depth - 1);
                    }
                    None => albedo,
                },
                None => {
                    return Vector::new(0.0, 0.0, 0.0, VectorType::Color);
                }
            }
        }
        None => {
            let unit_direction = r.direction.get_unit_vector();
            let t = 0.5 * (unit_direction.data.1 + 1.0);
            return (1.0 - t) * Vector::new(1.0, 1.0, 1.0, VectorType::Color)
                + t * Vector::new(0.5, 0.7, 1.0, VectorType::Color);
        }
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    x
}

pub fn degrees_to_radians(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}

pub fn random_in_unit_disk() -> Vector {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            0.0,
            VectorType::Point,
        );

        if p.length_squared() > 1.0 {
            continue;
        }

        return p;
    }
}
