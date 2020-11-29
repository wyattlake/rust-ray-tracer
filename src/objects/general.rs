use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::sphere::Sphere;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use std::rc::Rc;
use std::borrow::Borrow;

//Creates an object enum
#[derive(Debug, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}

impl Object {
    pub fn get_transform(&self) -> &Matrix4x4 {
        match &*self.borrow() {
            Object::Sphere(sphere) => {
                sphere.get_transform() 
            }
        }
    }

    pub fn intersect(object: Rc<Object>, ray: &Ray) -> Option<Vec<Intersection>> {
        match &*object.borrow() {
            Object::Sphere(_) => {
                Sphere::intersect(object, ray)
            }
        }
    }

    pub fn normal(object: &Rc<Object>, point: &Vec4) -> Vec4 {
        match &*object.borrow() {
            Object::Sphere(_) => {
                Sphere::normal(object, point)
            }
        }
    }

    pub fn get_mut_material(&mut self) -> &mut Material {
        match self {
            Object::Sphere(sphere) => {
                sphere.get_mut_material() 
            }
        }
    }

    pub fn transform(&mut self, matrix: Matrix4x4) {
        match self {
            Object::Sphere(sphere) => {
                sphere.transform(matrix);
            }
        }
    }
    pub fn get_material(&self) -> &Material {
        match self {
            Object::Sphere(sphere) => {
                sphere.get_material()
            }
        } 
    }
}

//Trait which holds necessary methods for an object
pub trait ObjectMethods {
    //Creates a new instance of an object
    fn new_raw() -> Object;

    //Creates a new object wrapped in a reference counter
    fn new() -> Rc<Object>;

    //Transforms an object
    fn transform(&mut self, matrix: Matrix4x4);

    //Returns the object transform
    fn get_transform(&self) -> &Matrix4x4;

    //Returns the object material
    fn get_material(&self) -> &Material;

    //Sets the material of an object
    fn set_material(&mut self, material: Material);

    //Gets a mutable reference to the material
    fn get_mut_material(&mut self) -> &mut Material;
}