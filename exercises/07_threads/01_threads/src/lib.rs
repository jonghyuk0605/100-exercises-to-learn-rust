use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    // Official solution suggests to use split_at function of vector
    let (mut v_first, mut v_second): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());
    for x in v {
        if v_first.len() <= v_second.len() {
            v_first.push(x);
        }
        else {
            v_second.push(x);
        }
    }

    let handle_first:thread::JoinHandle<i32>  = thread::spawn(move || {
        v_first.iter().sum()
    });

    let handle_second:thread::JoinHandle<i32> = thread::spawn(move || {
        v_second.iter().sum()
    });

    handle_first.join().unwrap() + handle_second.join().unwrap()
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
