// The args library allows us read input from the command line
use std::env::args;

fn main() {
    // Store command-line arguments in a variable.
    let mut args = args();

    // Unpacks the arguments using the nth iterator.
    // unwrap() unwraps the value of the iterator and panics the app
    //  in case of an error.
    // Args are separated by a single space. Example: cargo run -- 1 + 2
    let first = args.nth(1).unwrap();
    // Convert the String() to char.
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // Converts the strings to floats
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    // Operates on the numbers
    let result = operate(operator, first_number, second_number);

    println!("{:?}", output(first_number, second_number, operator, result));

}

// Function takes in the numbers and the operators and operates on them
// based on the input. Panics app if the operator is invalid.
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used.")
    }
}

// This takes the inputs and the result from the operator and format
// them to be printed in the terminal.
fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}