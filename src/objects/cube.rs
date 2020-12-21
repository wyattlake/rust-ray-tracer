use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::materials::material::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use crate::misc::utils::*;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Cube {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
}

impl Cube {
    //Instantiates a Cube with an identity Matrix as its transform 
    pub fn new(transform: Matrix4x4, material: Material) -> Cube {
        Cube {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
        }
    }

    //Instantiates a Cube with an identity Matrix as its transform 
    pub fn default() -> Cube {
        Cube {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default()
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
                        Ray::position(&transformed_ray, tmin),
                        self.normal(&Ray::position(&transformed_ray, tmin)),
                        self,
                    ),
                    Intersection::new(
                        tmax,
                        Ray::position(&transformed_ray, tmax),
                        self.normal(&Ray::position(&transformed_ray, tmax)),
                        self,
                    ),
                ]
            )
        }
    }

    //Finds the normal on a given point on a cube
    fn normal(&self, world_point: &Vec4) -> Vec4 {
        let coords = [world_point.0.abs(), world_point.1.abs(), world_point.2.abs()];
        let max_coord = coords.iter().fold(-f32::INFINITY, |a, &b| a.max(b));
        let result;
        if max_coord == world_point.0.abs() {
            result = Vec4(world_point.0, 0.0, 0.0, 0.0);
        }
        else if max_coord == world_point.1.abs() {
            result = Vec4(0.0, world_point.1, 0.0, 0.0);
        }
        else {
            result = Vec4(0.0, 0.0, world_point.2, 0.0);
        }
        let mut world_normal = &self.inverse.transpose() * result;
        world_normal.3 = 0.0;
        world_normal.normalize()
    }

    fn eq(&self, other: &dyn Object) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any { self }
}