fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let (value1, value2, value3) = tup;
    println!("Destructured tuple values: {:?} / {:?} / {:?}", value1, value2, value3);

    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");

    // Arrays

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [3; 5]; // this creates an array of 5 elements with the value 3
    println!("The value of b is: {:?}", b);

    let first = a[0];
    println!("The value of first is: {first}");

    // Statements and expressions

    let statement = {
        let x = 3;
        x + 1
    };
    println!("The value of statement is: {statement}");

    // Functions with return values

    let five_result = five();
    println!("The value of five_result is: {five_result}");

    // Important: if the argument has a different number type than the function signature
    // the code will throw an error.
    let plus_one_result = plus_one(five_result);
    println!("The value of plus_one_result is: {plus_one_result}");

    // If expressions

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}


fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}