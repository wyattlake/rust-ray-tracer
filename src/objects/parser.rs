use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::core::vector::Vec4;
use crate::objects::triangle::Triangle;
use crate::objects::smooth_triangle::SmoothTriangle;
use crate::objects::group::Group;
use crate::objects::object::*;
use crate::materials::material::*;

pub struct Parser {
    pub vertices: Vec<Vec4>,
    pub normals: Vec<Vec4>,
    pub triangles: Vec<Triangle>,
    pub smooth_triangles: Vec<SmoothTriangle>,
}

impl Parser {
    pub fn convert_to_group(self, group: &mut Group) {
        for triangle in self.triangles {
            triangle.add_to_group(group);
        }
        for smooth_triangle in self.smooth_triangles {
            smooth_triangle.add_to_group(group);
        }
    }

    pub fn parse_obj(file: File) -> Parser {
        let file = BufReader::new(file);
        let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
        let mut vertices: Vec<Vec4> = vec![];
        let mut triangles: Vec<Triangle> = vec![];
        let mut normals: Vec<Vec4> = vec![];
        let mut smooth_triangles: Vec<SmoothTriangle> = vec![];

        let mut min_x = f32::INFINITY;
        let mut max_x = -f32::INFINITY;

        let mut min_y = f32::INFINITY;
        let mut max_y = -f32::INFINITY;

        let mut min_z = f32::INFINITY;
        let mut max_z = -f32::INFINITY;

        for line in lines {
            if line.chars().nth(0) == None {
                continue
            }
            if line.chars().nth(0).unwrap() == 'v' {
                if line.chars().nth(1).unwrap() == 'n' {
                    let split: Vec<&str> = line.split(" ").collect();
                    let mut starting_index = 1;
                    while split[starting_index] == "" {
                        starting_index += 1;
                    }
                    let x = split[starting_index].parse::<f32>().unwrap();

                    let y = split[starting_index + 1].parse::<f32>().unwrap();

                    let z = split[starting_index + 2].parse::<f32>().unwrap();

                    normals.push(Vec4(x, y, z, 0.0));
                }
                else {
                    let split: Vec<&str> = line.split(" ").collect();
                    let mut starting_index = 1;
                    while split[starting_index] == "" {
                        starting_index += 1;
                    }
                    let x = split[starting_index].parse::<f32>().unwrap();
                    if x > max_x {
                        max_x = x;
                    }
                    else if x < min_x {
                        min_x = x
                    }

                    let y = split[starting_index + 1].parse::<f32>().unwrap();
                    if y > max_y {
                        max_y = y;
                    }
                    else if y < min_y {
                        min_y = y;
                    }

                    let z = split[starting_index + 2].parse::<f32>().unwrap();
                    if z > max_z {
                        max_z = z;
                    }
                    else if z < min_z {
                        min_z = z;
                    }
                    
                    vertices.push(Vec4(x, y, z, 1.0));
                }
            }
            if line.chars().nth(0).unwrap() == 'f' {
                let mut split: Vec<&str> = line.split(" ").collect();
                split.retain(|string| string != &"");
                if split[1].contains("/") {
                    if split.len() > 4 {
                        let mut vertex_indices: Vec<usize> = vec![];
                        let mut normal_indices: Vec<usize> = vec![];
                        for index in 1..split.len() {
                            let subsplit: Vec<&str> = split[index].split("/").collect();
                            let v = subsplit[0].parse::<usize>().unwrap() - 1;
                            let vn = subsplit[subsplit.len() - 1].parse::<usize>().unwrap() - 1;
                            vertex_indices.push(v);
                            normal_indices.push(vn);
                        }
                        let polygon_triangles = Parser::fan_triangulation_smooth(&vertices, &normals, vertex_indices, normal_indices); 
                        for triangle in polygon_triangles {
                            smooth_triangles.push(triangle);
                        } 
                    }
                    else {
                        let subsplit1: Vec<&str> = split[1].split("/").collect();
                        let v1 = subsplit1[0].replace("-", "").parse::<usize>().unwrap() - 1;
                        let vn1 = subsplit1[subsplit1.len() - 1].replace("-", "").parse::<usize>().unwrap() - 1;

                        let subsplit2: Vec<&str> = split[2].split("/").collect();
                        let v2 = subsplit2[0].replace("-", "").parse::<usize>().unwrap() - 1;
                        let vn2 = subsplit2[subsplit2.len() - 1].replace("-", "").parse::<usize>().unwrap() - 1;

                        let subsplit3: Vec<&str> = split[3].split("/").collect();
                        let v3 = subsplit3[0].replace("-", "").parse::<usize>().unwrap() - 1;
                        let vn3 = subsplit3[subsplit3.len() - 1].replace("-", "").parse::<usize>().unwrap() - 1;

                        smooth_triangles.push(SmoothTriangle::new(vertices[v1].clone(), vertices[v2].clone(), vertices[v3].clone(), normals[vn1].clone(), normals[vn2].clone(), normals[vn3].clone(), Material::default()));
                    } 
                }
                else {
                    if split.len() > 4 {
                        let indices: Vec<usize> = split[1..].iter().map(|index| index.parse::<usize>().unwrap() - 1).collect();
                        let polygon_triangles = Parser::fan_triangulation(&vertices, indices);
                        for triangle in polygon_triangles {
                            triangles.push(triangle);
                        }
                    }
                    else {
                        let v1 = split[1].parse::<usize>().unwrap() - 1;
                        let v2 = split[2].parse::<usize>().unwrap() - 1;
                        let v3 = split[3].parse::<usize>().unwrap() - 1;
                        triangles.push(Triangle::new(vertices[v1].clone(), vertices[v2].clone(), vertices[v3].clone(), Material::default()));
                    }
                }
            } 
            else {
                continue
            }
        }
        println!("Dimensions: ");
        println!("min x: {}, max x: {}", min_x, max_x);
        println!("min y: {}, max y: {}", min_y, max_y);
        println!("min z: {}, max z: {}", min_z, max_z);
        Parser {
            vertices,
            normals,
            triangles,
            smooth_triangles,
        }
    }

    fn fan_triangulation(vertices: &Vec<Vec4>, indices: Vec<usize>) -> Vec<Triangle> {
        let mut triangles = vec![];
        for index in 1..(indices.len() - 1) {
            triangles.push(Triangle::new(vertices[indices[0]].clone(), vertices[indices[index]].clone(), vertices[indices[index + 1]].clone(), Material::default()));
        }
        triangles
    }

    fn fan_triangulation_smooth(vertices: &Vec<Vec4>, normals: &Vec<Vec4>, vertex_indices: Vec<usize>, normal_indices: Vec<usize>) -> Vec<SmoothTriangle> {
        let mut triangles = vec![];
        for index in 1..(vertex_indices.len() - 1) {
            triangles.push(SmoothTriangle::new(vertices[vertex_indices[0]].clone(), vertices[vertex_indices[index]].clone(), vertices[vertex_indices[index + 1]].clone(), normals[normal_indices[0]].clone(), normals[normal_indices[index]].clone(), normals[normal_indices[index + 1]].clone(), Material::default()));
        }
        triangles
    }
}