#[derive(PartialEq, Debug)]
pub enum NumOrString {
    Num(i32), 
    String(String)
}

pub fn fizzbuzz(number: i32) -> NumOrString {
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

    NumOrString::Num(number)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_1() {
        let result = fizzbuzz(1);
        let expectation = NumOrString::Num(1);
        assert_eq!(result, expectation);
    }

    #[test]
    fn fizzbuzz_2() {
        let result = fizzbuzz(2);
        let expectation = NumOrString::Num(2);
        assert_eq!(result, expectation);
    }
    
    // #[test]
    // fn fizzbuzz_3() {
    //     let result = fizzbuzz(3);
    //     assert_eq!(result, "fizz");
    // }
}