#[cfg(test)]

mod tests {
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::misc::axis::Axis;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::objects::general::{ObjectMethods, Object};
    use std::rc::Rc;

    #[test]
    //Tests surface normals on the x axis
    fn surface_normal_y() {
        let s = Sphere::new();
        let vector = Object::normal(&s, &Vec4::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(vector, Vec4::new(0.0, 1.0, 0.0, 0.0)) 
    }

    #[test]
    //Tests surface normals on the x axis
    fn surface_normal_z() {
        let s = Sphere::new();
        let vector = Object::normal(&s, &Vec4::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(vector, Vec4::new(0.0, 0.0, 1.0, 0.0))
    }

    #[test]
    //Tests surface normals at an arbitrary point
    fn surface_normal() {
        let s = Sphere::new();
        let vector = Object::normal(&s, &Vec4::new(((3.0 as f32).sqrt())/3.0, ((3.0 as f32).sqrt())/3.0, ((3.0 as f32).sqrt())/3.0, 1.0));
        assert_eq!(vector.round(), Vec4::new(((3.0 as f32).sqrt())/3.0, ((3.0 as f32).sqrt())/3.0, ((3.0 as f32).sqrt())/3.0, 0.0).round());
    }

    #[test]
    //Tests if surface normals are normalized
    fn surface_normal_normalized() {
        let s = Sphere::new();
        let vector = Object::normal(&s, &Vec4::new(((3.0 as f32).sqrt())/3.0, ((3.0 as f32).sqrt())/3.0, ((3.0 as f32).sqrt())/3.0, 1.0));
        assert_eq!(&vector.round(), &vector.normalize().round());
    }

    #[test]
    //Tests surface normals on translated spheres
    fn surface_normal_translated() {
        let mut s = Sphere::new_raw();
        &s.transform(Matrix4x4::translation(1.0, 0.0, 0.0));
        let sphere = Rc::new(s);
        let vector = Object::normal(&sphere, &Vec4::new(2.0, 0.0, 0.0, 0.0));
        assert_eq!(&vector, &Vec4::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    //Tests surface normals on scaled spheres
    fn surface_normal_scaled() {
        let mut s = Sphere::new_raw();
        &s.transform(Matrix4x4::scaling(1.5, 1.0, 1.0));
        &s.transform(Matrix4x4::rotation(Axis::Z, 360.0));
        let sphere = Rc::new(s);
        let vector = Object::normal(&sphere, &Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(vector.round(), Vec4::new(1.0, 0.0, 0.0, 0.0));
    }
}