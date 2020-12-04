use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::ray_tracing::scene::Scene;
use crate::ray_tracing::ray::Ray;
use crate::core::canvas::Canvas;
use crate::core::color::Color;

//The camera stores all the info relevant to how the scene is viewed
pub struct Camera {
    pub hsize: i32,
    pub vsize: i32,
    pub pixel_size: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub transform: Matrix4x4,
}

impl Camera {
    //Creates a new Camera
    pub fn new(hsize: usize, vsize: usize, fov_degrees: f32) -> Camera {
        let fov = fov_degrees.to_radians();
        let aspect_ratio = (hsize as f32) / (vsize as f32);
        let half_view = (fov/2.0).tan() as f32;
        let mut _half_width = 0.0;
        let mut _half_height = 0.0;
        if aspect_ratio >= 1.0 {
            _half_width = half_view;
            _half_height = half_view / (aspect_ratio as f32);
        }
        else {
            _half_height = half_view;
            _half_width = half_view * (aspect_ratio as f32);
        }
        let pixel_size = (_half_width * 2.0) / (hsize as f32);
        Camera {
            hsize: (hsize as i32),
            vsize: (vsize as i32),
            half_width: _half_width,
            half_height: _half_height,
            pixel_size,
            transform: Matrix4x4::identity(),
        }
    }

    //Transforms the camera
    pub fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = &self.transform * matrix;
    }

    //Creates a ray with a vector towards a pixel on the canvas
    pub fn ray_towards_pixel(camera: &Camera, pixel_x: i32, pixel_y: i32) -> Ray {
        //Offset from canvas edge to the center of the pixel
        let x_offset = (pixel_x as f32 + 0.5) * camera.pixel_size;
        let y_offset = (pixel_y as f32 + 0.5) * camera.pixel_size;

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

    //Creates a ray with a vector towards a pixel on the canvas
    pub fn ray_towards_pixel_raw(camera: &Camera, pixel_x: i32, pixel_y: i32, offset_x: f32, offset_y: f32) -> Ray {
        //Offset from canvas edge to the center of the pixel
        let x_offset = (pixel_x as f32 + offset_x) * camera.pixel_size;
        let y_offset = (pixel_y as f32 + offset_y) * camera.pixel_size;

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
    pub fn render(camera: &Camera, scene: Scene, canvas: &mut Canvas) {
        let mut counter = 0;
        let mut percent = 0;
        let pixels = &camera.hsize * &camera.vsize;
        let percentage_update = pixels as f32 / 10.0;
        for y in 0..camera.vsize {
            for x in 0..camera.hsize {
                let ray = Camera::ray_towards_pixel(camera, x, y);
                let color = Scene::compute_color(ray, &scene, 5);
                if color != None {
                    canvas.set(color.unwrap().clone(), x, y);
                }
                if counter as f32 > percentage_update {
                    percent += 10;
                    println!("Render is {}% complete", percent);
                    counter = 0;
                }
                else {
                    counter += 1;
                }
            }
        }
    }

    //Renders a scene
    pub fn render_supersampled(camera: &Camera, scene: Scene, canvas: &mut Canvas) {
        let mut counter = 0;
        let mut percent = 0;
        let pixels = &camera.hsize * &camera.vsize;
        let percentage_update = pixels as f32 / 10.0;
        for y in 0..camera.vsize {
            for x in 0..camera.hsize {
                let ray1 = Camera::ray_towards_pixel(camera, x, y);
                let ray2 = Camera::ray_towards_pixel_raw(camera, x, y, 0.0, 0.0);
                let ray3 = Camera::ray_towards_pixel_raw(camera, x, y, 0.0, 1.0);
                let ray4 = Camera::ray_towards_pixel_raw(camera, x, y, 1.0, 0.0);
                let ray5 = Camera::ray_towards_pixel_raw(camera, x, y, 1.0, 1.0);
                let mut list = vec![Scene::compute_color(ray1, &scene, 5), Scene::compute_color(ray2, &scene, 5), Scene::compute_color(ray3, &scene, 5), Scene::compute_color(ray4, &scene, 5), Scene::compute_color(ray5, &scene, 5)];
                let mut result = Color::new(0.0, 0.0, 0.0);
                for _ in 0..5 {
                    if list[0] != None {
                        result = result + list.remove(0).unwrap();
                    }
                    else {
                        list.remove(0); 
                    }
                }
                result = result * 0.2;
                canvas.set(result, x, y);
                if counter as f32 > percentage_update {
                    percent += 10;
                    println!("Render is {}% complete", percent);
                    counter = 0;
                }
                else {
                    counter += 1;
                }
            }
        }
    }

    //Renders a scene without lighting
    pub fn quick_render(camera: &Camera, scene: Scene, canvas: &mut Canvas) {
        let mut counter = 0;
        let mut percent = 0;
        let pixels = &camera.hsize * &camera.vsize;
        let percentage_update = pixels as f32 / 10.0;
        for y in 0..camera.vsize {
            for x in 0..camera.hsize {
                let ray = Camera::ray_towards_pixel(camera, x, y);
                let color = Scene::compute_color_quick(ray, &scene);
                if color != None {
                    canvas.set(color.unwrap().clone(), x, y);
                    if counter as f32 > percentage_update {
                        percent += 10;
                        println!("Render is {}% complete", percent);
                        counter = 0;
                    }
                    else {
                        counter += 1;
                    }
                }
            }
        }
    }
}
