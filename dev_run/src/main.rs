use raytrace::ray::Ray;
use raytrace::camera::Camera;

fn main() {
    let camera = Camera::new(500, 500);
    let image = camera.render();
    let _ = image.save("output/render.jpg");
}
