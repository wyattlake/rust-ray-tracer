#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::group::Group;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::objects::object::*;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::misc::axis::Axis;
    use rust_ray_tracer::materials::material::*;
    use rust_ray_tracer::ray_tracing::ray::Ray;

    #[test]
    //Tests creating a group
    fn create_new_group() {
        let sphere = Sphere::default();
        let mut group = Group::new(
            Matrix4x4::identity(),
            Material::default(),
        );
        sphere.add_to_group(&mut group);
        assert_eq!(group.objects.len(), 1);
    }

    #[test]
    //Tests intersecting a ray with a group
    fn ray_group_intersections() {
        let s1 = Sphere::new(Matrix4x4::identity(), Material::default());
        let s2 = Sphere::new(Matrix4x4::translation(0.0, 0.0, -3.0), Material::default());
        let s3 = Sphere::new(Matrix4x4::translation(5.0, 0.0, 0.0), Material::default());
        
        let mut group = Group::new(
            Matrix4x4::identity(),
            Material::default(),
        );

        s1.add_to_group(&mut group);
        s2.add_to_group(&mut group);
        s3.add_to_group(&mut group);

        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let intersections = group.intersect(&ray);
        for x in &intersections {
            println!("{:?}\n", x);
        }
        assert_eq!(intersections.unwrap().len(), 4);
    }

    #[test]
    //Tests intersecting a ray with a transformed group
    fn transformed_group_intersections() {
        let sphere = Sphere::new(Matrix4x4::translation(5.0, 0.0, 0.0), Material::default());

        let mut group = Group::new(
            Matrix4x4::scaling(2.0, 2.0, 2.0),
            Material::default(),
        );

        sphere.add_to_group(&mut group);

        let ray = Ray::new((10.0, 0.0, -10.0), (0.0, 0.0, 1.0));
        let intersections = group.intersect(&ray);
        assert_eq!(intersections.unwrap().len(), 2);
    }

    #[test]
    //Tests the world space to object space conversion function
    fn world_to_object_fn () {
        let mut g1 = Group::new(Matrix4x4::rotation(Axis::Y, 90.0), Material::default());
        let mut g2 = Group::new(Matrix4x4::scaling(2.0, 2.0, 2.0), Material::default());
        let s = Sphere::new(Matrix4x4::translation(5.0, 0.0, 0.0), Material::default());
        s.add_to_group(&mut g2);
        g2.add_to_group(&mut g1);

        //list is from s's parent_inverses vector
        let list = vec![Matrix4x4::new((0.5, -0.0, 0.0, -0.0), (-0.0, 0.5, -0.0, 0.0), (0.0, -0.0, 0.5, -0.0), (-0.0, 0.0, -0.0, 1.0)), Matrix4x4::new((-0.00000004371139, -0.0, -1.0, -0.0), (-0.0, 1.0, -0.0, 0.0), (1.0, -0.0, -0.00000004371139, -0.0), (-0.0, 0.0, -0.0, 1.0))];
        assert_eq!(Vec4(0.0, 0.0, -1.0, 1.0), (Matrix4x4::translation(5.0, 0.0, 0.0).inverse().unwrap() * world_to_object(&list, &Vec4(-2.0, 0.0, -10.0, 1.0))).round());
    }

    #[test]
    //Tests the object space to world space normal conversion
    fn normal_to_world_fn () {
        let mut g1 = Group::new(Matrix4x4::rotation(Axis::Y, 90.0), Material::default());
        let mut g2 = Group::new(Matrix4x4::scaling(1.0, 2.0, 3.0), Material::default());
        let s = Sphere::new(Matrix4x4::translation(5.0, 0.0, 0.0), Material::default());
        s.add_to_group(&mut g2);
        g2.add_to_group(&mut g1);
        let point = Vec4((3.0 as f32).sqrt() / 3.0, (3.0 as f32).sqrt() / 3.0, (3.0 as f32).sqrt() / 3.0, 1.0);
        let list = vec![Matrix4x4::new((1.0, -0.0, 0.0, -0.0), (-0.0, 0.5, -0.0, 0.0), (0.0, -0.0, 0.33333334, -0.0), (-0.0, 0.0, -0.0, 1.0)), Matrix4x4::new((-0.00000004371139, -0.0, -1.0, -0.0), (-0.0, 1.0, -0.0, 0.0), (1.0, -0.0, -0.00000004371139, -0.0), (-0.0, 0.0, -0.0, 1.0))];
        assert_eq!(Vec4(0.2857, 0.4286, -0.8571, 0.0).round(), normal_to_world(&list, &point).round());
    }

    #[test]
    //Tests finding a normal on an object within a group
    fn normal_in_group () {
        let mut g1 = Group::new(Matrix4x4::rotation(Axis::Y, 90.0), Material::default());
        let mut g2 = Group::new(Matrix4x4::scaling(1.0, 2.0, 3.0), Material::default());
        let s = Sphere::new(Matrix4x4::translation(5.0, 0.0, 0.0), Material::default());
        s.add_to_group(&mut g2);
        g2.add_to_group(&mut g1);
        let list = vec![Matrix4x4::new((1.0, -0.0, 0.0, -0.0), (-0.0, 0.5, -0.0, 0.0), (0.0, -0.0, 0.33333334, -0.0), (-0.0, 0.0, -0.0, 1.0)), Matrix4x4::new((-0.00000004371139, -0.0, -1.0, -0.0), (-0.0, 1.0, -0.0, 0.0), (1.0, -0.0, -0.00000004371139, -0.0), (-0.0, 0.0, -0.0, 1.0))];
        let mut s_clone = Sphere::new(Matrix4x4::translation(5.0, 0.0, 0.0), Material::default());
        s_clone.parent_inverses = list;
        let normal = s_clone.normal(&Vec4(1.7321, 1.1547, -5.5774, 1.0));
        assert_eq!(normal.round(), Vec4(0.2857, 0.4286, -0.8571, 0.0).round());
    }
}