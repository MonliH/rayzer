use indicatif::ProgressBar;
use std::sync::Arc;
use std::thread::spawn;

use crate::camera::Camera;
use crate::hittables::Hittables;
use crate::utils;
use crate::vector::{Color, N};

pub fn sample(
    image_height: usize,
    image_width: usize,
    samples_per_pixel: usize,
    max_depth: usize,
    camera: &'static Camera,
    world: &'static Hittables,
    progress: Arc<ProgressBar>,
) -> Vec<u8> {
    let mut threads = Vec::with_capacity(num_cpus::get());
    for _ in 0..num_cpus::get() {
        let new_progress = Arc::clone(&progress);
        threads.push(spawn(move || {
            let mut buf = Vec::with_capacity(image_height * image_width);
            for j in (0..image_height).rev() {
                if j % 100 == 0 {
                    new_progress.inc(100);
                }
                for i in 0..image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..(samples_per_pixel / num_cpus::get()) {
                        let u = ((i as N) + utils::random_n()) / ((image_width - 1) as N);
                        let v = ((j as N) + utils::random_n()) / ((image_height - 1) as N);
                        pixel_color += camera.get_ray(u, v).color(world, max_depth);
                    }
                    buf.push(pixel_color);
                }
            }
            buf
        }));
    }

    let mut new_buf = vec![Color::new(0.0, 0.0, 0.0); image_height * image_width];
    for thread in threads.into_iter() {
        for (i, pixel) in thread.join().unwrap().into_iter().enumerate() {
            new_buf[i] += pixel;
        }
    }

    let mut image_buf = Vec::with_capacity(image_height * image_width * 3);
    for j in 0..image_height {
        for i in 0..image_width {
            write_samples(
                new_buf[j * image_width + i],
                samples_per_pixel,
                &mut image_buf,
            );
        }
    }

    image_buf
}

#[inline]
fn write_samples(c: Color, samples: usize, buf: &mut Vec<u8>) {
    let mut r = *c.x();
    let mut g = *c.y();
    let mut b = *c.z();

    let scale = 1.0 / (samples as N);
    r = N::sqrt(scale * r);
    g = N::sqrt(scale * g);
    b = N::sqrt(scale * b);

    buf.push((256.0 * utils::clamp(r, 0.0, 0.999)) as u8);
    buf.push((256.0 * utils::clamp(g, 0.0, 0.999)) as u8);
    buf.push((256.0 * utils::clamp(b, 0.0, 0.999)) as u8);
}
