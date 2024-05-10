use std::io::{self};

fn read_number(prompt: &str) -> i64 {
    let mut number = String::new();
    loop {
        println!("{}", prompt);
        number.clear();
        match io::stdin().read_line(&mut number) {
            Ok(_) => match number.trim().parse::<i64>() {
                Ok(x) => {
                    return x;
                }
                Err(error) => {
                    println!("Pls enter valid number: {}", error)
                }
            },
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn read_operator(prompt: &str) -> char {
    let mut operator = String::new();
    loop {
        println!("{}", prompt);
        operator.clear();
        match io::stdin().read_line(&mut operator) {
            Ok(_) => match operator.trim() {
                "+" | "-" | "*" | "/" | "%" => {
                    return operator.trim().chars().next().unwrap_or_default();
                }
                _ => {
                    println!("Invalid operator");
                }
            },
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}
fn main() {
    let number1 = read_number("Enter a number: ");
    let operator = read_operator("Enter operator (+, -, *, /, %): ");
    let number2 = read_number("Enter another number: ");

    match operator {
        '+' => {
            match number1.checked_add(number2) {
                Some(x) => {
                    println!("The sum of {} and {} is {}", number1, number2, x);
                }
                None => {
                    println!("Overflow occurred");
                }
            };
        }
        '-' => {
            match number1.checked_sub(number2) {
                Some(x) => {
                    println!("The subtraction of {} and {} is {}", number1, number2, x);
                }
                None => {
                    println!("Overflow occurred");
                }
            };
        }
        '*' => {
            match number1.checked_mul(number2) {
                Some(x) => {
                    println!("The multiplication of {} and {} is {}", number1, number2, x);
                }
                None => {
                    println!("Overflow occurred");
                }
            };
        }
        '/' => {
            match number1.checked_div(number2) {
                Some(x) => {
                    println!("The division of {} and {} is {}", number1, number2, x);
                }
                None => {
                    println!("Overflow occurred Or Division by Zero");
                }
            };
        }
        '%' => {
            match number1.checked_rem(number2) {
                Some(x) => {
                    println!("The remainder of {} and {} is {}", number1, number2, x);
                }
                None => {
                    println!("Overflow occurred Or Division by Zero");
                }
            };
        }
        _ => {
            println!("Invalid operator");
        }
    }
}
