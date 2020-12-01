use crate::objects::general::Object;
use std::rc::Rc;

//Intersection stores the time of intersection and an Rc to the Object
#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    t: f32,
    object: Rc<Object>,
}

impl Intersection where {
    //Creates a new intersection
    pub fn new(t: f32, object: Rc<Object>) -> Intersection {
        Intersection {
            t: t,
            object: object,
        }
    }

    //Finds which intersection is visible given a list of intersection
    pub fn hit(list: &mut Vec<Intersection>) -> Option<Intersection> {
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

    //Gets the t value of an intersection
    pub fn get_t(&self) -> f32 {
        self.t
    }

    //Gets the object value of an intersection
    pub fn get_object(self) -> Rc<Object> {
        self.object
    }
}