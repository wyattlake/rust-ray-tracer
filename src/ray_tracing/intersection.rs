use crate::objects::general::Object;
use std::rc::Rc;

//Intersection stores the time of intersection and an Rc to the Object
#[derive(Debug, PartialEq)]
pub struct Intersection {
    t: f64,
    object: Rc<Object>,
}

impl Intersection where {
    //Creates a new intersection
    pub fn new(t: f64, object: Rc<Object>) -> Intersection {
        Intersection {
            t: t,
            object: object,
        }
    }

    //Finds which intersection is visible given a list of intersection
    pub fn hit(list: &[Intersection]) -> Option<Intersection> {
        let mut min_val = f64::MAX;
        let mut min_index = -1;
        for i in 0..(list.len() as i32) {
            if list[i as usize].t < min_val && list[i as usize].t > 0.0 {
                min_val = list[i as usize].t;
                min_index = i;
            }
        };
        if min_index != -1 {
            Some(list[min_index as usize].clone())
        }
        else {
            None
        }
    }

    //Gets the t value of an intersection
    pub fn get_t(&self) -> f64 {
        self.t
    }

    //Gets the object value of an intersection
    pub fn get_object(self) -> Rc<Object> {
        self.object
    }
}

impl Clone for Intersection {
    //Clones an Intersection
    fn clone(&self) -> Intersection {
        Intersection {
            t: self.t,
            object: Rc::clone(&(*self).object)
        }
    }
}