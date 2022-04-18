fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 1_000;
    println!("The value of x is {}", x);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum {}, difference {}, product {}, quotient {}, floored {}, remainder {}",
        sum, difference, product, quotient, floored, remainder
    );

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("x {}, y {}, z {}", x, y, z);
    println!("direct access of x, {}", tup.0);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array of fixed length {}", a);
}
