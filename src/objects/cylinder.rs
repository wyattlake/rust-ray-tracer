use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::materials::material::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use crate::objects::group::Group;
use crate::misc::utils::*;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Cylinder {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
    pub minimum: f32,
    pub maximum: f32,
    pub capped: bool,
    pub parent_inverses: Vec<Matrix4x4>,
    pub parent_material: Option<Material>,
}

impl Cylinder {
    //Instantiates a Cylinder with an identity Matrix as its transform 
    pub fn new(transform: Matrix4x4, material: Material, minimum: f32, maximum: f32, capped: bool) -> Cylinder {
        Cylinder {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
            minimum,
            maximum,
            capped,
            parent_inverses: vec![],
            parent_material: None,
        }
    }

    //Instantiates a Cylinder with an identity Matrix as its transform 
    pub fn default() -> Cylinder {
        Cylinder {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default(),
            minimum: -std::f32::INFINITY,
            maximum: std::f32::INFINITY,
            capped: false,
            parent_inverses: vec![],
            parent_material: None,
        }
    }

    fn check_cap(ray: &Ray, t: f32) -> bool {
        let x = ray.origin.0 + t * ray.direction.0;
        let z = ray.origin.2 + t * ray.direction.2;
        x.powi(2) + z.powi(2) <= 1.0
    }
}

impl Object for Cylinder {
    //Returns the cylinder material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Returns the cylinder matrix
    fn get_inverse(&self) -> &Matrix4x4 {
        &self.inverse
    }

    //Intersects a ray with a cylinder
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        let a = (transformed_ray.direction.0.powi(2)) + (transformed_ray.direction.2.powi(2));
        let b = (2.0 * transformed_ray.origin.0 * transformed_ray.direction.0) + (2.0 * transformed_ray.origin.2 * transformed_ray.direction.2);
        let c = transformed_ray.origin.0.powi(2) + transformed_ray.origin.2.powi(2) - 1.0;
        let discriminant = (b.powi(2)) - (4.0 * a * c);
        if discriminant < 0.0 {
            return None;
        }
        else {
            let mut t1 = (-b - (discriminant.sqrt())) / (2.0 * a);
            let mut t2 = (-b + (discriminant.sqrt())) / (2.0 * a);
            if t1 > t2 {
                let temp = t1;
                t1 = t2;
                t2 = temp;
            }
            let mut intersections = vec![];
            
            //Modifies intersection slightly to prevent acne
            let bump = 0.00005;

            let y1 = transformed_ray.origin.1 + (t1 * transformed_ray.direction.1);
            if self.minimum < y1 && y1 < self.maximum {
                intersections.push(
                    Intersection::new(
                        t1 - bump,
                        Ray::position(&ray, t1 - bump),
                        self.normal(&Ray::position(&ray, t1 - bump), None, None),
                        self,
                    )
                );
            }

            let y2 = transformed_ray.origin.1 + (t2 * transformed_ray.direction.1);
            if self.minimum < y2 && y2 < self.maximum {
                intersections.push(
                    Intersection::new(
                        t2 - bump,
                        Ray::position(&ray, t2 - bump),
                        self.normal(&Ray::position(&ray, t2 - bump), None, None),
                        self,
                    )
                );
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
                let t1 = (self.minimum - transformed_ray.origin.1) / transformed_ray.direction.1;
                if Cylinder::check_cap(&transformed_ray, t1) {
                    intersections.push(
                        Intersection::new(
                            t1,
                            Ray::position(&ray, t1),
                            self.normal(&Ray::position(&ray, t1), None, None),
                            self,
                        )
                    );
                }
                let t2 = (self.maximum - transformed_ray.origin.1) / transformed_ray.direction.1;
                if Cylinder::check_cap(&transformed_ray, t2) {
                    intersections.push(
                        Intersection::new(
                            t2,
                            Ray::position(&ray, t2),
                            self.normal(&Ray::position(&ray, t2), None, None),
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

    //Finds the normal on a given point on a cylinder
    fn normal(&self, world_point: &Vec4, _u: Option<f32>, _v: Option<f32>) -> Vec4 {
        let group_point = world_to_object(&self.parent_inverses, world_point);
        let object_point = &self.inverse * group_point;
        let distance = object_point.0.powi(2) + object_point.2.powi(2);
        let result;
        if distance < 1.0  && world_point.1 >= self.maximum - EPSILON_BUMP {
            result = Vec4(0.0, 1.0, 0.0, 0.0);
        }
        else if distance < 1.0  && world_point.1 <= self.minimum + EPSILON_BUMP {
            result = Vec4(0.0, -1.0, 0.0, 0.0);
        }
        else {
            result = Vec4(object_point.0, 0.0, object_point.2, 0.0);
        }
        let mut world_normal = &self.inverse.transpose() * result;
        world_normal.3 = 0.0;
        let world_normal = world_normal.normalize();
        normal_to_world(&self.parent_inverses, &world_normal)
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