#[cfg(test)]
mod tests {
    use rust_ray_tracer::vector::*;

    //Tests vector negation 
    #[test]
    fn negation() {
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(&Vec4::new(-1.0, -2.0, -3.0, -1.0), &vec1.negate());
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
}

