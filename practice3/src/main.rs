/// nalgebra

extern crate nalgebra as na;
use na::*;

fn main() {
    println!("Try nalgebra");

    let x = Vector3::new(1, 2, 3);
    let a = Matrix3::new(1, 0, 0,
                         0, 1, 0,
                         0, 0, 1);

    println!("{}", x);
    println!("{}", a);

    let v = a*x;
    println!("{}", v);

    println!("{}", v.transpose());
}
