#[macro_use]
extern crate itertools;
use itertools::Itertools;
use itertools::zip;
use itertools::all;
use itertools::any;
use itertools::rev;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_four_returns_4() {
        assert_eq!(get_four(), 4);
    }

    #[test]
    fn demo_all_false_returns_false(){
        assert!(!demo_all_false());
    }

    #[test]
    fn demo_all_true_returns_true(){
        assert!(demo_all_true());
    }

    #[test]
    fn demo_any_false_returns_false() {
        assert!(!demo_any_false())
    }

    #[test]
    fn demo_any_true_returns_true() {
        assert!(demo_any_true())
    }
}

fn main() {
    println!("this program does not serve any functionality, it's just about the tests.");
    demo_rev();
    demo_zip();
}

fn get_four() -> i32 {
    4
}

fn demo_all_false() -> bool {
    all(&[1, 3, 3, 7], |elt| *elt == 3)
}

fn demo_all_true() -> bool {
    all(&[1, 3, 3, 7], |elt| *elt < 8)
}

fn demo_any_true() -> bool {
    any(&[4, 2], |elt| *elt == 4)
}

fn demo_any_false() -> bool {
    any(&[4, 2], |elt| *elt == 3)
}

fn demo_rev() {
    // -> [3,2,1]
    for elt in rev(&[1,2,3]) {
        println!("Reversed element: {}", elt);
    }
}

fn demo_zip() {
    // -> [(1,3), (5,4), (2,6)]
    for (elt_a, elt_b) in zip(&[1, 5, 2], &[3, 4, 6]) {
        println!("elt a: {}, elt b: {}", elt_a, elt_b);
    }
}