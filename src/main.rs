enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

fn check_number(number: &str) -> Option<i64> {
    let n = number.parse::<i64>();
    match n {
        Ok(_) => Some(n.unwrap()),
        Err(e) => {
            println!("Could not parse number: \"{}\"", number);
            println!("{}", e);
            None
        }
    }
}

fn check_operation(operation: &str) -> Option<Operation> {
    match operation {
        "+" => Some(Operation::Add),
        "-" => Some(Operation::Sub),
        "*" => Some(Operation::Mul),
        "/" => Some(Operation::Div),
        _ => {
            println!("Unsupported arithmetic operation!");
            print_supported_operations();
            None
        }
    }
}

fn print_supported_operations() -> () {
    println!("Supported operations: + - * /")
}

fn print_help() -> () {
    println!("Press \"q\" to exit");
    println!("Insert arithmetic operation, like 1 + 1");
    print_supported_operations();
}

fn main() {
    use std::io::{stdin, stdout, Write};
    loop {
        let mut line = String::new();
        print!("input> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        line = line.trim_end().to_string();
        match line.as_str() {
            "q" => {
                println!("Good bye!");
                break;
            }
            _ => ()
        }
        let v: Vec<&str> = line.split(" ").collect();
        if v.len() != 3 {
            print_help();
        } else {
            if let (Some(n1), Some(n2), Some(op)) = (check_number(v[0]), check_number(v[2]), check_operation(v[1])) {
                match op {
                    Operation::Add => println!("{}", n1 + n2),
                    Operation::Sub => println!("{}", n1 - n2),
                    Operation::Mul => println!("{}", n1 * n2),
                    Operation::Div => println!("{}", n1 / n2),
                }
            }
        }
    }
}
