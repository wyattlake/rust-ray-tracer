#[cfg(test)]

mod tests {
    use rust_ray_tracer::matrix::*;

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
        let matrix1 = Matrix3x3::new((1.0, 2.0, 3.0),
                                     (4.0, 5.0, 6.0),
                                     (7.0, 8.0, 9.0));
        let matrix2 = Matrix3x3::sub_matrix(&matrix1, 1, 2);
        assert_eq!(Matrix2x2::new((1.0, 2.0), (7.0, 8.0)), matrix2);
        let matrix3 = Matrix4x4::new((-2.0, -8.0, 3.0, 5.0),
                                     (-3.0, 1.0, 7.0, 3.0),
                                     (1.0, 2.0, -9.0, 6.0),
                                     (-6.0, 7.0, 7.0, -9.0));
        let matrix4 = Matrix3x3::new((1.0, 7.0, 3.0),
                                     (2.0, -9.0, 6.0),
                                     (7.0, 7.0, -9.0));
        let matrix5 = Matrix4x4::sub_matrix(&matrix3, 0, 0);
        assert_eq!(matrix4, matrix5);
    }

    #[test]
    //Tests matrix cofactors
    fn find_matrix_cofactor() {
        let matrix1 = Matrix3x3::new((3.0, 5.0, 0.0),
                                     (2.0, -1.0, -7.0),
                                     (6.0, -1.0, 5.0));
        let result1 = Matrix3x3::cofactor(&matrix1, 1, 0);
        assert_eq!(result1, -25.0);
    }

    #[test]
    //Tests matrix determinants
    fn find_determinant() {
        let matrix1 = Matrix3x3::new((1.0, 2.0, 6.0),
                                     (-5.0, 8.0, -4.0),
                                     (2.0, 6.0, 4.0));
        let result1 = Matrix3x3::determinant(&matrix1);
        assert_eq!(result1, -196.0);
        let matrix2 = Matrix4x4::new((-2.0, -8.0, 3.0, 5.0),
                                    (-3.0, 1.0, 7.0, 3.0),
                                    (1.0, 2.0, -9.0, 6.0),
                                    (-6.0, 7.0, 7.0, -9.0));
        let result2 = Matrix4x4::determinant(&matrix2);
        assert_eq!(result2, -4071.0);
    }

    #[test]
    //Tests matrix inverse
    fn matrix_inverse() {
        let matrix1 = Matrix4x4::new((1.0, 1.0, 1.0, -1.0), (1.0, 1.0, -1.0, 1.0), (1.0, -1.0, 1.0, 1.0), (-1.0, 1.0, 1.0, 1.0));
        let result1 = matrix1.inverse();
        assert_eq!(None, result1);
        let matrix3 = Matrix4x4::identity();
        let matrix4 = matrix3.clone().inverse();
        assert_eq!(Some(matrix3), matrix4);
        let matrix5 = Matrix3x3::identity();
        let matrix6 = matrix5.clone().inverse();
        assert_eq!(Some(matrix5), matrix6)
    }
}