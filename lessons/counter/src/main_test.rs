fn main() {
    // This is where the main logic of the counter application will be implemented.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment() {
        let mut counter = 0;
        counter += 1;
        assert_eq!(counter, 1);
    }

    #[test]
    fn test_counter_decrement() {
        let mut counter = 1;
        counter -= 1;
        assert_eq!(counter, 0);
    }
}