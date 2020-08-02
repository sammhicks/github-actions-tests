fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pass() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_bad_add() {
        assert_eq!(1 + 2, 4);
    }
}
