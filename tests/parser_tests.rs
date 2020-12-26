#[cfg(test)]

mod tests {
    use rust_ray_tracer::objects::parser::Parser;
    use std::fs::File;

    #[test]
    //Tests obj vertex parsing
    fn obj_vertex_parsing() {
        let file = File::open("tests/test1.obj");
        let result = Parser::parse_obj(file.unwrap());
        assert_eq!(result.vertices.len(), 4);
    }

    #[test]
    //Tests obj triangle parsing
    fn obj_triangle_parsing() {
        let file = File::open("tests/test1.obj");
        let result = Parser::parse_obj(file.unwrap());
        assert_eq!(&result.triangles.len(), &2);
        let t1 = result.triangles[0].clone();
        let t2 = result.triangles[1].clone();
        assert_eq!(&t1.p1, &result.vertices[0]);
        assert_eq!(&t1.p2, &result.vertices[1]);
        assert_eq!(&t1.p3, &result.vertices[2]);
        assert_eq!(&t2.p1, &result.vertices[0]);
        assert_eq!(&t2.p2, &result.vertices[2]);
        assert_eq!(&t2.p3, &result.vertices[3]);
    }

    #[test]
    //Tests obj polygon parsing
    fn obj_polygon_parsing() {
        let file = File::open("tests/test2.obj");
        let result = Parser::parse_obj(file.unwrap());
        assert_eq!(&result.triangles.len(), &3);
        let t1 = result.triangles[0].clone();
        let t2 = result.triangles[1].clone();
        let t3 = result.triangles[2].clone();
        assert_eq!(&t1.p1, &result.vertices[0]);
        assert_eq!(&t1.p2, &result.vertices[1]);
        assert_eq!(&t1.p3, &result.vertices[2]);
        assert_eq!(&t2.p1, &result.vertices[0]);
        assert_eq!(&t2.p2, &result.vertices[2]);
        assert_eq!(&t2.p3, &result.vertices[3]);
        assert_eq!(&t3.p1, &result.vertices[0]);
        assert_eq!(&t3.p2, &result.vertices[3]);
        assert_eq!(&t3.p3, &result.vertices[4]);
    }

    #[test]
    fn smooth_obj_parsing() {
        let file = File::open("tests/test3.obj");
        let result = Parser::parse_obj(file.unwrap());
        assert_eq!(&result.smooth_triangles.len(), &2);
        let t1 = result.smooth_triangles[0].clone();
        let t2 = result.smooth_triangles[1].clone();
        assert_eq!(&t1.p1, &result.vertices[0]);
        assert_eq!(&t1.p2, &result.vertices[1]);
        assert_eq!(&t1.p3, &result.vertices[2]);
        assert_eq!(&t1.n1, &result.normals[2]);
        assert_eq!(&t1.n2, &result.normals[0]);
        assert_eq!(&t1.n3, &result.normals[1]);
        assert_eq!(t1, t2);
    }
}