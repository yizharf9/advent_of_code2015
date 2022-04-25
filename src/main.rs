// use std::{fs, primitive};
// mod ex3;
mod ex4;
mod ex6;
use crate::ex6::Aera;
use crate::ex6::Point;
// use crate::ex6::;

#[allow(dead_code)]
fn main() {

    ex6::run();
}




#[allow(dead_code)]
fn test_code() {
    let point_a= Point { x:0, y:0 } ;
    let point_b= Point { x:2, y:1 } ;

    println!("a: {point_a:?}");
    println!("b: {point_b:?}\n");
    let a = Aera::new(point_a,point_b);

    for point in a.points {
        println!("{point:?}")
    }
}




