//
// ODE: Euler Method
// sol: e = 2.71828_18284_59045_23536_02874_71352 ...
//

mod tools;

fn main () {
    let t_lim   = 1.0;
    let delta_t = 1.0e-6;
    let mut t   = 0.0;
    let mut sol = 1.0;

    while t < t_lim {
        sol = sol + delta_t * tools::my_functions::my_identity(sol);
        t += delta_t;
    }
    println!("{} : {}", t, sol);
}