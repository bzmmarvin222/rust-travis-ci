#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_four_returns_4() {
        assert_eq!(get_four(), 4);
    }
}

fn main() {
    println!("this program does not serve any functionality, it's just about the tests.");
}

fn get_four() -> i32 {
    4
}