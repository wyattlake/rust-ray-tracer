
use crate::sphere::Sphere;
use std::ops::*;
use std::rc::Rc;

//Vec4 is a wrapper for Tuple
#[derive(Debug, PartialEq)]
pub struct Vec4(pub f64, pub f64, pub f64, pub f64);

impl Vec4 {
    //Checks if a given Vec4 is a vector
    fn is_vector(vector: &Vec4) {
        if vector.3 != 0.0 {
            panic!("Vec4 cannot be a point")
        }
    }

    //Creates a new Vec4
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4(x, y, z, w)
    }

    //Negates a Vec4
    pub fn negate(&self) -> Vec4 {
        Vec4(-self.0, -self.1, -self.2, self.3)
    }

    //Gets the magnitude of a Vec4
    pub fn magnitude(vector: &Vec4) -> f64 {
        Vec4::is_vector(&vector);
        ((vector.0 * vector.0) + (vector.1 * vector.1) + (vector.2 * vector.2)).sqrt()
    }

    //Normalizes a Vec4
    pub fn normalize(&self) -> Vec4 {
        let magnitude = Vec4::magnitude(&self);
        Vec4(self.0 / magnitude, self.1 / magnitude, self.2 / magnitude, 0.0)
    }

    //Finds the dot product of 2 Vec4
    pub fn dot(vec1: &Vec4, vec2: &Vec4) -> f64 {
        Vec4::is_vector(&vec1);
        Vec4::is_vector(&vec2);
        (vec1.0 * vec2.0) + (vec1.1 * vec2.1) + (vec1.2 * vec2.2)
    }

    //Finds the normal of a given point on a sphere
    pub fn normal(sphere: &Rc<Sphere>, world_point: &Vec4) -> Vec4 {
        let object_point = (*sphere.get_transform()).inverse().unwrap() * world_point;
        let object_normal = object_point - Vec4::new(0.0, 0.0, 0.0, 1.0);
        let mut world_normal = (*sphere.get_transform()).inverse().unwrap().transpose() * object_normal; 
        world_normal.3 = 0.0;
        world_normal.normalize()
    }

    //Reflects a vector about a given normal
    pub fn reflect(vector: &Vec4, normal: &Vec4) -> Vec4 {
        vector - (normal * 2.0 * Vec4::dot(vector, normal))
    }

    //Rounds a vector (used for testing)
    pub fn round(&self) -> Vec4 {
        Vec4::new(self.0.round(), self.1.round(), self.2.round(), 0.0)
    }
}


//Vec4 + Vec4
impl Add for Vec4 {
    type Output = Vec4;
    
    fn add(self, other: Vec4) -> Vec4 {
        Vec4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}
//&Vec4 + &Vec4
impl<'a, 'b> Add<&'b Vec4> for &'a Vec4 {
    type Output = Vec4;
    
    fn add(self, other: &'b Vec4) -> Vec4 {
        Vec4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}
//&Vec4 + Vec4
impl<'a> Add<Vec4> for &'a Vec4 {
    type Output = Vec4;
    
    fn add(self, other: Vec4) -> Vec4 {
        Vec4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}
//Vec4 + &Vec4
impl<'a> Add<&'a Vec4> for Vec4 {
    type Output = Vec4;
    
    fn add(self, other: &'a Vec4) -> Vec4 {
        Vec4(self.0 + other.0, self.1 + other.1, self.2 + other.2, self.3 + other.3)
    }
}

//Vec4 - Vec4
impl Sub for Vec4 {
    type Output = Vec4;
    
    fn sub(self, other: Vec4) -> Vec4 {
        Vec4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3)
    }
}
//&Vec4 - &Vec4s
impl<'a, 'b> Sub<&'b Vec4> for &'a Vec4 {
    type Output = Vec4;
    
    fn sub(self, other: &'b Vec4) -> Vec4 {
        Vec4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3)
    }
}
//&Vec4 - Vec4
impl<'a> Sub<Vec4> for &'a Vec4 {
    type Output = Vec4;
    
    fn sub(self, other: Vec4) -> Vec4 {
        Vec4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3)
    }
}
//Vec4 - &Vec4
impl<'a> Sub<&'a Vec4> for Vec4 {
    type Output = Vec4;
    
    fn sub(self, other: &'a Vec4) -> Vec4 {
        Vec4(self.0 - other.0, self.1 - other.1, self.2 - other.2, self.3 - other.3)
    }
}

//Vec4 * f64
impl Mul<f64> for Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: f64) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}
//&Vec4 * &f64
impl<'a, 'b> Mul<&'b f64> for &'a Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: &'b f64) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}
//&Vec4 * f64
impl<'a> Mul<f64> for &'a Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: f64) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}
//Vec4 * &f64
impl<'a> Mul<&'a f64> for Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: &'a f64) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}

//f64 * Vec4
impl Mul<Vec4> for f64 {
    type Output = Vec4;
    
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4(other.0 * self, other.1 * self, other.2 * self, other.3 * self)
    }
}
//&f64 * &Vec4
impl<'a, 'b> Mul<&'b Vec4> for &'a f64 {
    type Output = Vec4;
    
    fn mul(self, other: &'b Vec4) -> Vec4 {
        Vec4(other.0 * self, other.1 * self, other.2 * self, other.3 * self)
    }
}
//&f64 * &Vec4
impl<'a> Mul<Vec4> for &'a f64 {
    type Output = Vec4;
    
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4(other.0 * self, other.1 * self, other.2 * self, other.3 * self)
    }
}
//f64 * &Vec4
impl<'a> Mul<&'a Vec4> for f64 {
    type Output = Vec4;
    
    fn mul(self, other: &'a Vec4) -> Vec4 {
        Vec4(other.0 * self, other.1 * self, other.2 * self, other.3 * self)
    }
}

//VecA * VecA
impl Mul for Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0), 0.0)
    }
}
//&VecA * &VecA
impl<'a, 'b> Mul<&'b Vec4> for &'a Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: &'b Vec4) -> Vec4 {
        Vec4((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0), 0.0)
    }
}
//&VecA * VecA
impl<'a> Mul<Vec4> for &'a Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0), 0.0)
    }
}
//VecA * &VecA
impl<'a> Mul<&'a Vec4> for Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: &'a Vec4) -> Vec4 {
        Vec4((self.1 * other.2) - (self.2 * other.1), (self.2 * other.0) - (self.0 * other.2), (self.0 * other.1) - (self.1 * other.0), 0.0)
    }
}

//Clones Vec4 
impl Clone for Vec4 {
    fn clone(&self) -> Vec4 {
        Vec4(self.0, self.1, self.2, self.3)
    }
}