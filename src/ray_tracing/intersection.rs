use crate::core::vector::Vec4;
use crate::objects::object::*;

//Intersection stores the time of intersection and an Rc to the Object
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection<'a> {
    pub t: f32,
    pub hit: Vec4,
    pub normal: Vec4,
    pub object: &'a dyn Object,
}

impl<'a> Intersection<'a> {
    //Creates a new intersection
    pub fn new(
        t: f32,
        hit: Vec4,
        normal: Vec4,
        object: &'a dyn Object,
    ) -> Intersection {
        Intersection {
            t,
            hit,
            normal,
            object,
        }
    }

    //Finds which intersection is visible given a list of intersection
    pub fn hit(list_ref: &'a Vec<Intersection>) -> Option<Intersection<'a>> {
        let mut list = list_ref.clone();
        let mut min_val = f32::MAX;
        let mut min_index: i32 = -1;
        for i in 0..(list.len() as i32) {
            if list[i as usize].t < min_val && list[i as usize].t > 0.0 {
                min_val = list[i as usize].t;
                min_index = i;
            }
        }
        if min_index != -1 {
            Some(list.remove(min_index as usize))
        } else {
            None
        }
    }
}
