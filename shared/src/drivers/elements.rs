#![allow(dead_code)]

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Position{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Vector{
    pub i: f64,
    pub j: f64,
    pub k: f64
}


#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ChrItem{
    pub id: String,
    pub id_type: String,
    pub act: f64,
    pub nom: f64,
    pub utol: f64,
    pub ltol: f64,
    pub f_id: String,
    pub group: String,
    pub groups: String
}




#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Point{
    pub id: String,
    pub id_type: String,
    pub nom_pos: Position,
    pub nom_vec: Vector,
    pub act_pos: Position,
    pub act_vec: Vector,
}

impl Point {
    pub fn from_str(input: &str) -> Point{
        Point { 
            id: "Point1".to_string(), 
            id_type: "Point".to_string(), 
            nom_pos: Position { x: 1.0, y: 2.0, z: 3.0 }, 
            nom_vec: Vector { i: 1.0, j: 0.0, k: 0.0 }, 
            act_pos: Position { x: 1.1, y: 2.1, z: 3.1 },
            act_vec: Vector { i: 0.9, j: 0.1, k: 0.0 } 
        }
    }
}


#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Line{
    id: String
}

impl Line {
    pub fn from_str(input: &str) -> Line{
        Line { id: "hello".to_string() }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Plane{

}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Circle{
    pub x: f64
}

impl Circle {
    pub fn from_str(input: &str) -> Circle {
        Circle { x: 1.0 }
    }
    
}

pub struct Cylinder{

}


#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum FetData {
    Point(Point),
    Line(Line),
    Circle(Circle)
}
