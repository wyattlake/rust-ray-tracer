use std::ops::*;

//Vec4 is a wrapper for Tuple
#[derive(Debug, PartialEq, Clone)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

impl Vec4 {
    //Checks if a given Vec4 is a vector
    fn is_vector(vector: &Vec4) {
        if vector.3 != 0.0 {
            panic!("Vec4 {:?} cannot be a point", vector)
        }
    }

    //Creates a new Vec4
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4(x, y, z, w)
    }

    //Negates a Vec4
    pub fn negate(&self) -> Vec4 {
        Vec4(-self.0, -self.1, -self.2, self.3)
    }

    //Gets the magnitude of a Vec4
    pub fn magnitude(vector: &Vec4) -> f32 {
        Vec4::is_vector(&vector);
        ((vector.0 * vector.0) + (vector.1 * vector.1) + (vector.2 * vector.2)).sqrt()
    }

    //Normalizes a Vec4
    pub fn normalize(&self) -> Vec4 {
        let magnitude = Vec4::magnitude(&self);
        Vec4(self.0 / magnitude, self.1 / magnitude, self.2 / magnitude, 0.0)
    }

    //Finds the dot product of 2 Vec4
    pub fn dot(vec1: &Vec4, vec2: &Vec4) -> f32 {
        Vec4::is_vector(&vec1);
        Vec4::is_vector(&vec2);
        (vec1.0 * vec2.0) + (vec1.1 * vec2.1) + (vec1.2 * vec2.2)
    }

    //Reflects a vector about a given normal
    pub fn reflect(vector: &Vec4, normal: &Vec4) -> Vec4 {
        vector - (normal * 2.0 * Vec4::dot(vector, normal))
    }

    //Rounds a vector (used for testing)
    pub fn round(&self) -> Vec4 {
        Vec4::new((self.0 * 100.0).round() / 100.0, (self.1 * 100.0).round() / 100.0, (self.2 * 100.0).round() / 100.0, self.3.round())
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

//Vec4 * f32
impl Mul<f32> for Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: f32) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}
//&Vec4 * &f32
impl<'a, 'b> Mul<&'b f32> for &'a Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: &'b f32) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}
//&Vec4 * f32
impl<'a> Mul<f32> for &'a Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: f32) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}
//Vec4 * &f32
impl<'a> Mul<&'a f32> for Vec4 {
    type Output = Vec4;
    
    fn mul(self, other: &'a f32) -> Vec4 {
        Vec4(self.0 * other, self.1 * other, self.2 * other, self.3 * other)
    }
}

//f32 * Vec4
impl Mul<Vec4> for f32 {
    type Output = Vec4;
    
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4(other.0 * self, other.1 * self, other.2 * self, other.3 * self)
    }
}
//&f32 * &Vec4
impl<'a, 'b> Mul<&'b Vec4> for &'a f32 {
    type Output = Vec4;
    
    fn mul(self, other: &'b Vec4) -> Vec4 {
        Vec4(other.0 * self, other.1 * self, other.2 * self, other.3 * self)
    }
}
//&f32 * &Vec4
impl<'a> Mul<Vec4> for &'a f32 {
    type Output = Vec4;
    
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4(other.0 * self, other.1 * self, other.2 * self, other.3 * self)
    }
}
//f32 * &Vec4
impl<'a> Mul<&'a Vec4> for f32 {
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