use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::materials::material::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use crate::misc::utils::*;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Cone {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
    pub minimum: f32,
    pub maximum: f32,
    pub capped: bool,
}

impl Cone {
    //Instantiates a Cone with an identity Matrix as its transform 
    pub fn new(transform: Matrix4x4, material: Material, minimum: f32, maximum: f32, capped: bool) -> Cone {
        Cone {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
            minimum,
            maximum,
            capped,
        }
    }

    //Instantiates a Cone with an identity Matrix as its transform 
    pub fn default() -> Cone {
        Cone {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default(),
            minimum: -std::f32::INFINITY,
            maximum: std::f32::INFINITY,
            capped: false,
        }
    }

    fn check_cap(ray: &Ray, t: f32, radius: f32) -> bool {
        let x = ray.origin.0 + t * ray.direction.0;
        let z = ray.origin.2 + t * ray.direction.2;
        x.powi(2) + z.powi(2) <= radius
    }
}

impl Object for Cone {
    //Returns the cone material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Returns the cone matrix
    fn get_inverse(&self) -> &Matrix4x4 {
        &self.inverse
    }

    //Intersects a ray with a cone
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        let a = ((transformed_ray.direction.0.powi(2))) - (transformed_ray.direction.1.powi(2)) + ((transformed_ray.direction.2.powi(2)));
        let b = (2.0 * transformed_ray.origin.0 * transformed_ray.direction.0) - (2.0 * transformed_ray.origin.1 * transformed_ray.direction.1) + (2.0 * transformed_ray.origin.2 * transformed_ray.direction.2);
        let c = transformed_ray.origin.0.powi(2) - transformed_ray.origin.1.powi(2) + transformed_ray.origin.2.powi(2);
        if a == 0.0 && b == 0.0 {
            return None;
        }
        else {
            let mut intersections = vec![];
            if a == 0.0 {
                let t = - c / (2.0 * b);
                intersections.push(
                    Intersection::new(
                        t,
                        Ray::position(&ray, t),
                        self.normal(&Ray::position(&ray, t)),
                        self,
                    ) 
                );
            }
            else {
                let discriminant = (b.powi(2)) - (4.0 * a * c);
                if discriminant < 0.0 {
                    return None;
                }
                let mut t1 = (-b - (discriminant.sqrt())) / (2.0 * a);
                let mut t2 = (-b + (discriminant.sqrt())) / (2.0 * a);
                if t1 > t2 {
                    let temp = t1;
                    t1 = t2;
                    t2 = temp;
                }
    
                let y1 = transformed_ray.origin.1 + t1 * transformed_ray.direction.1;
                if self.minimum < y1 && y1 < self.maximum {
                    intersections.push(
                        Intersection::new(
                            t1,
                            Ray::position(&ray, t1),
                            self.normal(&Ray::position(&ray, t1)),
                            self,
                        )
                    );
                }
    
                let y2 = transformed_ray.origin.1 + t2 * transformed_ray.direction.1;
                if self.minimum < y2 && y2 < self.maximum {
                    intersections.push(
                        Intersection::new(
                            t2,
                            Ray::position(&ray, t2),
                            self.normal(&Ray::position(&ray, t2)),
                            self,
                        )
                    );
                }
            }

            if !self.capped || transformed_ray.direction.1.abs() <= EPSILON_BUMP {
                if intersections.len() > 0 {
                    return Some(intersections);
                }
                else {
                    return None;
                }
            }
            else {
                let t1 = (self.minimum - ray.origin.1) / ray.direction.1;
                if Cone::check_cap(&transformed_ray, t1, self.minimum) {
                    intersections.push(
                        Intersection::new(
                            t1,
                            Ray::position(&ray, t1),
                            self.normal(&Ray::position(&ray, t1)),
                            self,
                        )
                    );
                }
                let t2 = (self.maximum - ray.origin.1) / ray.direction.1;
                if Cone::check_cap(&transformed_ray, t2, self.maximum) {
                    intersections.push(
                        Intersection::new(
                            t2,
                            Ray::position(&ray, t2),
                            self.normal(&Ray::position(&ray, t2)),
                            self,
                        )
                    );
                }
                
                if intersections.len() > 0 {
                    Some(intersections)
                }
                else {
                    None
                }
            }
        }
    }

    //Finds the normal on a given point on a cone
    fn normal(&self, world_point: &Vec4) -> Vec4 {
        let object_point = &self.inverse * world_point;
        let distance = object_point.0.powi(2) + object_point.2.powi(2);
        let result;
        if distance < 1.0  && object_point.1 >= self.maximum - EPSILON_BUMP {
            result = Vec4(0.0, 1.0, 0.0, 0.0);
        }
        else if distance < 1.0 && object_point.1 <= self.minimum + EPSILON_BUMP {
            result = Vec4(0.0, -1.0, 0.0, 0.0);
        }
        else {
            let mut y;
            if object_point.1 > 0.0 {
                y = -(object_point.0.powi(2) + object_point.2.powi(2)).sqrt()
            }
            else {
                y = (object_point.0.powi(2) + object_point.2.powi(2)).sqrt()
            }
            result = Vec4(object_point.0, y, object_point.2, 0.0);
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