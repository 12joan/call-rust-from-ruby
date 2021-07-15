#[no_mangle]
pub extern fn fibonacci(n: u32) -> u32 {
    match n {
        1 => 1,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(1, fibonacci(1));
    }

    #[test]
    fn test_two() {
        assert_eq!(1, fibonacci(2));
    }

    #[test]
    fn test_three() {
        assert_eq!(2, fibonacci(3));
    }

    #[test]
    fn test_four() {
        assert_eq!(3, fibonacci(4));
    }

    #[test]
    fn test_five() {
        assert_eq!(5, fibonacci(5));
    }

    #[test]
    fn test_ten() {
        assert_eq!(55, fibonacci(10));
    }
}
