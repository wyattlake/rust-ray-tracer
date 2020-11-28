#[cfg(test)]

mod tests {
    use rust_ray_tracer::sphere::*;
    use rust_ray_tracer::intersection::*; 
    use std::rc::Rc;

    #[test]
    //Tests hit given a positive and negative Intersection
    fn pos_and_neg_intersections() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(3.0, Rc::clone(&sphere));
        let i2 = Intersection::new(-1.0, sphere.clone());
        let i3 = Intersection::hit(&[i1, i2]);
        assert_eq!(i3.unwrap(), &i1);
    }

    #[test]
    //Tests hit given negative Intersections
    fn  neg_intersections() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(-3.0, Rc::clone(&sphere));
        let i2 = Intersection::new(-1.0, Rc::clone(&sphere));
        let i3 = Intersection::new(-4.0, Rc::clone(&sphere));
        let i4 = Intersection::hit(&[i1, i2, i3]);
        assert_eq!(i4, None);
    }

    #[test]
    //Tests hit given mixed intersections
    fn  mixed_intersections() {
        let sphere = Sphere::new();
        let i1 = Intersection::new(5.0, Rc::clone(&sphere));
        let i2 = Intersection::new(7.0, Rc::clone(&sphere));
        let i3 = Intersection::new(-3.0, Rc::clone(&sphere));
        let i4 = Intersection::new(2.0, Rc::clone(&sphere));
        let i5 = Intersection::hit(&[i1, i2, i3, i4]);
        assert_eq!(i5.unwrap(), &i4);
    }
}