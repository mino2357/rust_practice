//
// floating point number test.
// not recommended for using the "assert_eq!" ...
// using approx("assert_approx_eq" ...) is better for numerical calculus.
//

#[cfg_attr(test, macro_use)] 
extern crate approx;

mod tools;

#[cfg(test)]
pub mod tests {
    use crate::tools;

    #[test]
    #[allow(unused_must_use)]
    pub fn test_identity_map_abs() {
        abs_diff_eq!(tools::my_functions::my_identity(12.1), 12.1, epsilon = std::f64::EPSILON);
    }
    #[test]
    #[allow(unused_must_use)]
    pub fn test_identity_map_rel() {
        relative_eq!(tools::my_functions::my_identity(12.1), 12.1, epsilon = std::f64::EPSILON);
    }
}