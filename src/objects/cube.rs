use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::materials::material::*;
use crate::ray_tracing::ray::Ray;
use crate::objects::group::Group;
use crate::ray_tracing::intersection::Intersection;
use crate::misc::utils::*;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Cube {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
    pub parent_inverses: Vec<Matrix4x4>,
    pub parent_material: Option<Material>,
}

impl Cube {
    //Instantiates a Cube with an identity Matrix as its transform 
    pub fn new(transform: Matrix4x4, material: Material) -> Cube {
        Cube {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
            parent_inverses: vec![],
            parent_material: None,
        }
    }

    //Instantiates a Cube with an identity Matrix as its transform 
    pub fn default() -> Cube {
        Cube {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default(),
            parent_inverses: vec![],
            parent_material: None,
        }
    }

    //Finds the minimum and maximum values of where a given ray intersects an axis
    pub fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let mut tmin;
        let mut tmax;

        if direction.abs() >= EPSILON_BUMP {
            tmin = tmin_numerator / direction;
            tmax = tmax_numerator / direction;
        }
        else {
            tmin = tmin_numerator * f32::INFINITY;
            tmax = tmax_numerator * f32::INFINITY;
        }

        if tmin > tmax {
            let temp = tmin;
            tmin = tmax;
            tmax = temp;
        }

        (tmin, tmax)
    }
}

impl Object for Cube {
    //Returns the cube material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Returns the cube matrix
    fn get_inverse(&self) -> &Matrix4x4 {
        &self.inverse
    }

    //Intersects a ray with a cube
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        let (xmin, xmax) = Cube::check_axis(transformed_ray.origin.0, transformed_ray.direction.0);
        let (ymin, ymax) = Cube::check_axis(transformed_ray.origin.1, transformed_ray.direction.1);
        let (zmin, zmax) = Cube::check_axis(transformed_ray.origin.2, transformed_ray.direction.2);

        let tmins = [xmin, ymin, zmin];
        let tmin = tmins.iter().fold(-f32::INFINITY, |a, &b| a.max(b));

        let tmaxes = [xmax, ymax, zmax];
        let tmax = tmaxes.iter().fold(f32::INFINITY, |a, &b| a.min(b));

        if tmin > tmax {
            None
        }
        else {
            Some(
                vec![
                    Intersection::new(
                        tmin,
                        Ray::position(&ray, tmin),
                        self.normal(&Ray::position(&ray, tmin)),
                        self,
                    ),
                    Intersection::new(
                        tmax,
                        Ray::position(&ray, tmax),
                        self.normal(&Ray::position(&ray, tmax)),
                        self,
                    ),
                ]
            )
        }
    }

    //Finds the normal on a given point on a cube
    fn normal(&self, world_point: &Vec4) -> Vec4 {
        let group_point = world_to_object(&self.parent_inverses, world_point);
        let transformed_point = &self.inverse * group_point;
        let coords = [transformed_point.0.abs(), transformed_point.1.abs(), transformed_point.2.abs()];
        let max_coord = coords.iter().fold(-f32::INFINITY, |a, &b| a.max(b));
        let result;
        if max_coord == transformed_point.0.abs() {
            result = Vec4(transformed_point.0, 0.0, 0.0, 0.0);
        }
        else if max_coord == transformed_point.1.abs() {
            result = Vec4(0.0, transformed_point.1, 0.0, 0.0);
        }
        else {
            result = Vec4(0.0, 0.0, transformed_point.2, 0.0);
        }
        let mut world_normal = &self.inverse.transpose() * result;
        world_normal.3 = 0.0;
        normal_to_world(&self.parent_inverses, &world_normal.normalize())
    }

    fn get_parent_inverses(&self) -> &Vec<Matrix4x4> {
        &self.parent_inverses
    }

    fn push_parent_inverse(&mut self, inverse: Matrix4x4) {
        self.parent_inverses.push(inverse);
    }

    fn get_parent_material(&self) -> &Option<Material> {
        &self.parent_material
    }

    fn set_parent_material(&mut self, material: &Material) {
        self.parent_material = Some(material.clone());
    }

    fn add_to_group(mut self, group: &mut Group) {
        self.push_parent_inverse(group.get_inverse().clone());
        self.set_parent_material(&group.material);
        group.objects.push(Box::new(self));
    }

    fn eq(&self, other: &dyn Object) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any { self }
}