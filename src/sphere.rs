use crate::matrix::Matrix4x4;
use std::rc::Rc;

//A sphere has a transform trait which keeps track of its transformations
#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix4x4,
}

impl Sphere {
    //Instantiates a Sphere with a Rc and an identity Matrix as its transform
    pub fn new() -> Rc<Sphere> {
        Rc::new(
            Sphere {
                transform: Matrix4x4::identity(),
            }
        )
    }

    //Instantiates a sphere with an identity Matrix as its transform 
    pub fn new_raw() -> Sphere {
        Sphere {
            transform: Matrix4x4::identity(),
        } 
    }

    //Applies a transformation to a sphere
    pub fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix * self.transform.clone();
    }

    //Returns the sphere transform
    pub fn get_transform(&self) -> &Matrix4x4 {
        &self.transform
    }
}

impl Clone for Sphere {
    //Clones a given sphere
    fn clone(&self) -> Sphere {
        Sphere {
            transform: self.transform.clone(),
        }
    }
}