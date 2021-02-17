//
// 1dim Heat Equation.
// u_t = u_xx
//
extern crate num_traits;
use num_traits::{Float, FromPrimitive};

pub struct Sol<T: Float + FromPrimitive> {
    pub u1: Vec<T>,
    pub u2: Vec<T>,
}

impl<T:Float + FromPrimitive> Sol<T> {
    pub fn new(n: usize) -> Sol<T> {
        Sol{
            u1: vec![num_traits::zero(); n],
            u2: vec![num_traits::zero(); n],
        }
    }
}

fn main() {
    let dim: usize = 100;
    let u = Sol::<f64>::new(dim);

    println!("{}", u.u1[99])
}