use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::core::vector::Vec4;
use crate::objects::triangle::Triangle;
use crate::objects::group::Group;
use crate::objects::object::*;
use crate::materials::material::*;

pub struct Parser {
    pub vertices: Vec<Vec4>,
    pub triangles: Vec<Triangle>,
}

impl Parser {
    pub fn convert_to_group(self, group: &mut Group) {
        for triangle in self.triangles {
            triangle.add_to_group(group);
        }
    }

    pub fn parse_obj(file: File) -> Parser {
        let file = BufReader::new(file);
        let lines: Vec<String> = file.lines().map(|line| line.unwrap()).collect();
        let mut vertices: Vec<Vec4> = vec![];
        let mut triangles: Vec<Triangle> = vec![];
        for line in lines {
            if line.chars().nth(0) == None {
                continue
            }
            if line.chars().nth(0).unwrap() == 'v' {
                let split: Vec<&str> = line.split(" ").collect();
                let x = split[1].parse::<f32>().unwrap();
                let y = split[2].parse::<f32>().unwrap();
                let z = split[3].parse::<f32>().unwrap();
                vertices.push(Vec4(x, y, z, 1.0));
            }
            if line.chars().nth(0).unwrap() == 'f' {
                let split: Vec<&str> = line.split(" ").collect();
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
            else {
                continue
            }
        }
        Parser {
            vertices,
            triangles,
        }
    }

    fn fan_triangulation(vertices: &Vec<Vec4>, indices: Vec<usize>) -> Vec<Triangle> {
        let mut triangles = vec![];
        for index in 1..(indices.len() - 1) {
            triangles.push(Triangle::new(vertices[indices[0]].clone(), vertices[indices[index]].clone(), vertices[indices[index + 1]].clone(), Material::default()));
        }
        triangles
    }
}