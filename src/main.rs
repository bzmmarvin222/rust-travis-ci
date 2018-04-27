#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_three_returns_3() {
        assert_eq!(get_three(), 3);
    }
}

fn main() {
    println!("this program does not serve any functionality, it's just about the tests.");
}

fn get_three() -> i32 {
    3
}