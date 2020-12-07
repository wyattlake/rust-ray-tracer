use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::misc::utils::*;
use crate::objects::object::*;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;

//Stores values for lighting computations
#[derive(Debug)]
pub struct Comp<'a> {
    pub t: f32,
    pub point: Vec4,
    pub object_inverse: Matrix4x4, //Object's inverse matrix
    pub e_vec: Vec4,               //Eye vector
    pub n_vec: Vec4,               //Normal vector
    pub r_vec: Vec4,               //Reflection vector
    pub inside: bool,
    pub over_point: Vec4, //Position of intersection adjusted along normal with EPSILON_BUMP
    pub material: &'a Material,
    pub n1: f32, //Refraction index of the object the ray is passing form
    pub n2: f32, //Refraction index of the object the ray is passing to
}

impl<'a> Comp<'a> {
    //Creates a new Comp
    pub fn new(
        t: f32,
        material: &'a Material,
        object_inverse: Matrix4x4,
        point: Vec4,
        e_vec: Vec4,
        n_vec: Vec4,
        r_vec: Vec4,
        inside: bool,
        over_point: Vec4,
        n1: f32,
        n2: f32,
    ) -> Comp<'a> {
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
            n1,
            n2,
        }
    }

    //Prepares vars for shading
    pub fn compute_vars(
        intersection: Intersection<'a>,
        ray: &'a Ray,
        intersection_list: Vec<Intersection>,
    ) -> Comp<'a> {
        let t = intersection.t;
        let point = Ray::position(ray, t);
        let mut n_vec = intersection.normal.clone();
        let e_vec = ray.get_direction().negate();
        let r_vec = Vec4::reflect(ray.get_direction(), &n_vec);
        let mut inside = false;
        let over_point = &point + (&n_vec.normalize() * EPSILON_BUMP);
        if Vec4::dot(&n_vec, &e_vec) < 0.0 {
            inside = true;
            n_vec = n_vec.negate();
        }
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        let mut containers: Vec<(&Matrix4x4, &Material, &ObjectEnum)> = vec![];
        for i in intersection_list {
            if i == intersection {
                if !containers.is_empty() {
                    n1 = containers.last().unwrap().1.refractive_index;
                }
            }

            let mut has_object = false;
            let mut index = 0;
            let mut remove_index = 0;
            for x in &containers {
                if i.object_inverse == x.0 && i.material == x.1 && i.object_type == x.2 {
                    has_object = true;
                    remove_index = index;
                }
                index += 1;
            }

            if has_object {
                containers.remove(remove_index);
            } else {
                containers.push((&i.object_inverse, &i.material, &i.object_type));
            }

            if i == intersection {
                if !containers.is_empty() {
                    n2 = containers.last().unwrap().1.refractive_index;
                }
            }
        }
        Comp::new(
            t,
            intersection.material,
            intersection.object_inverse.clone(),
            point,
            e_vec,
            n_vec,
            r_vec,
            inside,
            over_point,
            n1,
            n2,
        )
    }
}
