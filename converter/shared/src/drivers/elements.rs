#![allow(dead_code)]

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Position{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Vector{
    pub i: f64,
    pub j: f64,
    pub k: f64
}


#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ChrData{
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




#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Point{
    pub id: String,
    pub id_type: String,
    pub nom_pos: Position,
    pub nom_vec: Vector,
    pub act_pos: Position,
    pub act_vec: Vector,
}



#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Line{
    id: String
}


#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Plane{

}


pub struct Circle{

}


pub struct Cylinder{

}

