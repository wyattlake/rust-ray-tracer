#[cfg(test)]
mod tests {
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::misc::axis::Axis;
    use std::rc::Rc;

    //Tests vector negation 
    #[test]
    fn negation() {
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(&Vec4::new(-1.0, -2.0, -3.0, 1.0), &vec1.negate());
    }

    //Tests vector addition
    #[test]
    fn addition() {
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let vec2 = Vec4::new(3.0, 2.0, 1.0, 0.0);
        let vec3 = vec1 + vec2;
        assert_eq!(&Vec4::new(4.0, 4.0, 4.0, 1.0), &vec3);
    }

    //Tests vector subtraction 
    #[test]
    fn subtraction() {
        let vec1 = Vec4::new(5.0, 4.0, 3.0, 1.0);
        let vec2 = Vec4::new(2.0, 1.0, 0.0, 0.0);
        let vec3 = vec1 - vec2;
        assert_eq!(&Vec4::new(3.0, 3.0, 3.0, 1.0), &vec3);
    }

    //Tests scalar multiplication 
    #[test]
    fn scalar_mult() {
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let vec2 = vec1 * 2.0;
        let vec3 = 0.5 * vec2;
        assert_eq!(&Vec4::new(1.0, 2.0, 3.0, 4.0), &vec3);
    }

    //Tests vector magnitude
    #[test]
    fn magnitude() {
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 0.0);
        let result = (14.0_f64).sqrt();
        assert_eq!(result, Vec4::magnitude(&vec1));
    }

    //Tests vector normalization
    #[test]
    fn normalize() {
        let vec1 = Vec4::new(4.0, 0.0, 0.0, 0.0);
        assert_eq!(Vec4::new(1.0, 0.0, 0.0, 0.0), vec1.normalize());
    }

    //Tests dot products 
    #[test]
    fn dot() {
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 0.0);
        let vec2 = Vec4::new(2.0, 3.0, 4.0, 0.0);
        assert_eq!(20.0, Vec4::dot(&vec1, &vec2));
    }

    //Tests cross products 
    #[test]
    fn cross_product() {
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 0.0);
        let vec3 = vec1.clone();
        let vec2 = Vec4::new(2.0, 3.0, 4.0, 0.0);
        let vec4 = vec2.clone();
        let cross1 = vec1 * vec2;
        let cross2 = vec4 * vec3;
        assert_eq!(Vec4::new(-1.0, 2.0, -1.0, 0.0), cross1);
        assert_eq!(Vec4::new(1.0, -2.0, 1.0, 0.0), cross2);
    }

    #[test]
    //Tests surface normals on the x axis
    fn surface_normal_y() {
        let s = Sphere::new();
        let vector = Vec4::normal(&s, &Vec4::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(vector, Vec4::new(0.0, 1.0, 0.0, 0.0)) 
    }

    #[test]
    //Tests surface normals on the x axis
    fn surface_normal_z() {
        let s = Sphere::new();
        let vector = Vec4::normal(&s, &Vec4::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(vector, Vec4::new(0.0, 0.0, 1.0, 0.0))
    }

    #[test]
    //Tests surface normals at an arbitrary point
    fn surface_normal() {
        let s = Sphere::new();
        let vector = Vec4::normal(&s, &Vec4::new(((3.0 as f64).sqrt())/3.0, ((3.0 as f64).sqrt())/3.0, ((3.0 as f64).sqrt())/3.0, 1.0));
        assert_eq!(vector, Vec4::new(((3.0 as f64).sqrt())/3.0, ((3.0 as f64).sqrt())/3.0, ((3.0 as f64).sqrt())/3.0, 0.0));
    }

    #[test]
    //Tests if surface normals are normalized
    fn surface_normal_normalized() {
        let s = Sphere::new();
        let vector = Vec4::normal(&s, &Vec4::new(((3.0 as f64).sqrt())/3.0, ((3.0 as f64).sqrt())/3.0, ((3.0 as f64).sqrt())/3.0, 1.0));
        assert_eq!(&vector, &vector.normalize());
    }

    #[test]
    //Tests surface normals on translated spheres
    fn surface_normal_translated() {
        let mut s = Sphere::new_raw();
        &s.transform(Matrix4x4::translation(1.0, 0.0, 0.0));
        let sphere = Rc::new(s);
        let vector = Vec4::normal(&sphere, &Vec4::new(2.0, 0.0, 0.0, 0.0));
        assert_eq!(&vector, &Vec4::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    //Tests surface normals on scaled spheres
    fn surface_normal_scaled() {
        let mut s = Sphere::new_raw();
        &s.transform(Matrix4x4::scaling(1.5, 1.0, 1.0));
        &s.transform(Matrix4x4::rotation(Axis::Z, 360.0));
        let sphere = Rc::new(s);
        let vector = Vec4::normal(&sphere, &Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(vector.round(), Vec4::new(1.0, 0.0, 0.0, 0.0));
    }

    #[test]
    //Tests reflections about a 45 degree normal
    fn vector_reflection() {
        let vec1 = Vec4::new(1.0, -1.0, 0.0, 0.0);
        let normal = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let vec2 = Vec4::reflect(&vec1, &normal);
        assert_eq!(vec2, Vec4::new(1.0, 1.0, 0.0, 0.0));
    }

    #[test]
    //Tests reflections about a 45 degree normal
    fn vector_reflection_slanted() {
        let vec1 = Vec4::new(0.0, -1.0, 0.0, 0.0);
        let normal = Vec4::new(((2.0 as f64).sqrt())/2.0, ((2.0 as f64).sqrt())/2.0, 0.0, 0.0);
        let vec2 = Vec4::reflect(&vec1, &normal);
        assert_eq!(vec2.round(), Vec4::new(1.0, 0.0, 0.0, 0.0));
    }
}

