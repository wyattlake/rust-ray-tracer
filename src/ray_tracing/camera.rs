use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::ray_tracing::scene::Scene;
use crate::ray_tracing::ray::Ray;
use crate::core::canvas::Canvas;

//The camera stores all the info relevant to how the scene is viewed
pub struct Camera {
    hsize: i32,
    vsize: i32,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
    transform: Matrix4x4,
}

impl Camera {
    //Creates a new Camera
    pub fn new(hsize: i32, vsize: i32, fov_degrees: f32) -> Camera {
        let fov = fov_degrees.to_radians();
        let aspect_ratio = (hsize as f32) / (vsize as f32);
        let half_view = (fov/2.0).tan() as f64;
        let mut _half_width = 0.0;
        let mut _half_height = 0.0;
        if aspect_ratio >= 1.0 {
            _half_width = half_view;
            _half_height = half_view / (aspect_ratio as f64);
        }
        else {
            _half_height = half_view;
            _half_width = half_view * (aspect_ratio as f64);
        }
        let pixel_size = (_half_width * 2.0) / (hsize as f64);
        Camera {
            hsize,
            vsize,
            half_width: _half_width,
            half_height: _half_height,
            pixel_size,
            transform: Matrix4x4::identity(),
        }
    }

    //Gets the camera pixel size
    pub fn get_pixel_size(&self) -> &f64 {
        &self.pixel_size
    }

    //Transforms the camera
    pub fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Creates a ray with a vector towards a pixel on the canvas
    pub fn ray_towards_pixel(camera: &Camera, pixel_x: i32, pixel_y: i32) -> Ray {
        //Offset from canvas edge to the center of the pixel
        let x_offset = (pixel_x as f64 + 0.5) * camera.pixel_size;
        let y_offset = (pixel_y as f64 + 0.5) * camera.pixel_size;

        //Undoes transform applied to coordinates due to camera facing towards -z
        let scene_x = camera.half_width - x_offset;
        let scene_y = camera.half_height - y_offset;

        //Finds the target pixel and origin coordinates by applying the inverse camera transformations
        let target_pixel = camera.transform.inverse().unwrap() * Vec4::new(scene_x, scene_y, -1.0, 1.0);
        let origin = camera.transform.inverse().unwrap() * Vec4::new(0.0, 0.0, 0.0, 1.0);

        //Normalizes the vector
        let direction = (target_pixel - &origin).normalize();

        Ray::new_from_vec(origin, direction)
    }

    //Renders a scene
    pub fn render(camera: &Camera, scene: &Scene, canvas: &mut Canvas) {
        for y in 0..camera.vsize {
            for x in 0..camera.hsize {
                let ray = Camera::ray_towards_pixel(camera, x, y);
                let color = Scene::compute_color(ray, scene);
                if color != None {
                    canvas.set(color.unwrap().clone(), x, y);
                }
            }
        }
    }

    //Renders a scene without lighting
    pub fn quick_render(camera: &Camera, scene: &Scene, canvas: &mut Canvas) {
        for y in 0..camera.vsize {
            for x in 0..camera.hsize {
                let ray = Camera::ray_towards_pixel(camera, x, y);
                let color = Scene::compute_color_quick(ray, scene);
                if color != None {
                    canvas.set(color.unwrap().clone(), x, y);
                }
            }
        }
    }
}