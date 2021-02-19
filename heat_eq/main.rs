//
// 1dim Heat Equation.
// u_t = u_xx
//

// use num_traits
extern crate num_traits;
use num_traits::{Float, FromPrimitive};

pub struct HeatSol<T: Float + FromPrimitive> {
    pub n_grids: usize,
    pub domain:  T,
    pub delta_t: T,
    pub time:    T,
    pub u1:      Vec<T>,
    pub u2:      Vec<T>,
}

impl<T: Float + FromPrimitive> HeatSol<T> {
    //
    // constructor zero initialization
    //
    pub fn new(n: usize, interval: T, dt: T) -> HeatSol<T> {
        HeatSol {
            n_grids: n,
            domain:  interval,
            delta_t: dt,
            time:    num_traits::zero(),
            u1:      vec![num_traits::zero(); n],
            u2:      vec![num_traits::zero(); n],
        }
    }
    // set initial condition
    //pub fn set_init_cond(&mut self) {
    //    self.delta_t = 1.0
    //}
}

pub struct InitialCond<T: Float + FromPrimitive> {
    pub init_func: fn(T) -> T,
}

fn main() {
    // settings
    let n_x      = 100;
    let interval_x = 1.0;
    let dt         = 1.0e-3;
    let heat = HeatSol::<f64>::new(n_x, interval_x, dt);

    println!("{}", heat.u1[99])
}