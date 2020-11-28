#[cfg(test)]

mod tests {
    use rust_ray_tracer::core::matrix::*;
    use rust_ray_tracer::core::vector::*;
    use rust_ray_tracer::misc::axis::Axis;

    //Tests matrix indexing
    #[test]
    fn indexing() {
        let matrix = Matrix4x4::new((1.0, 2.0, 3.0, 4.0), (5.0, 6.0, 7.0, 8.0), (9.0, 10.0, 11.0, 12.0), (13.0, 14.0, 15.0, 16.0));
        assert_eq!(matrix.1.2, 7.0);
    }

    //Tests multiplying matrices and the identity matrix
    #[test]
    fn multiplication() {
        let matrix1 = Matrix4x4::new((1.0, 2.0, 3.0, 4.0), (5.0, 6.0, 7.0, 8.0), (9.0, 10.0, 11.0, 12.0), (13.0, 14.0, 15.0, 16.0));
        let identity1 = Matrix4x4::identity();
        let result1 = matrix1.clone() * identity1;
        assert_eq!(&matrix1, &result1);
        let matrix2 = Matrix3x3::new((1.0, 2.0, 3.0), (4.0, 5.0, 6.0), (7.0, 8.0, 9.0));
        let identity2 = Matrix3x3::identity();
        let result2 = matrix2.clone() * identity2;
        assert_eq!(&matrix2, &result2);
        let matrix3 = Matrix2x2::new((1.0, 2.0), (3.0, 4.0));
        let identity3 = Matrix2x2::identity();
        let result3 = matrix3.clone() * identity3;
        assert_eq!(&matrix3, &result3);
    }

    //Tests transposing matrices
    #[test]
    fn transposition() {
        let matrix1 = Matrix4x4::new((0.0, 9.0, 3.0, 0.0), (9.0, 8.0, 0.0, 8.0), (1.0, 8.0, 5.0, 3.0), (0.0, 0.0, 5.0, 8.0));
        let matrix2 = Matrix4x4::new((0.0, 9.0, 1.0, 0.0), (9.0, 8.0, 8.0, 0.0), (3.0, 0.0, 5.0, 5.0), (0.0, 8.0, 3.0, 8.0));
        assert_eq!(matrix2, matrix1.transpose());
        let matrix3 = Matrix3x3::new((0.0, 9.0, 3.0), (9.0, 8.0, 0.0), (1.0, 8.0, 5.0));
        let matrix4 = Matrix3x3::new((0.0, 9.0, 1.0), (9.0, 8.0, 8.0), (3.0, 0.0, 5.0));
        assert_eq!(matrix3, matrix4.transpose());
        let matrix5 = Matrix2x2::new((1.0, 2.0), (3.0, 4.0));
        let matrix6 = Matrix2x2::new((1.0, 3.0), (2.0, 4.0));
        assert_eq!(matrix5, matrix6.transpose());
    }

    //Tests sub-matrices
    #[test]
    fn find_sub_matrices() {
        let matrix1 = Matrix3x3::new((1.0, 2.0, 3.0), (4.0, 5.0, 6.0), (7.0, 8.0, 9.0));
        let matrix2 = Matrix3x3::sub_matrix(&matrix1, 1, 2);
        assert_eq!(Matrix2x2::new((1.0, 2.0), (7.0, 8.0)), matrix2);
        let matrix3 = Matrix4x4::new((-2.0, -8.0, 3.0, 5.0), (-3.0, 1.0, 7.0, 3.0), (1.0, 2.0, -9.0, 6.0), (-6.0, 7.0, 7.0, -9.0));
        let matrix4 = Matrix3x3::new((1.0, 7.0, 3.0), (2.0, -9.0, 6.0), (7.0, 7.0, -9.0));
        let matrix5 = Matrix4x4::sub_matrix(&matrix3, 0, 0);
        assert_eq!(matrix4, matrix5);
    }

    #[test]
    //Tests matrix cofactors
    fn find_matrix_cofactor() {
        let matrix1 = Matrix3x3::new((3.0, 5.0, 0.0), (2.0, -1.0, -7.0), (6.0, -1.0, 5.0));
        let result1 = Matrix3x3::cofactor(&matrix1, 1, 0);
        assert_eq!(result1, -25.0);
    }

    #[test]
    //Tests matrix determinants
    fn find_determinant() {
        let matrix1 = Matrix3x3::new((1.0, 2.0, 6.0), (-5.0, 8.0, -4.0), (2.0, 6.0, 4.0));
        let result1 = Matrix3x3::determinant(&matrix1);
        assert_eq!(result1, -196.0);
        let matrix2 = Matrix4x4::new((-2.0, -8.0, 3.0, 5.0), (-3.0, 1.0, 7.0, 3.0), (1.0, 2.0, -9.0, 6.0), (-6.0, 7.0, 7.0, -9.0));
        let result2 = Matrix4x4::determinant(&matrix2);
        assert_eq!(result2, -4071.0);
    }

    #[test]
    //Tests matrix inverse
    fn matrix_inverse() {
        let matrix1 = Matrix4x4::new((1.0, 0.0, 0.0, 5.0), (0.0, 1.0, 0.0, 0.0), (0.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 1.0));
        let result1 = matrix1.inverse();
        let matrix2 = Matrix4x4::new((1.0, 0.0, 0.0, -5.0), (0.0, 1.0, 0.0, 0.0), (0.0, 0.0, 1.0, 0.0), (0.0, 0.0, 0.0, 1.0));
        assert_eq!(matrix2, result1.unwrap());
        let matrix3 = Matrix4x4::identity();
        let matrix4 = matrix3.clone().inverse();
        assert_eq!(Some(matrix3), matrix4);
        let matrix5 = Matrix3x3::identity();
        let matrix6 = matrix5.clone().inverse();
        assert_eq!(Some(matrix5), matrix6)
    }

    #[test]
    //Tests translation
    fn translation() {
        let vector = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let translation_matrix1 = Matrix4x4::translation(0.0, 1.0, 2.0);
        let result = translation_matrix1 * vector;
        assert_eq!(result, Vec4::new(1.0, 2.0, 3.0, 1.0));
        let vector = Vec4::new(1.0, 1.0, 1.0, 0.0);
        let translation_matrix1 = Matrix4x4::translation(0.0, 1.0, 2.0);
        let result = translation_matrix1 * vector;
        assert_eq!(result, Vec4::new(1.0, 1.0, 1.0, 0.0));
    }

    #[test]
    //Tests scaling
    fn scaling() {
        let point = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let translation_matrix = Matrix4x4::scaling(2.0, 1.0, 5.0);
        let result = translation_matrix * point;
        assert_eq!(result, Vec4::new(2.0, 1.0, 5.0, 1.0));
        let vector = Vec4::new(1.0, 1.0, 1.0, 0.0);
        let translation_matrix2 = (Matrix4x4::scaling(2.0, 1.0, 5.0)).inverse().unwrap();
        let result2 = translation_matrix2 * vector;
        assert_eq!(result2, Vec4::new(0.5, 1.0, 0.2, 0.0));
    }

    #[test]
    //Tests reflection
    fn reflection() {
        let mut point = Vec4::new(1.0, 1.0, 1.0, 1.0);
        point = Matrix4x4::reflection(Axis::Y) * point;
        assert_eq!(point, Vec4::new(1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    //Tests rotation
    fn rotation() {
        let point = Vec4(1.0, 1.0, 1.0, 1.0);
        let point2 = Matrix4x4::rotation(Axis::Z, 360.0) * Matrix4x4::rotation(Axis::Y, 360.0) * Matrix4x4::rotation(Axis::X, 360.0) * point.clone();
        //Values are rounded because rotations are affected by rounding errors
        assert_eq!((point.0.round(), point.1.round(), point.2.round()), (point2.0.round(), point2.2.round(), point2.2.round()));
    }

    #[test]
    //Tests shearing
    fn shearing() {
        let mut point1 = Vec4(2.0, 3.0, 4.0, 1.0);
        point1 = Matrix4x4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * point1;
        assert_eq!(Vec4(5.0, 3.0, 4.0, 1.0), point1);
        let mut point2 = Vec4(2.0, 3.0, 4.0, 1.0);
        point2 = Matrix4x4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * point2;
        assert_eq!(Vec4(6.0, 3.0, 4.0, 1.0), point2);
        let mut point3 = Vec4(2.0, 3.0, 4.0, 1.0);
        point3 = Matrix4x4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0) * point3;
        assert_eq!(Vec4(2.0, 5.0, 4.0, 1.0), point3);
        let mut point4 = Vec4(2.0, 3.0, 4.0, 1.0);
        point4 = Matrix4x4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0) * point4;
        assert_eq!(Vec4(2.0, 7.0, 4.0, 1.0), point4);
        let mut point5 = Vec4(2.0, 3.0, 4.0, 1.0);
        point5 = Matrix4x4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0) * point5;
        assert_eq!(Vec4(2.0, 3.0, 6.0, 1.0), point5);
        let mut point6 = Vec4(2.0, 3.0, 4.0, 1.0);
        point6 = Matrix4x4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0) * point6;
        assert_eq!(Vec4(2.0, 3.0, 7.0, 1.0), point6);
    }
}