#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::plane::Plane;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::ray_tracing::ray::Ray;
    use rust_ray_tracer::objects::general::{Object, ObjectMethods};

    //Tests plane normals
    #[test]
    fn plane_normal() {
        let p = Plane::new();
        let n1 = Object::normal(&p, &Vec4::new(0.0, 0.0, 0.0, 1.0));
        let n2 = Object::normal(&p, &Vec4::new(10.0, 0.0, -10.0, 1.0));
        let n3 = Object::normal(&p, &Vec4::new(-5.0, 0.0, 150.0, 1.0));
        assert_eq!(n1, Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(n2, Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(n3, Vec4::new(0.0, 1.0, 0.0, 0.0));
    }

    #[test]
    //Tests intersecting a plane with a ray parallel to the plane
    fn parallel_intersection() {
        let p = Plane::new();
        let ray = Ray::new((0.0, 10.0, 0.0), (0.0, 0.0, 1.0));
        let intersections = Plane::intersect(p, &ray);
        assert_eq!(intersections, None);
    }

    #[test]
    //Tests intersecting a plane with a coplanar ray
    fn coplanar_intersection() {
        let p = Plane::new();
        let ray = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let intersections = Plane::intersect(p, &ray);
        assert_eq!(intersections, None);
    }

    #[test]
    //Tests intersecting a plane with a ray from above
    fn above_plane_intersection() {
        let p = Plane::new();
        let ray = Ray::new((0.0, 1.0, 0.0), (0.0, -1.0, 0.0));
        let intersections = Plane::intersect(p, &ray).unwrap();
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].get_t(), 1.0);
    }

    #[test]
    //Tests intersecting a plane with a ray from below
    fn below_plane_intersection() {
        let p = Plane::new();
        let ray = Ray::new((0.0, -1.0, 0.0), (0.0, 1.0, 0.0));
        let intersections = Plane::intersect(p, &ray).unwrap();
        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].get_t(), 1.0);
    }
}