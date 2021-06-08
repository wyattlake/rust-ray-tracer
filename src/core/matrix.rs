use crate::core::vector::*;
use crate::misc::axis::*;
use std::fmt::Debug;
use std::ops::*;

pub const IDENTITY: Matrix4x4 = Matrix4x4(
    (1.0, 0.0, 0.0, 0.0),
    (0.0, 1.0, 0.0, 0.0),
    (0.0, 0.0, 1.0, 0.0),
    (0.0, 0.0, 0.0, 1.0),
);

//Matrix4x4 is composed of nested tuples
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix4x4(
    pub (f32, f32, f32, f32),
    pub (f32, f32, f32, f32),
    pub (f32, f32, f32, f32),
    (f32, f32, f32, f32),
);

impl Matrix4x4 {
    //Creates a new Matrix4x4
    pub fn new(
        row1: (f32, f32, f32, f32),
        row2: (f32, f32, f32, f32),
        row3: (f32, f32, f32, f32),
        row4: (f32, f32, f32, f32),
    ) -> Matrix4x4 {
        Matrix4x4(row1, row2, row3, row4)
    }

    //Creates an instance of the 4x4 identity Matrix
    pub fn identity() -> Matrix4x4 {
        Matrix4x4::new(
            (1.0, 0.0, 0.0, 0.0),
            (0.0, 1.0, 0.0, 0.0),
            (0.0, 0.0, 1.0, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    //Transposes a Matrix
    pub fn transpose(&self) -> Matrix4x4 {
        Matrix4x4(
            (self.0 .0, self.1 .0, self.2 .0, self.3 .0),
            (self.0 .1, self.1 .1, self.2 .1, self.3 .1),
            (self.0 .2, self.1 .2, self.2 .2, self.3 .2),
            (self.0 .3, self.1 .3, self.2 .3, self.3 .3),
        )
    }

    //Finds a sub matrix of a Matrix4x4 given a row and column to remove
    pub fn sub_matrix(matrix: &Matrix4x4, row: usize, column: usize) -> Matrix3x3 {
        let mut m = vec![
            vec![matrix.0 .0, matrix.0 .1, matrix.0 .2, matrix.0 .3],
            vec![matrix.1 .0, matrix.1 .1, matrix.1 .2, matrix.1 .3],
            vec![matrix.2 .0, matrix.2 .1, matrix.2 .2, matrix.2 .3],
            vec![matrix.3 .0, matrix.3 .1, matrix.3 .2, matrix.3 .3],
        ];
        m.remove(row);
        for x in 0..3 {
            m[x].remove(column);
        }
        Matrix3x3::new(
            (m[0][0], m[0][1], m[0][2]),
            (m[1][0], m[1][1], m[1][2]),
            (m[2][0], m[2][1], m[2][2]),
        )
    }

    //Finds the cofactor of a Matrix3x3
    pub fn cofactor(matrix: &Matrix4x4, row: usize, column: usize) -> f32 {
        let sub_matrix: Matrix3x3 = Matrix4x4::sub_matrix(matrix, row, column);
        let mut sign = -1.0;
        if (row + column) % 2 == 0 {
            sign = 1.0;
        }
        Matrix3x3::determinant(&sub_matrix) * sign
    }

    //Gets a value from a Matrix4x4
    pub fn get(&self, row: usize, column: usize) -> f32 {
        let m = [
            [self.0 .0, self.0 .1, self.0 .2, self.0 .3],
            [self.1 .0, self.1 .1, self.1 .2, self.1 .3],
            [self.2 .0, self.2 .1, self.2 .2, self.2 .3],
            [self.3 .0, self.3 .1, self.3 .2, self.3 .3],
        ];
        m[row][column]
    }

    //Finds the determinant of a Matrix4x4
    pub fn determinant(matrix: &Matrix4x4) -> f32 {
        let mut determinant: f32 = 0.0;
        for i in 0..4 {
            determinant = determinant + (Matrix4x4::cofactor(&matrix, 0, i) * matrix.get(0, i));
        }
        determinant
    }

    //Finds the inverse of a Matrix4x4
    pub fn inverse(&self) -> Option<Matrix4x4> {
        let det = Matrix4x4::determinant(&self);
        if det < 0.0 {
            None
        } else {
            let cofactor_matrix = Matrix4x4::new(
                (
                    Matrix4x4::cofactor(&self, 0, 0),
                    Matrix4x4::cofactor(&self, 0, 1),
                    Matrix4x4::cofactor(&self, 0, 2),
                    Matrix4x4::cofactor(&self, 0, 3),
                ),
                (
                    Matrix4x4::cofactor(&self, 1, 0),
                    Matrix4x4::cofactor(&self, 1, 1),
                    Matrix4x4::cofactor(&self, 1, 2),
                    Matrix4x4::cofactor(&self, 1, 3),
                ),
                (
                    Matrix4x4::cofactor(&self, 2, 0),
                    Matrix4x4::cofactor(&self, 2, 1),
                    Matrix4x4::cofactor(&self, 2, 2),
                    Matrix4x4::cofactor(&self, 2, 3),
                ),
                (
                    Matrix4x4::cofactor(&self, 3, 0),
                    Matrix4x4::cofactor(&self, 3, 1),
                    Matrix4x4::cofactor(&self, 3, 2),
                    Matrix4x4::cofactor(&self, 3, 3),
                ),
            )
            .transpose();
            let result = Matrix4x4(
                (
                    cofactor_matrix.0 .0 / det,
                    cofactor_matrix.0 .1 / det,
                    cofactor_matrix.0 .2 / det,
                    cofactor_matrix.0 .3 / det,
                ),
                (
                    cofactor_matrix.1 .0 / det,
                    cofactor_matrix.1 .1 / det,
                    cofactor_matrix.1 .2 / det,
                    cofactor_matrix.1 .3 / det,
                ),
                (
                    cofactor_matrix.2 .0 / det,
                    cofactor_matrix.2 .1 / det,
                    cofactor_matrix.2 .2 / det,
                    cofactor_matrix.2 .3 / det,
                ),
                (
                    cofactor_matrix.3 .0 / det,
                    cofactor_matrix.3 .1 / det,
                    cofactor_matrix.3 .2 / det,
                    cofactor_matrix.3 .3 / det,
                ),
            );
            Some(result)
        }
    }

    //Creates a translation matrix
    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4::new(
            (1.0, 0.0, 0.0, x),
            (0.0, 1.0, 0.0, y),
            (0.0, 0.0, 1.0, z),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    //Creates a translation matrix
    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix4x4 {
        Matrix4x4::new(
            (x, 0.0, 0.0, 0.0),
            (0.0, y, 0.0, 0.0),
            (0.0, 0.0, z, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    //Creates a reflection matrix
    pub fn reflection(axis: Axis) -> Matrix4x4 {
        match axis {
            Axis::X => Matrix4x4::scaling(-1.0, 1.0, 1.0),
            Axis::Y => Matrix4x4::scaling(1.0, -1.0, 1.0),
            Axis::Z => Matrix4x4::scaling(1.0, 1.0, -1.0),
        }
    }

    //Creates a rotation matrix
    pub fn rotation(axis: Axis, degrees: f32) -> Matrix4x4 {
        let r = degrees.to_radians();
        match axis {
            Axis::X => Matrix4x4::new(
                (1.0, 0.0, 0.0, 0.0),
                (0.0, r.cos(), -r.sin(), 0.0),
                (0.0, r.sin(), r.cos(), 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ),
            Axis::Y => Matrix4x4::new(
                (r.cos(), 0.0, r.sin(), 0.0),
                (0.0, 1.0, 0.0, 0.0),
                (-r.sin(), 0.0, r.cos(), 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ),
            Axis::Z => Matrix4x4::new(
                (r.cos(), -r.sin(), 0.0, 0.0),
                (r.sin(), r.cos(), 0.0, 0.0),
                (0.0, 0.0, 1.0, 0.0),
                (0.0, 0.0, 0.0, 1.0),
            ),
        }
    }

    //Creates a shearing matrix
    pub fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix4x4 {
        Matrix4x4::new(
            (1.0, x_y, x_z, 0.0),
            (y_x, 1.0, y_z, 0.0),
            (z_x, z_y, 1.0, 0.0),
            (0.0, 0.0, 0.0, 1.0),
        )
    }

    //Rounds a matrix for testing
    pub fn round(&self) -> Matrix4x4 {
        let mut m = vec![
            vec![self.0 .0, self.0 .1, self.0 .2, self.0 .3],
            vec![self.1 .0, self.1 .1, self.1 .2, self.1 .3],
            vec![self.2 .0, self.2 .1, self.2 .2, self.2 .3],
            vec![self.3 .0, self.3 .1, self.3 .2, self.3 .3],
        ];
        for y in 0..4 {
            for x in 0..4 {
                m[y][x] = ((m[y][x] * 10000.0).round()) / 10000.0;
            }
        }
        Matrix4x4::new(
            (m[0][0], m[0][1], m[0][2], m[0][3]),
            (m[1][0], m[1][1], m[1][2], m[1][3]),
            (m[2][0], m[2][1], m[2][2], m[2][3]),
            (m[3][0], m[3][1], m[3][2], m[3][3]),
        )
    }

    //Creates a new view transform matrix
    pub fn view_transform(view_start_pos: Vec4, view_end_pos: Vec4, up_vec: Vec4) -> Matrix4x4 {
        let forward = (view_end_pos - &view_start_pos).normalize();
        let up_normalized = up_vec.normalize();
        let left = &forward * up_normalized;
        let true_up = &left * &forward;
        let orientation = Matrix4x4::new(
            (left.0, left.1, left.2, 0.0),
            (true_up.0, true_up.1, true_up.2, 0.0),
            (
                forward.negate().0,
                forward.negate().1,
                forward.negate().2,
                0.0,
            ),
            (0.0, 0.0, 0.0, 1.0),
        );
        orientation
            * Matrix4x4::translation(-view_start_pos.0, -view_start_pos.1, -view_start_pos.2)
    }
}

//Matrix4x4 * Matrix4x4
impl Mul for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4(
            (
                (self.0 .0 * other.0 .0)
                    + (self.0 .1 * other.1 .0)
                    + (self.0 .2 * other.2 .0)
                    + (self.0 .3 * other.3 .0),
                (self.0 .0 * other.0 .1)
                    + (self.0 .1 * other.1 .1)
                    + (self.0 .2 * other.2 .1)
                    + (self.0 .3 * other.3 .1),
                (self.0 .0 * other.0 .2)
                    + (self.0 .1 * other.1 .2)
                    + (self.0 .2 * other.2 .2)
                    + (self.0 .3 * other.3 .2),
                (self.0 .0 * other.0 .3)
                    + (self.0 .1 * other.1 .3)
                    + (self.0 .2 * other.2 .3)
                    + (self.0 .3 * other.3 .3),
            ),
            (
                (self.1 .0 * other.0 .0)
                    + (self.1 .1 * other.1 .0)
                    + (self.1 .2 * other.2 .0)
                    + (self.1 .3 * other.3 .0),
                (self.1 .0 * other.0 .1)
                    + (self.1 .1 * other.1 .1)
                    + (self.1 .2 * other.2 .1)
                    + (self.1 .3 * other.3 .1),
                (self.1 .0 * other.0 .2)
                    + (self.1 .1 * other.1 .2)
                    + (self.1 .2 * other.2 .2)
                    + (self.1 .3 * other.3 .2),
                (self.1 .0 * other.0 .3)
                    + (self.1 .1 * other.1 .3)
                    + (self.1 .2 * other.2 .3)
                    + (self.1 .3 * other.3 .3),
            ),
            (
                (self.2 .0 * other.0 .0)
                    + (self.2 .1 * other.1 .0)
                    + (self.2 .2 * other.2 .0)
                    + (self.2 .3 * other.3 .0),
                (self.2 .0 * other.0 .1)
                    + (self.2 .1 * other.1 .1)
                    + (self.2 .2 * other.2 .1)
                    + (self.2 .3 * other.3 .1),
                (self.2 .0 * other.0 .2)
                    + (self.2 .1 * other.1 .2)
                    + (self.2 .2 * other.2 .2)
                    + (self.2 .3 * other.3 .2),
                (self.2 .0 * other.0 .3)
                    + (self.2 .1 * other.1 .3)
                    + (self.2 .2 * other.2 .3)
                    + (self.2 .3 * other.3 .3),
            ),
            (
                (self.3 .0 * other.0 .0)
                    + (self.3 .1 * other.1 .0)
                    + (self.3 .2 * other.2 .0)
                    + (self.3 .3 * other.3 .0),
                (self.3 .0 * other.0 .1)
                    + (self.3 .1 * other.1 .1)
                    + (self.3 .2 * other.2 .1)
                    + (self.3 .3 * other.3 .1),
                (self.3 .0 * other.0 .2)
                    + (self.3 .1 * other.1 .2)
                    + (self.3 .2 * other.2 .2)
                    + (self.3 .3 * other.3 .2),
                (self.3 .0 * other.0 .3)
                    + (self.3 .1 * other.1 .3)
                    + (self.3 .2 * other.2 .3)
                    + (self.3 .3 * other.3 .3),
            ),
        )
    }
}
//&Matrix4x4 * &Matrix4x4
impl<'a, 'b> Mul<&'b Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, other: &'b Matrix4x4) -> Matrix4x4 {
        Matrix4x4(
            (
                (self.0 .0 * other.0 .0)
                    + (self.0 .1 * other.1 .0)
                    + (self.0 .2 * other.2 .0)
                    + (self.0 .3 * other.3 .0),
                (self.0 .0 * other.0 .1)
                    + (self.0 .1 * other.1 .1)
                    + (self.0 .2 * other.2 .1)
                    + (self.0 .3 * other.3 .1),
                (self.0 .0 * other.0 .2)
                    + (self.0 .1 * other.1 .2)
                    + (self.0 .2 * other.2 .2)
                    + (self.0 .3 * other.3 .2),
                (self.0 .0 * other.0 .3)
                    + (self.0 .1 * other.1 .3)
                    + (self.0 .2 * other.2 .3)
                    + (self.0 .3 * other.3 .3),
            ),
            (
                (self.1 .0 * other.0 .0)
                    + (self.1 .1 * other.1 .0)
                    + (self.1 .2 * other.2 .0)
                    + (self.1 .3 * other.3 .0),
                (self.1 .0 * other.0 .1)
                    + (self.1 .1 * other.1 .1)
                    + (self.1 .2 * other.2 .1)
                    + (self.1 .3 * other.3 .1),
                (self.1 .0 * other.0 .2)
                    + (self.1 .1 * other.1 .2)
                    + (self.1 .2 * other.2 .2)
                    + (self.1 .3 * other.3 .2),
                (self.1 .0 * other.0 .3)
                    + (self.1 .1 * other.1 .3)
                    + (self.1 .2 * other.2 .3)
                    + (self.1 .3 * other.3 .3),
            ),
            (
                (self.2 .0 * other.0 .0)
                    + (self.2 .1 * other.1 .0)
                    + (self.2 .2 * other.2 .0)
                    + (self.2 .3 * other.3 .0),
                (self.2 .0 * other.0 .1)
                    + (self.2 .1 * other.1 .1)
                    + (self.2 .2 * other.2 .1)
                    + (self.2 .3 * other.3 .1),
                (self.2 .0 * other.0 .2)
                    + (self.2 .1 * other.1 .2)
                    + (self.2 .2 * other.2 .2)
                    + (self.2 .3 * other.3 .2),
                (self.2 .0 * other.0 .3)
                    + (self.2 .1 * other.1 .3)
                    + (self.2 .2 * other.2 .3)
                    + (self.2 .3 * other.3 .3),
            ),
            (
                (self.3 .0 * other.0 .0)
                    + (self.3 .1 * other.1 .0)
                    + (self.3 .2 * other.2 .0)
                    + (self.3 .3 * other.3 .0),
                (self.3 .0 * other.0 .1)
                    + (self.3 .1 * other.1 .1)
                    + (self.3 .2 * other.2 .1)
                    + (self.3 .3 * other.3 .1),
                (self.3 .0 * other.0 .2)
                    + (self.3 .1 * other.1 .2)
                    + (self.3 .2 * other.2 .2)
                    + (self.3 .3 * other.3 .2),
                (self.3 .0 * other.0 .3)
                    + (self.3 .1 * other.1 .3)
                    + (self.3 .2 * other.2 .3)
                    + (self.3 .3 * other.3 .3),
            ),
        )
    }
}
//&Matrix4x4 * Matrix4x4
impl<'a> Mul<Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4(
            (
                (self.0 .0 * other.0 .0)
                    + (self.0 .1 * other.1 .0)
                    + (self.0 .2 * other.2 .0)
                    + (self.0 .3 * other.3 .0),
                (self.0 .0 * other.0 .1)
                    + (self.0 .1 * other.1 .1)
                    + (self.0 .2 * other.2 .1)
                    + (self.0 .3 * other.3 .1),
                (self.0 .0 * other.0 .2)
                    + (self.0 .1 * other.1 .2)
                    + (self.0 .2 * other.2 .2)
                    + (self.0 .3 * other.3 .2),
                (self.0 .0 * other.0 .3)
                    + (self.0 .1 * other.1 .3)
                    + (self.0 .2 * other.2 .3)
                    + (self.0 .3 * other.3 .3),
            ),
            (
                (self.1 .0 * other.0 .0)
                    + (self.1 .1 * other.1 .0)
                    + (self.1 .2 * other.2 .0)
                    + (self.1 .3 * other.3 .0),
                (self.1 .0 * other.0 .1)
                    + (self.1 .1 * other.1 .1)
                    + (self.1 .2 * other.2 .1)
                    + (self.1 .3 * other.3 .1),
                (self.1 .0 * other.0 .2)
                    + (self.1 .1 * other.1 .2)
                    + (self.1 .2 * other.2 .2)
                    + (self.1 .3 * other.3 .2),
                (self.1 .0 * other.0 .3)
                    + (self.1 .1 * other.1 .3)
                    + (self.1 .2 * other.2 .3)
                    + (self.1 .3 * other.3 .3),
            ),
            (
                (self.2 .0 * other.0 .0)
                    + (self.2 .1 * other.1 .0)
                    + (self.2 .2 * other.2 .0)
                    + (self.2 .3 * other.3 .0),
                (self.2 .0 * other.0 .1)
                    + (self.2 .1 * other.1 .1)
                    + (self.2 .2 * other.2 .1)
                    + (self.2 .3 * other.3 .1),
                (self.2 .0 * other.0 .2)
                    + (self.2 .1 * other.1 .2)
                    + (self.2 .2 * other.2 .2)
                    + (self.2 .3 * other.3 .2),
                (self.2 .0 * other.0 .3)
                    + (self.2 .1 * other.1 .3)
                    + (self.2 .2 * other.2 .3)
                    + (self.2 .3 * other.3 .3),
            ),
            (
                (self.3 .0 * other.0 .0)
                    + (self.3 .1 * other.1 .0)
                    + (self.3 .2 * other.2 .0)
                    + (self.3 .3 * other.3 .0),
                (self.3 .0 * other.0 .1)
                    + (self.3 .1 * other.1 .1)
                    + (self.3 .2 * other.2 .1)
                    + (self.3 .3 * other.3 .1),
                (self.3 .0 * other.0 .2)
                    + (self.3 .1 * other.1 .2)
                    + (self.3 .2 * other.2 .2)
                    + (self.3 .3 * other.3 .2),
                (self.3 .0 * other.0 .3)
                    + (self.3 .1 * other.1 .3)
                    + (self.3 .2 * other.2 .3)
                    + (self.3 .3 * other.3 .3),
            ),
        )
    }
}
//&Matrix4x4 * &Matrix4x4
impl<'a> Mul<&'a Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, other: &'a Matrix4x4) -> Matrix4x4 {
        Matrix4x4(
            (
                (self.0 .0 * other.0 .0)
                    + (self.0 .1 * other.1 .0)
                    + (self.0 .2 * other.2 .0)
                    + (self.0 .3 * other.3 .0),
                (self.0 .0 * other.0 .1)
                    + (self.0 .1 * other.1 .1)
                    + (self.0 .2 * other.2 .1)
                    + (self.0 .3 * other.3 .1),
                (self.0 .0 * other.0 .2)
                    + (self.0 .1 * other.1 .2)
                    + (self.0 .2 * other.2 .2)
                    + (self.0 .3 * other.3 .2),
                (self.0 .0 * other.0 .3)
                    + (self.0 .1 * other.1 .3)
                    + (self.0 .2 * other.2 .3)
                    + (self.0 .3 * other.3 .3),
            ),
            (
                (self.1 .0 * other.0 .0)
                    + (self.1 .1 * other.1 .0)
                    + (self.1 .2 * other.2 .0)
                    + (self.1 .3 * other.3 .0),
                (self.1 .0 * other.0 .1)
                    + (self.1 .1 * other.1 .1)
                    + (self.1 .2 * other.2 .1)
                    + (self.1 .3 * other.3 .1),
                (self.1 .0 * other.0 .2)
                    + (self.1 .1 * other.1 .2)
                    + (self.1 .2 * other.2 .2)
                    + (self.1 .3 * other.3 .2),
                (self.1 .0 * other.0 .3)
                    + (self.1 .1 * other.1 .3)
                    + (self.1 .2 * other.2 .3)
                    + (self.1 .3 * other.3 .3),
            ),
            (
                (self.2 .0 * other.0 .0)
                    + (self.2 .1 * other.1 .0)
                    + (self.2 .2 * other.2 .0)
                    + (self.2 .3 * other.3 .0),
                (self.2 .0 * other.0 .1)
                    + (self.2 .1 * other.1 .1)
                    + (self.2 .2 * other.2 .1)
                    + (self.2 .3 * other.3 .1),
                (self.2 .0 * other.0 .2)
                    + (self.2 .1 * other.1 .2)
                    + (self.2 .2 * other.2 .2)
                    + (self.2 .3 * other.3 .2),
                (self.2 .0 * other.0 .3)
                    + (self.2 .1 * other.1 .3)
                    + (self.2 .2 * other.2 .3)
                    + (self.2 .3 * other.3 .3),
            ),
            (
                (self.3 .0 * other.0 .0)
                    + (self.3 .1 * other.1 .0)
                    + (self.3 .2 * other.2 .0)
                    + (self.3 .3 * other.3 .0),
                (self.3 .0 * other.0 .1)
                    + (self.3 .1 * other.1 .1)
                    + (self.3 .2 * other.2 .1)
                    + (self.3 .3 * other.3 .1),
                (self.3 .0 * other.0 .2)
                    + (self.3 .1 * other.1 .2)
                    + (self.3 .2 * other.2 .2)
                    + (self.3 .3 * other.3 .2),
                (self.3 .0 * other.0 .3)
                    + (self.3 .1 * other.1 .3)
                    + (self.3 .2 * other.2 .3)
                    + (self.3 .3 * other.3 .3),
            ),
        )
    }
}

//Matrix4x4 * Vec4
impl Mul<Vec4> for Matrix4x4 {
    type Output = Vec4;
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new(
            (self.0 .0 * other.0)
                + (self.0 .1 * other.1)
                + (self.0 .2 * other.2)
                + (self.0 .3 * other.3),
            (self.1 .0 * other.0)
                + (self.1 .1 * other.1)
                + (self.1 .2 * other.2)
                + (self.1 .3 * other.3),
            (self.2 .0 * other.0)
                + (self.2 .1 * other.1)
                + (self.2 .2 * other.2)
                + (self.2 .3 * other.3),
            (self.3 .0 * other.0)
                + (self.3 .1 * other.1)
                + (self.3 .2 * other.2)
                + (self.3 .3 * other.3),
        )
    }
}
//&Matrix4x4 * &Vec4
impl<'a, 'b> Mul<&'b Vec4> for &'a Matrix4x4 {
    type Output = Vec4;
    fn mul(self, other: &'b Vec4) -> Vec4 {
        Vec4::new(
            (self.0 .0 * other.0)
                + (self.0 .1 * other.1)
                + (self.0 .2 * other.2)
                + (self.0 .3 * other.3),
            (self.1 .0 * other.0)
                + (self.1 .1 * other.1)
                + (self.1 .2 * other.2)
                + (self.1 .3 * other.3),
            (self.2 .0 * other.0)
                + (self.2 .1 * other.1)
                + (self.2 .2 * other.2)
                + (self.2 .3 * other.3),
            (self.3 .0 * other.0)
                + (self.3 .1 * other.1)
                + (self.3 .2 * other.2)
                + (self.3 .3 * other.3),
        )
    }
}
//&Matrix4x4 * Vec4
impl<'a> Mul<Vec4> for &'a Matrix4x4 {
    type Output = Vec4;
    fn mul(self, other: Vec4) -> Vec4 {
        Vec4::new(
            (self.0 .0 * other.0)
                + (self.0 .1 * other.1)
                + (self.0 .2 * other.2)
                + (self.0 .3 * other.3),
            (self.1 .0 * other.0)
                + (self.1 .1 * other.1)
                + (self.1 .2 * other.2)
                + (self.1 .3 * other.3),
            (self.2 .0 * other.0)
                + (self.2 .1 * other.1)
                + (self.2 .2 * other.2)
                + (self.2 .3 * other.3),
            (self.3 .0 * other.0)
                + (self.3 .1 * other.1)
                + (self.3 .2 * other.2)
                + (self.3 .3 * other.3),
        )
    }
}
//Matrix4x4 * Vec4
impl<'a> Mul<&'a Vec4> for Matrix4x4 {
    type Output = Vec4;
    fn mul(self, other: &'a Vec4) -> Vec4 {
        Vec4::new(
            (self.0 .0 * other.0)
                + (self.0 .1 * other.1)
                + (self.0 .2 * other.2)
                + (self.0 .3 * other.3),
            (self.1 .0 * other.0)
                + (self.1 .1 * other.1)
                + (self.1 .2 * other.2)
                + (self.1 .3 * other.3),
            (self.2 .0 * other.0)
                + (self.2 .1 * other.1)
                + (self.2 .2 * other.2)
                + (self.2 .3 * other.3),
            (self.3 .0 * other.0)
                + (self.3 .1 * other.1)
                + (self.3 .2 * other.2)
                + (self.3 .3 * other.3),
        )
    }
}

//Matrix3x3 is composed of nested tuples
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix3x3(
    pub (f32, f32, f32),
    pub (f32, f32, f32),
    pub (f32, f32, f32),
);

impl Matrix3x3 {
    //Creates a new Matrix3x3
    pub fn new(row1: (f32, f32, f32), row2: (f32, f32, f32), row3: (f32, f32, f32)) -> Matrix3x3 {
        Matrix3x3(row1, row2, row3)
    }

    //Creates an instance of the 3x3 identity Matrix
    pub fn identity() -> Matrix3x3 {
        Matrix3x3::new((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0))
    }

    //Transposes a Matrix3x3
    pub fn transpose(&self) -> Matrix3x3 {
        Matrix3x3(
            (self.0 .0, self.1 .0, self.2 .0),
            (self.0 .1, self.1 .1, self.2 .1),
            (self.0 .2, self.1 .2, self.2 .2),
        )
    }

    //Finds a sub matrix of a Matrix3x3 given a row and column to remove
    pub fn sub_matrix(matrix: &Matrix3x3, row: usize, column: usize) -> Matrix2x2 {
        let mut m = vec![
            vec![matrix.0 .0, matrix.0 .1, matrix.0 .2],
            vec![matrix.1 .0, matrix.1 .1, matrix.1 .2],
            vec![matrix.2 .0, matrix.2 .1, matrix.2 .2],
        ];
        m.remove(row);
        for x in 0..2 {
            m[x].remove(column);
        }
        Matrix2x2::new((m[0][0], m[0][1]), (m[1][0], m[1][1]))
    }

    //Finds the cofactor of a Matrix3x3
    pub fn cofactor(matrix: &Matrix3x3, row: usize, column: usize) -> f32 {
        let sub_matrix: Matrix2x2 = Matrix3x3::sub_matrix(matrix, row, column);
        let mut sign = -1.0;
        if (row + column) % 2 == 0 {
            sign = 1.0;
        }
        Matrix2x2::determinant(sub_matrix) * sign
    }

    //Gets a value from a Matrix3x3
    pub fn get(&self, row: usize, column: usize) -> f32 {
        let m = [
            [self.0 .0, self.0 .1, self.0 .2],
            [self.1 .0, self.1 .1, self.1 .2],
            [self.2 .0, self.2 .1, self.2 .2],
        ];
        m[row][column]
    }

    //Finds the determinant of a Matrix3x3
    pub fn determinant(matrix: &Matrix3x3) -> f32 {
        let mut determinant: f32 = 0.0;
        for i in 0..3 {
            determinant = determinant + (Matrix3x3::cofactor(matrix, 0, i) * matrix.get(0, i));
        }
        determinant
    }

    //Finds the inverse of a Matrix4x4
    pub fn inverse(self) -> Option<Matrix3x3> {
        let det = Matrix3x3::determinant(&self);
        if det < 0.0 {
            None
        } else {
            let cofactor_matrix = Matrix3x3::new(
                (
                    Matrix3x3::cofactor(&self, 0, 0),
                    Matrix3x3::cofactor(&self, 0, 1),
                    Matrix3x3::cofactor(&self, 0, 2),
                ),
                (
                    Matrix3x3::cofactor(&self, 1, 0),
                    Matrix3x3::cofactor(&self, 1, 1),
                    Matrix3x3::cofactor(&self, 1, 2),
                ),
                (
                    Matrix3x3::cofactor(&self, 2, 0),
                    Matrix3x3::cofactor(&self, 2, 1),
                    Matrix3x3::cofactor(&self, 2, 2),
                ),
            );
            let cofactor_matrix_t = cofactor_matrix.transpose();
            let result = Matrix3x3(
                (
                    cofactor_matrix_t.0 .0 / det,
                    cofactor_matrix_t.0 .1 / det,
                    cofactor_matrix_t.0 .2 / det,
                ),
                (
                    cofactor_matrix_t.1 .0 / det,
                    cofactor_matrix_t.1 .1 / det,
                    cofactor_matrix_t.1 .2 / det,
                ),
                (
                    cofactor_matrix_t.2 .0 / det,
                    cofactor_matrix_t.2 .1 / det,
                    cofactor_matrix_t.2 .2 / det,
                ),
            );
            Some(result)
        }
    }
}

//Matrix3x3 * Matrix3x3
impl Mul for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, other: Matrix3x3) -> Matrix3x3 {
        Matrix3x3(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0) + (self.0 .2 * other.2 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1) + (self.0 .2 * other.2 .1),
                (self.0 .0 * other.0 .2) + (self.0 .1 * other.1 .2) + (self.0 .2 * other.2 .2),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0) + (self.1 .2 * other.2 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1) + (self.1 .2 * other.2 .1),
                (self.1 .0 * other.0 .2) + (self.1 .1 * other.1 .2) + (self.1 .2 * other.2 .2),
            ),
            (
                (self.2 .0 * other.0 .0) + (self.2 .1 * other.1 .0) + (self.2 .2 * other.2 .0),
                (self.2 .0 * other.0 .1) + (self.2 .1 * other.1 .1) + (self.2 .2 * other.2 .1),
                (self.2 .0 * other.0 .2) + (self.2 .1 * other.1 .2) + (self.2 .2 * other.2 .2),
            ),
        )
    }
}
//&Matrix3x3 * &Matrix3x3
impl<'a, 'b> Mul<&'b Matrix3x3> for &'a Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, other: &'b Matrix3x3) -> Matrix3x3 {
        Matrix3x3(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0) + (self.0 .2 * other.2 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1) + (self.0 .2 * other.2 .1),
                (self.0 .0 * other.0 .2) + (self.0 .1 * other.1 .2) + (self.0 .2 * other.2 .2),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0) + (self.1 .2 * other.2 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1) + (self.1 .2 * other.2 .1),
                (self.1 .0 * other.0 .2) + (self.1 .1 * other.1 .2) + (self.1 .2 * other.2 .2),
            ),
            (
                (self.2 .0 * other.0 .0) + (self.2 .1 * other.1 .0) + (self.2 .2 * other.2 .0),
                (self.2 .0 * other.0 .1) + (self.2 .1 * other.1 .1) + (self.2 .2 * other.2 .1),
                (self.2 .0 * other.0 .2) + (self.2 .1 * other.1 .2) + (self.2 .2 * other.2 .2),
            ),
        )
    }
}
//&Matrix3x3 * Matrix3x3
impl<'a> Mul<Matrix3x3> for &'a Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, other: Matrix3x3) -> Matrix3x3 {
        Matrix3x3(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0) + (self.0 .2 * other.2 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1) + (self.0 .2 * other.2 .1),
                (self.0 .0 * other.0 .2) + (self.0 .1 * other.1 .2) + (self.0 .2 * other.2 .2),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0) + (self.1 .2 * other.2 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1) + (self.1 .2 * other.2 .1),
                (self.1 .0 * other.0 .2) + (self.1 .1 * other.1 .2) + (self.1 .2 * other.2 .2),
            ),
            (
                (self.2 .0 * other.0 .0) + (self.2 .1 * other.1 .0) + (self.2 .2 * other.2 .0),
                (self.2 .0 * other.0 .1) + (self.2 .1 * other.1 .1) + (self.2 .2 * other.2 .1),
                (self.2 .0 * other.0 .2) + (self.2 .1 * other.1 .2) + (self.2 .2 * other.2 .2),
            ),
        )
    }
}
//&Matrix3x3 * &Matrix3x3
impl<'a> Mul<&'a Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;
    fn mul(self, other: &'a Matrix3x3) -> Matrix3x3 {
        Matrix3x3(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0) + (self.0 .2 * other.2 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1) + (self.0 .2 * other.2 .1),
                (self.0 .0 * other.0 .2) + (self.0 .1 * other.1 .2) + (self.0 .2 * other.2 .2),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0) + (self.1 .2 * other.2 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1) + (self.1 .2 * other.2 .1),
                (self.1 .0 * other.0 .2) + (self.1 .1 * other.1 .2) + (self.1 .2 * other.2 .2),
            ),
            (
                (self.2 .0 * other.0 .0) + (self.2 .1 * other.1 .0) + (self.2 .2 * other.2 .0),
                (self.2 .0 * other.0 .1) + (self.2 .1 * other.1 .1) + (self.2 .2 * other.2 .1),
                (self.2 .0 * other.0 .2) + (self.2 .1 * other.1 .2) + (self.2 .2 * other.2 .2),
            ),
        )
    }
}

//Matrix2x2 is composed of nested tuples
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix2x2(pub (f32, f32), pub (f32, f32));

impl Matrix2x2 {
    //Creates a new Matrix2x2
    pub fn new(row1: (f32, f32), row2: (f32, f32)) -> Matrix2x2 {
        Matrix2x2(row1, row2)
    }

    //Creates an instance of the 2x2 identity Matrix
    pub fn identity() -> Matrix2x2 {
        Matrix2x2::new((1.0, 0.0), (0.0, 1.0))
    }

    //Transposes a Matrix2x2
    pub fn transpose(&self) -> Matrix2x2 {
        Matrix2x2((self.0 .0, self.1 .0), (self.0 .1, self.1 .1))
    }

    //Finds the determinant of a Matrix2x2
    pub fn determinant(matrix: Matrix2x2) -> f32 {
        (matrix.0 .0 * matrix.1 .1) - (matrix.1 .0 * matrix.0 .1)
    }
}

//Matrix2x2 * Matrix2x2
impl Mul for Matrix2x2 {
    type Output = Matrix2x2;
    fn mul(self, other: Matrix2x2) -> Matrix2x2 {
        Matrix2x2(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1),
            ),
        )
    }
}
//&Matrix2x2 * &Matrix2x2
impl<'a, 'b> Mul<&'b Matrix2x2> for &'a Matrix2x2 {
    type Output = Matrix2x2;
    fn mul(self, other: &'b Matrix2x2) -> Matrix2x2 {
        Matrix2x2(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1),
            ),
        )
    }
}
//&Matrix2x2 * Matrix2x2
impl<'a> Mul<Matrix2x2> for &'a Matrix2x2 {
    type Output = Matrix2x2;
    fn mul(self, other: Matrix2x2) -> Matrix2x2 {
        Matrix2x2(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1),
            ),
        )
    }
}
//&Matrix2x2 * &Matrix2x2
impl<'a> Mul<&'a Matrix2x2> for Matrix2x2 {
    type Output = Matrix2x2;
    fn mul(self, other: &'a Matrix2x2) -> Matrix2x2 {
        Matrix2x2(
            (
                (self.0 .0 * other.0 .0) + (self.0 .1 * other.1 .0),
                (self.0 .0 * other.0 .1) + (self.0 .1 * other.1 .1),
            ),
            (
                (self.1 .0 * other.0 .0) + (self.1 .1 * other.1 .0),
                (self.1 .0 * other.0 .1) + (self.1 .1 * other.1 .1),
            ),
        )
    }
}
