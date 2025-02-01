pub fn sum(v: Vec<i32>) -> i32 {
    let m = v.len() / 2;
    let (mut x, mut y) = (0, 0);

    std::thread::scope(|scope| {
        scope.spawn(|| {
            x += v[..m].iter().sum::<i32>();
        });
        scope.spawn(|| {
            y += v[m..].iter().sum::<i32>();
        });
    });
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
