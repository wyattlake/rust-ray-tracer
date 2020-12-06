#[cfg(test)]

mod tests {
    use rust_ray_tracer::ray_tracing::camera::Camera;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::misc::axis::Axis;
    use rust_ray_tracer::ray_tracing::scene::Scene;
    use rust_ray_tracer::core::canvas::Canvas;
    use rust_ray_tracer::core::color::Color;
    use rust_ray_tracer::core::sequence::Sequence;

    //Tests the pixel size of a new camera
    #[test]
    fn pixel_size() {
        let camera = Camera::new(200, 125, 90.0);
        assert_eq!(camera.pixel_size, 0.01)
    }

    //Tests creating a ray through the center of the camera's view
    #[test]
    fn center_ray() {
        let camera = Camera::new(201, 101, 90.0);
        let ray = Camera::ray_towards_pixel(&camera, 100, 50);
        assert_eq!(ray.get_origin(), &Vec4::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(ray.get_direction().round(), Vec4::new(0.0, 0.0, -1.0, 0.0));
    }

    //Tests creating a ray towards the corner of the camera's view
    #[test]
    fn corner_ray() {
        let camera = Camera::new(201, 101, 90.0);
        let ray = Camera::ray_towards_pixel(&camera, 0, 0);
        assert_eq!(ray.get_origin(), &Vec4::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(ray.get_direction().round(), Vec4::new(0.66519, 0.33259, -0.66851, 0.0).round());
    }

    //Tests creating a transformed ray
    #[test]
    fn transformed_ray() {
        let mut camera = Camera::new(201, 101, 90.0);
        camera.transform(Matrix4x4::rotation(Axis::Y, 45.0) * Matrix4x4::translation(0.0, -2.0, 5.0));
        let ray = Camera::ray_towards_pixel(&camera, 100, 50);
        assert_eq!(ray.get_origin(), &Vec4::new(0.0, 2.0, -5.0000005, 1.0));
        assert_eq!(ray.get_direction().round(), Vec4::new((2 as f32).sqrt() / 2.0, 0.0, - (2 as f32).sqrt() / 2.0, 0.0).round());
    }

    //Tests rendering a scene
    #[test]
    fn render_scene() {
        let scene = Scene::default();
        let mut camera = Camera::new(11, 11, 90.0);
        let start_pos = Vec4::new(0.0, 0.0, -5.0, 1.0);
        let end_pos = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);
        camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));
        let mut canvas = Canvas::new(11, 11);
        Camera::render(&camera, &scene, &mut canvas, &mut Sequence::new(vec![0.0]));
        assert_eq!(canvas.get(5, 5).unwrap().round(), Color::new(0.38062, 0.47583, 0.2855).round());
    }
}