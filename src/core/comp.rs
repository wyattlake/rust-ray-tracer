use crate::objects::sphere::Sphere;
use crate::core::vector::Vec4;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::ray::Ray;
use std::rc::Rc;

//Stores values for lighting computations
pub struct Comp {
    pub t: f64,
    pub object: Rc<Sphere>,
    pub point: Vec4,
    pub e_vec: Vec4,
    pub n_vec: Vec4,
    pub inside: bool,
    pub over_point: Vec4,
}

impl Comp {
    //Creates a new Comp
    pub fn new(t: f64, object: Rc<Sphere>, point: Vec4, e_vec: Vec4, n_vec: Vec4, inside: bool, over_point: Vec4) -> Comp {
        Comp {
            t,
            object,
            point,
            e_vec,
            n_vec,
            inside,
            over_point,
        }
    }

    //Prepares vars for shading
    pub fn compute_vars(intersection: Intersection, ray: &Ray) -> Comp {
        let t = intersection.get_t();
        let object = intersection.get_object();
        let point = Ray::position(ray, &t);
        let mut n_vec = Vec4::normal(&object, &point);
        let e_vec = ray.get_direction().negate();
        let mut inside = false;
        let over_point = &point + (&n_vec.normalize() * (f64::EPSILON + 0.00001));
        if Vec4::dot(&n_vec, &e_vec) < 0.0 {
            inside = true;
            n_vec = n_vec.negate();
        }
        Comp::new(t, object, over_point.clone(), e_vec, n_vec, inside, over_point)
    }
}