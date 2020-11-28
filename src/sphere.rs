use crate::matrix::Matrix4x4;
use crate::material::Material;
use std::rc::Rc;

//A sphere has a transform trait which keeps track of its transformations
#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix4x4,
    material: Material,
}

impl Sphere {
    //Instantiates a Sphere with a Rc and an identity Matrix as its transform
    pub fn new() -> Rc<Sphere> {
        Rc::new(
            Sphere {
                transform: Matrix4x4::identity(),
                material: Material::default()
            }
        )
    }

    //Instantiates a sphere with an identity Matrix as its transform 
    pub fn new_raw() -> Sphere {
        Sphere {
            transform: Matrix4x4::identity(),
            material: Material::default()
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

    //Returns the sphere material
    pub fn get_material(&self) -> &Material {
        &self.material
    }

    //Sets the material of the sphere
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    //Gets a mutable reference to the material
    pub fn mut_material_ref(&mut self) -> &mut Material {
        &mut self.material
    }

    //Gets a mutable reference to the material
    pub fn material_ref(&self) -> &Material {
        &self.material
    }
}

impl Clone for Sphere {
    //Clones a given sphere
    fn clone(&self) -> Sphere {
        Sphere {
            transform: self.transform.clone(),
            material: self.material.clone(),
        }
    }
}