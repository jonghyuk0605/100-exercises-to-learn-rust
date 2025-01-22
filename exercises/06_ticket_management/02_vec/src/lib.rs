static mut f: Vec<u32> = Vec::new();

pub fn fibonacci(n: u32) -> u32 {
    unsafe {
        if f.len() == 0 {
            f.push(0);
            f.push(1);
        }
        while f.len() <= n as usize {
            f.push(f[f.len() - 2] + f[f.len() - 1])
        }
        return f[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
