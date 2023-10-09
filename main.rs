use std::io;

fn main() {
    println!("Choose one of the following operations.");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let user_input = get_input("> ");

    match user_input.as_str() {
        "1" => prompt_addition(),
        "2" => prompt_subtraction(),
        "3" => prompt_multiplication(),
        "4" => prompt_division(),
        _ => println!("Invalid choice!"),
    }
}

fn prompt_addition() {
    let x_string = get_input("Type your first number: ");
    let y_string = get_input("Type your second number: ");
    let x = x_string.trim().parse::<i32>().unwrap();
    let y = y_string.trim().parse::<i32>().unwrap();
    let solution = x + y;
    println!("The solution to this problem is {}", solution);
}

fn prompt_subtraction() {
    let x_string = get_input("Type your first number: ");
    let y_string = get_input("Type your second number: ");
    let x = x_string.trim().parse::<i32>().unwrap();
    let y = y_string.trim().parse::<i32>().unwrap();
    let solution = x - y;
    println!("The solution to this problem is {}", solution);
}

fn prompt_multiplication() {
    let x_string = get_input("Type your first number: ");
    let y_string = get_input("Type your second number: ");
    let x = x_string.trim().parse::<i32>().unwrap();
    let y = y_string.trim().parse::<i32>().unwrap();
    let solution = x * y;
    println!("The solution to this problem is {}", solution);
}

fn prompt_division() {
    let x_string = get_input("Type your first number: ");
    let y_string = get_input("Type your second number: ");
    let x = x_string.trim().parse::<i32>().unwrap();
    let y = y_string.trim().parse::<i32>().unwrap();

    if y == 0 {
        println!("Error: Division by zero is not allowed.");
    } else {
        let solution = x / y;
        println!("The solution to this problem is {}", solution);
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
