use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::misc::utils::*;
use crate::objects::object::*;
use crate::ray_tracing::intersection::Intersection;
use crate::materials::material::Material;
use crate::ray_tracing::ray::Ray;

//Stores values for lighting computations
#[derive(Debug)]
pub struct Comp {
    pub t: f32,
    pub point: Vec4,
    pub object_inverse: Matrix4x4, //Object's inverse matrix
    pub e_vec: Vec4,               //Eye vector
    pub n_vec: Vec4,               //Normal vector
    pub r_vec: Vec4,               //Reflection vector
    pub inside: bool,
    pub over_point: Vec4, //Position of intersection adjusted along normal with EPSILON_BUMP
    pub under_point: Vec4, //Position of intersectino pushed slightly below the object surface via EPSILON_BUMP
    pub material: Material,
    pub n1: f32, //Refraction index of the object the ray is passing form
    pub n2: f32, //Refraction index of the object the ray is passing to
    pub parent_inverses: Vec<Matrix4x4>
}

impl Comp {
    //Creates a new Comp
    pub fn new(
        t: f32,
        material: Material,
        object_inverse: Matrix4x4,
        point: Vec4,
        e_vec: Vec4,
        n_vec: Vec4,
        r_vec: Vec4,
        inside: bool,
        over_point: Vec4,
        under_point: Vec4,
        n1: f32,
        n2: f32,
        parent_inverses: Vec<Matrix4x4> 
    ) -> Comp {
        Comp {
            t,
            material,
            object_inverse,
            point,
            e_vec,
            n_vec,
            r_vec,
            inside,
            over_point,
            under_point,
            n1,
            n2,
            parent_inverses,
        }
    }

    //Prepares vars for shading
    pub fn compute_vars(
        intersection: Intersection,
        ray: &Ray,
        intersection_list: &Vec<Intersection>,
    ) -> Comp {
        let t = intersection.t;
        let point = Ray::position(ray, t);
        let mut n_vec = intersection.normal.clone();
        let e_vec = ray.direction.negate();
        let r_vec = Vec4::reflect(&ray.direction, &n_vec);
        let inside;
        if Vec4::dot(&n_vec, &e_vec) < 0.0 {
            inside = true;
            n_vec = n_vec.negate();
        }
        else {
            inside = false;
        }
        let offset = &n_vec * EPSILON_BUMP;
        let over_point = &point + &offset;
        let under_point = &point - &offset;

        
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        let object_material;
        if intersection.object.get_parent_material() != &None {
            object_material = intersection.object.get_parent_material().as_ref().unwrap().clone()
        }
        else {
            object_material = intersection.object.get_material().clone();
        }

        let mut containers: Vec<&dyn Object> = vec![];
        for i in intersection_list {
            if i == &intersection {
                if !containers.is_empty() {
                    n1 = containers.last().unwrap().get_material().refractive_index;
                }
            }

            let intersection_object = i.object.clone();

            let before_len = containers.len();

            containers.retain(|object| !(&intersection_object == object));

            if containers.len() == before_len {
                containers.push(intersection_object);
            }

            if i == &intersection {
                if !containers.is_empty() {
                    n2 = containers.last().unwrap().get_material().refractive_index;
                }
            }
        }
        Comp::new(
            t,
            object_material,
            intersection.object.get_inverse().clone(),
            point,
            e_vec,
            n_vec,
            r_vec,
            inside,
            over_point,
            under_point,
            n1,
            n2,
            intersection.object.get_parent_inverses().clone(),
        )
    }
}
