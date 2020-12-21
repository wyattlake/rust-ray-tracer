#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::cube::Cube;
    use rust_ray_tracer::objects::object::*;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::materials::material::*;
    use rust_ray_tracer::ray_tracing::ray::Ray;

    #[test]
    //Tests ray intersections with each face of a cube and from within inside a cube
    fn ray_intersects_sphere() {
        let cube = Cube::new(Matrix4x4::identity(), Material::default());
        
        let ray1 = Ray::new((5.0, 0.5, 0.0), (-1.0, 0.0, 0.0));
        let intersections1 = cube.intersect(&ray1).unwrap();
        assert_eq!(intersections1[0].t, 4.0);
        assert_eq!(intersections1[1].t, 6.0);
 
        let ray2 = Ray::new((-5.0, 0.5, 0.0), (1.0, 0.0, 0.0));
        let intersections2 = cube.intersect(&ray2).unwrap();
        assert_eq!(intersections2[0].t, 4.0);
        assert_eq!(intersections2[1].t, 6.0);
 
        let ray3 = Ray::new((0.5, 5.0, 0.0), (0.0, -1.0, 0.0));
        let intersections3 = cube.intersect(&ray3).unwrap();
        assert_eq!(intersections3[0].t, 4.0);
        assert_eq!(intersections3[1].t, 6.0);
  
        let ray4 = Ray::new((0.5, -5.0, 0.0), (0.0, 1.0, 0.0));
        let intersections4 = cube.intersect(&ray4).unwrap();
        assert_eq!(intersections4[0].t, 4.0);
        assert_eq!(intersections4[1].t, 6.0);

        let ray5 = Ray::new((0.5, 0.0, 5.0), (0.0, 0.0, -1.0));
        let intersections5 = cube.intersect(&ray5).unwrap();
        assert_eq!(intersections5[0].t, 4.0);
        assert_eq!(intersections5[1].t, 6.0);
 
        let ray6 = Ray::new((0.5, 0.0, -5.0), (0.0, 0.0, 1.0));
        let intersections6 = cube.intersect(&ray6).unwrap();
        assert_eq!(intersections6[0].t, 4.0);
        assert_eq!(intersections6[1].t, 6.0);
 
        let ray6 = Ray::new((0.0, 0.5, 0.0), (0.0, 0.0, 1.0));
        let intersections6 = cube.intersect(&ray6).unwrap();
        assert_eq!(intersections6[0].t, -1.0);
        assert_eq!(intersections6[1].t, 1.0);
    }

    #[test]
    //Tests rays missing a cube
    fn ray_misses_cube() {
        let cube = Cube::default();
        let ray1 = Ray::new((2.0, 0.0, 2.0), (0.0, 0.0, -1.0));
        let intersections1 = cube.intersect(&ray1);
        assert_eq!(intersections1, None);
    }

    #[test]
    //Tests cube normals
    fn cube_normal() {
        let cube = Cube::default();
        assert_eq!(cube.normal(&Vec4(1.0, 0.5, -0.8, 1.0)), Vec4(1.0, 0.0, 0.0, 0.0));
        assert_eq!(cube.normal(&Vec4(-1.0, -0.2, 0.9, 1.0)), Vec4(-1.0, 0.0, 0.0, 0.0));
        assert_eq!(cube.normal(&Vec4(-0.4, 1.0, -0.1, 1.0)), Vec4(0.0, 1.0, 0.0, 0.0));
    }
}