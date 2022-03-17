mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod utils;
mod vector;

use camera::Camera;
use material::{Lambertian, Material, Metal};
use rand::Rng;
use rayon::prelude::*;
use sphere::Sphere;
use vector::{Vector, VectorType};

use crate::material::Dielectric;

fn main() {
    //Image
    let aspect_ratio = 3.0 / 2.0;
    let width: u32 = 1200;
    let height = (width as f64 / aspect_ratio).floor() as u32;
    let samples_per_pixel: u32 = 1000;
    let max_depth: u64 = 50;

    //World
    let world = random_scene();

    //Camera
    let look_from = Vector::new(13.0, 2.0, 3.0, VectorType::Point);
    let look_at = Vector::new(0.0, 0.0, 0.0, VectorType::Point);
    let vup = Vector::new(0.0, 1.0, 0.0, VectorType::Vector);
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        distance_to_focus,
    );

    //Render
    println!("P3\n{} {}\n255\n", width, height);

    let pixels = render(height, width, samples_per_pixel, &world, camera, max_depth);

    for (i, _) in pixels.iter().enumerate().step_by(3) {
        println!("{} {} {}", pixels[i], pixels[i + 1], pixels[i + 2]);
    }
}

fn render(
    height: u32,
    width: u32,
    samples_per_pixel: u32,
    world: &Vec<Sphere>,
    camera: Camera,
    max_depth: u64,
) -> Vec<u8> {
    let mut pixels = vec![0; width as usize * height as usize * 3];
    let bands: Vec<(usize, &mut [u8])> = pixels
        .chunks_mut(width as usize * 3)
        .rev()
        .enumerate()
        .collect();

    bands.into_par_iter().for_each(|(i, band)| {
        render_line(
            band,
            samples_per_pixel,
            world,
            &camera,
            width,
            height,
            i,
            max_depth,
        )
    });

    return pixels;
}

fn render_line(
    pixels: &mut [u8],
    samples_per_pixel: u32,
    world: &Vec<Sphere>,
    camera: &Camera,
    width: u32,
    height: u32,
    y: usize,
    max_depth: u64,
) {
    let mut rng = rand::thread_rng();

    for x in 0..width {
        let mut pixel_colors = vec![0.0; 3];

        for _s in 0..samples_per_pixel {
            let u = (x as f64 + rng.gen::<f64>()) / (width as f64 - 1.0);
            let v = (y as f64 + rng.gen::<f64>()) / (height as f64 - 1.0);
            let r = camera.get_ray(u, v);
            let c = utils::ray_color(&r, world, max_depth);

            pixel_colors[0] += c.data.0;
            pixel_colors[1] += c.data.1;
            pixel_colors[2] += c.data.2;
        }

        let scale: f64 = 1.0 / samples_per_pixel as f64;

        pixel_colors[0] = (scale * pixel_colors[0]).sqrt();
        pixel_colors[1] = (scale * pixel_colors[1]).sqrt();
        pixel_colors[2] = (scale * pixel_colors[2]).sqrt();

        let mut pixel: [u8; 3] = [0, 0, 0];

        pixel[0] = (256.0 * utils::clamp(pixel_colors[0], 0.0, 0.9999)) as u8;
        pixel[1] = (256.0 * utils::clamp(pixel_colors[1], 0.0, 0.9999)) as u8;
        pixel[2] = (256.0 * utils::clamp(pixel_colors[2], 0.0, 0.9999)) as u8;

        pixels[x as usize * 3] = pixel[0];
        pixels[x as usize * 3 + 1] = pixel[1];
        pixels[x as usize * 3 + 2] = pixel[2];
    }
}

fn random_scene() -> Vec<Sphere> {
    let mut world: Vec<Sphere> = vec![];

    let ground_material = Lambertian::new(Vector::new(0.5, 0.5, 0.5, VectorType::Color));
    world.push(Sphere::new(
        Vector::new(0.0, -1000.0, 0.0, VectorType::Point),
        1000.0,
        Material::Lambertian(ground_material),
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Vector::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
                VectorType::Point,
            );

            if (center - Vector::new(4.0, 0.2, 0.0, VectorType::Point)).len() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Vector::new(rng.gen(), rng.gen(), rng.gen(), VectorType::Color)
                        * Vector::new(rng.gen(), rng.gen(), rng.gen(), VectorType::Color);
                    let sphere_material = Lambertian::new(albedo);
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(sphere_material),
                    ));
                } else if choose_material < 0.95 {
                    let albedo = Vector::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        VectorType::Color,
                    );
                    let fuzz = rng.gen::<f64>();
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.push(Sphere::new(center, 0.2, Material::Metal(sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(sphere_material),
                    ));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.push(Sphere::new(
        Vector::new(0.0, 1.0, 0.0, VectorType::Point),
        1.0,
        Material::Dielectric(material1),
    ));

    let material2 = Lambertian::new(Vector::new(0.4, 0.2, 0.1, VectorType::Color));
    world.push(Sphere::new(
        Vector::new(-4.0, 1.0, 0.0, VectorType::Point),
        1.0,
        Material::Lambertian(material2),
    ));

    let material3 = Metal::new(Vector::new(0.7, 0.6, 0.6, VectorType::Color), 0.0);
    world.push(Sphere::new(
        Vector::new(4.0, 1.0, 0.0, VectorType::Point),
        1.0,
        Material::Metal(material3),
    ));

    return world;
}
