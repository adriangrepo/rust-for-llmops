/*
This is a calculator script. It has the following features:
* Addition
* Subtraction
* Multiplication
* Division

*/

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn divide(a: i32, b: i32) -> i32 {
    a / b
}

//test code here with unit test macro
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
    #[test]
    fn test_subtract() {
        assert_eq!(subtract(3, 2), 1);
    }
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }
    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 5), 2);
    }
}

fn main() {
    let a = 10;
    let b = 5;

    let sum = add(a, b);
    let difference = subtract(a, b);
    let product = multiply(a, b);
    let quotient = divide(a, b);

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
}
