fn calc_fibonacci(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 0..n-1 {
        let temp = a;
        a = b;
        
        if i64::MAX - b <= temp {
            return 0;
        }

        b = temp + b;
    }

    b
}

#[no_mangle]
pub extern "C" fn fibonacci(n: i32) -> i64 {
    calc_fibonacci(n.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fibonacci_0() {
        assert_eq!(calc_fibonacci(0), 0);
    }

    #[test]
    fn test_calc_fibonacci_1() {
        assert_eq!(calc_fibonacci(1), 1);
    }

    #[test]
    fn test_calc_fibonacci_6() {
        assert_eq!(calc_fibonacci(6), 8);
    }

    #[test]
    fn test_calc_fibonacci_10() {
        assert_eq!(calc_fibonacci(10), 55);
    }

    #[test]
    fn test_calc_fibonacci_100() {
        assert_eq!(calc_fibonacci(100), 0);
    }
}
