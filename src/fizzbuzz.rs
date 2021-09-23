pub fn fizzbuzz(number: i32) -> i32 {
    // Implement fizzbuzz  in Rust
    //  - if number is divisible by 3, print "fizz"
    //  - if number is divisible by 5, print "buzz"
    //  - if number is divisible by 3 and 5, print "fizzbuzz"
    //  - otherwise, print the number
    //
    // Example:
    //  - fizzbuzz(1) -> 1
    //  - fizzbuzz(2) -> 2
    //  - fizzbuzz(3) -> "fizz"
    //  - fizzbuzz(5) -> "buzz"
    //  - fizzbuzz(15) -> "fizzbuzz"
    //  - fizzbuzz(30) -> "fizzbuzz"
    //  - fizzbuzz(7) -> 7

    1

}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::fizzbuzz;

    #[test]
    fn fizzbuzz_1() {
        let result = fizzbuzz(1);
        assert_eq!(result, 1);
    }
}