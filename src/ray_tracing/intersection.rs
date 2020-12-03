use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use crate::ray_tracing::material::Material;

//Intersection stores the time of intersection and an Rc to the Object
#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub hit: Vec4,
    pub object_inverse: &'a Matrix4x4,
    pub normal: Vec4,
    pub material: &'a Material,
}

impl<'a> Intersection<'a> where {
    //Creates a new intersection
    pub fn new(t: f32, hit: Vec4, object_inverse: &'a Matrix4x4, normal: Vec4, material: &'a Material) -> Intersection<'a> {
        Intersection {
            t,
            hit,
            normal,
            material,
            object_inverse,
        }
    }

    //Finds which intersection is visible given a list of intersection
    pub fn hit(list_1: Vec<Intersection<'a>>) -> Option<Intersection<'a>> {
        let mut list = list_1.clone();
        let mut min_val = f32::MAX;
        let mut min_index = -1;
        for i in 0..(list.len() as i32) {
            if list[i as usize].t < min_val && list[i as usize].t > 0.0 {
                min_val = list[i as usize].t;
                min_index = i;
            }
        };
        if min_index != -1 {
            Some(list.remove(min_index as usize))
        }
        else {
            None
        }
    }
}