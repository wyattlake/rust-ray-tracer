use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use crate::ray_tracing::intersection::Intersection;
use crate::misc::utils::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::material::Material;

//Stores values for lighting computations
#[derive(Debug)]
pub struct Comp<'a> {
    pub t: f32,
    pub point: Vec4,
    pub object_inverse: Matrix4x4,
    pub e_vec: Vec4,
    pub n_vec: Vec4,
    pub r_vec: Vec4,
    pub inside: bool,
    pub over_point: Vec4,
    pub material: &'a Material,
}

impl<'a> Comp<'a> {
    //Creates a new Comp
    pub fn new(t: f32, material: &'a Material, object_inverse: Matrix4x4, point: Vec4, e_vec: Vec4, n_vec: Vec4, r_vec: Vec4, inside: bool, over_point: Vec4) -> Comp<'a> {
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
        }
    }

    //Prepares vars for shading
    pub fn compute_vars(intersection: Intersection<'a>, ray: &'a Ray) -> Comp<'a> {
        let t = intersection.get_t();
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
        Comp::new(t, intersection.material, intersection.get_inverse().clone(), point, e_vec, n_vec, r_vec, inside, over_point)
    }
}