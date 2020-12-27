use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::ray_tracing::intersection::Intersection;
use crate::materials::material::*;
use crate::objects::group::Group;
use crate::objects::sphere::Sphere;
use crate::objects::cube::Cube;
use crate::ray_tracing::ray::Ray;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Union,
    Intersect,
    Difference,
}

//A csg has a transform trait which keeps track of its transformations
#[derive(Debug, PartialEq, Clone)]
pub struct CSG {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
    pub objects: Vec<Box<dyn Object>>,
    pub operation: Operation,
    pub parent_inverses: Vec<Matrix4x4>,
    pub parent_material: Option<Material>,
}

impl CSG {
    //Instantiates a csg with an identity Matrix as its transform
    pub fn default() -> CSG {
        let mut sphere = Sphere::default();
        sphere.set_parent_material(&Material::default());
        sphere.push_parent_inverse(Matrix4x4::identity());
        let mut cube = Cube::default();
        cube.set_parent_material(&Material::default());
        cube.push_parent_inverse(Matrix4x4::identity());
        let csg = CSG {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(),
            material: Material::default(),
            objects: vec![Box::new(sphere), Box::new(cube)],
            operation: Operation::Union,
            parent_inverses: vec![],
            parent_material: None,
        };
        csg
    }

    pub fn new(transform: Matrix4x4, material: Material, left_argument: Box<dyn Object>, right_argument: Box<dyn Object>, operation: Operation) -> CSG {
        let mut left = left_argument;
        let mut right = right_argument;
        let inverse = transform.inverse().unwrap();
        left.set_parent_material(&material);
        left.push_parent_inverse(inverse.clone());
        right.set_parent_material(&material);
        right.push_parent_inverse(inverse.clone());
        CSG {
            inverse: inverse,
            transform,
            material,
            objects: vec![left, right],
            operation,
            parent_inverses: vec![],
            parent_material: None,
        }
    }

    fn intersection_allowed(operation: &Operation, left_hit: bool, in_left: bool, in_right: bool) -> bool {
        match operation {
            Operation::Union => {
                (left_hit && !in_right) || (!left_hit && !in_left)
            }
            Operation::Intersect => {
                (left_hit && in_right) || (!left_hit && in_left)
            }
            Operation::Difference => {
                (left_hit && !in_right) || (!left_hit && in_left) 
            }
        }
    }
}

impl Object for CSG {
    //Returns the csg material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Returns the plane material
    fn get_inverse(&self) -> &Matrix4x4 {
        &self.inverse
    }

    //Intersects a ray with a csg
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        let mut valid_intersections: Vec<Intersection> = vec![];
        
        let left_intersections = self.objects[0].intersect(&transformed_ray);
        let right_intersections = self.objects[1].intersect(&transformed_ray);

        if !left_intersections.is_none() {
            let mut unwrapped_left = left_intersections.clone().unwrap();
            unwrapped_left.sort_by(|i1, i2| (i1.t).partial_cmp(&i2.t).unwrap());
            if !right_intersections.is_none() {
                let mut unwrapped_right = right_intersections.unwrap();
                unwrapped_right.sort_by(|i1, i2| (i1.t).partial_cmp(&i2.t).unwrap()); 
                let mut in_left = false;
                for i in unwrapped_left {
                    let mut in_right = false;
                    'in_right: for j in &unwrapped_right {
                        if j.t > i.t {
                            break 'in_right;
                        }
                        else {
                            in_right = !in_right;
                        }
                    }
                    if CSG::intersection_allowed(&self.operation, true, in_left, in_right) {
                        valid_intersections.push(i);
                    }
                    in_left = !in_left;
                }

                let unwrapped_left = left_intersections.unwrap();

                let mut in_right = false;
                for i in unwrapped_right {
                    let mut in_left = false;
                    'in_left: for j in &unwrapped_left {
                        if j.t > i.t {
                            break 'in_left;
                        }
                        else {
                            in_left = !in_left;
                        }
                    }
                    if CSG::intersection_allowed(&self.operation, false, in_left, in_right) {
                        valid_intersections.push(i);
                    }
                    in_right = !in_right;
                }
            }
            else {
                let mut in_left = false;
                for i in unwrapped_left {
                    if CSG::intersection_allowed(&self.operation, true, in_left, false) {
                        valid_intersections.push(i);
                    }
                    in_left = !in_left;
                }
            }
        }
        else {
            if !right_intersections.is_none() {
                let mut unwrapped_right = right_intersections.unwrap();
                unwrapped_right.sort_by(|i1, i2| (i1.t).partial_cmp(&i2.t).unwrap()); 
                let mut in_right = false;
                for i in unwrapped_right {
                    if CSG::intersection_allowed(&self.operation, false, false, in_right) {
                        valid_intersections.push(i);
                    }
                    in_right = !in_right;
                }
            }
        }

        let mut final_intersections = vec![];

        for intersection in &mut valid_intersections {
            let new_intersection;
            if intersection.u == None {
                new_intersection = Intersection::new(
                    intersection.t,
                    Ray::position(&transformed_ray, intersection.t),
                    (intersection.object).normal(&Ray::position(&ray, intersection.t), None, None),
                    intersection.object,
                );
            }
            else {
                new_intersection = Intersection::new(
                    intersection.t,
                    Ray::position(&transformed_ray, intersection.t),
                    (intersection.object).normal(&Ray::position(&ray, intersection.t), intersection.u, intersection.v),
                    intersection.object,
                );
            }
            final_intersections.push(new_intersection);
        }

        if valid_intersections.len() > 0 {
            Some(final_intersections)
        }
        else {
            None
        }
    }

    //Finds the normal of a given point on a csg
    fn normal(&self, _world_point: &Vec4, _u: Option<f32>, _v: Option<f32>) -> Vec4 {
        panic!("Cannot find the normal of a CSG");
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
