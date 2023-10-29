#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point{
    x: f32,
    y: f32,
}

struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Top_Left : ({}, {})\n", self.top_left.x, self.top_left.y);
        write!(f, "Bottom_Right : ({}, {})", self.bottom_right.x, self.bottom_right.y)
    }
}

impl Rectangle{
    fn calcArea(&self) -> f32 {
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.bottom_right.y - self.top_left.y;
        width*height
    }
}

fn main(){
    let name = String::from("Kartik");
    let age = 25;
    let kartik = Person{name, age};

    println!("{:?}", kartik);

    let point: Point = Point{x: 10.3, y: 0.4};

    println!("Point Coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point {x: 5.2, y: -0.4};

    println!("Second Point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {x: left_edge, y: right_edge} = point;

    let _rectangle = Rectangle {
        top_left: Point{x: left_edge, y: right_edge},
        bottom_right: bottom_right,
    };

    println!("Rectange Edges : {}", _rectangle);
    let area = _rectangle.calcArea();
    println!("The area of rectangle is {}", area);
}
