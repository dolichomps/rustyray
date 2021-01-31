use ray::Ray;
use std::{fs::File, io::Write, path::Path};
use vec3::{Color, Point, Vec3};

mod ray;
mod vec3;

fn main() {
    let path = Path::new("image.ppm");
    let display = path.display();

    // open a file in write only mode
    let mut file = match File::create(path) {
        Err(e) => panic!("Couldn't create {}: {}", display, e),
        Ok(f) => f,
    };

    let result = get_image_string();

    match file.write_all(result.as_bytes()) {
        Err(e) => panic!("Couldn't write to {}: {}", display, e),
        Ok(_) => println!("Successfully wrote to {}", display),
    };
}

fn get_image_string() -> String {
    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    // render
    let mut result = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("Scanlines remaining: {}\r", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray {
                origin,
                direction: lower_left_corner + (u * horizontal) + (v * vertical) - origin,
            };
            let pixel_color = ray_color(r);
            pixel_color.write_color(&mut result);
        }
    }
    result
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
