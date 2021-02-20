#[macro_use]
extern crate lazy_static;

mod camera;
mod hittables;
mod materials;
mod ray;
mod render;
mod utils;
mod vector;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::Arc;

use indicatif::ProgressBar;

use camera::Camera;
use hittables::{Hittables, MovingSphere, Sphere};
use materials::{Glass, Lambert, Material, Metal};
use utils::{random_n, random_range};
use vector::{Color, Point3D, Vector3D, N};

fn random_scene() -> Hittables {
    let mut world = Hittables::new();

    let groud_material = Arc::new(Lambert::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3D::new(0.0, -1000.0, 0.0),
        1000.0,
        groud_material,
    )));

    let amount = 11;

    for a in -amount..amount {
        for b in -amount..amount {
            let choose_material = random_n();
            let center = Point3D::new(0.9 * random_n() + a as N, 0.2, 0.9 * random_n() + b as N);

            if (center - Point3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material + Send + Sync> = if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let center2 = center + Vector3D::new(0.0, random_range(0.0, 0.5), 0.0);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambert::new(albedo)),
                    )));
                    continue;
                } else if choose_material < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let roughness = random_range(0.0, 0.5);
                    Arc::new(Metal::new(albedo, roughness))
                } else {
                    Arc::new(Glass::new(1.5))
                };

                world.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Arc::new(Glass::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3D::new(0.0, 1.0, 0.1),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambert::new(Color::new(0.0, 0.5, 1.0)));
    world.add(Box::new(Sphere::new(
        Point3D::new(-4.0, 1.0, 0.1),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3D::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

const ASPECT_RATIO: N = 16.0 / 9.0;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

lazy_static! {
    static ref WORLD: Hittables = random_scene();
    static ref CAMERA: Camera = {
        // Camera setup
        let lookat = Point3D::new(0.0, 0.0, 0.0);
        let lookfrom = Point3D::new(13.0, 2.0, 3.0);
        let vup = Vector3D::new(0.0, 1.0, 0.0);
        let focus_distance = 10.0;
        let aperature = 0.1;

        Camera::new(
            lookfrom,
            lookat,
            vup,
            20.0,
            ASPECT_RATIO,
            aperature,
            focus_distance,
            Some(0.0),
            Some(1.0)
        )
    };
}

fn main() {
    // Image dimensions
    let image_width: usize = 400;
    let image_height = ((image_width as N) / ASPECT_RATIO) as usize;

    // Set up PNG encoder
    let path = Path::new(r"./output.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image_width as u32, image_height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let progress = Arc::new(ProgressBar::new((image_height * num_cpus::get()) as u64));

    let buf = render::sample(
        image_height,
        image_width,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        &CAMERA,
        &WORLD,
        progress,
    );

    // Write buffer
    writer.write_image_data(&buf).unwrap();
}
