use std::io;

fn main() {
    let first_number: i32 = read_int();
    let symbol = read_string().trim().to_string();
    let second_number: i32 = read_int();

    println!("{}", first_number);
    println!("{}", symbol);
    println!("{}", second_number);

    if symbol == '+'{
        println!("{}",first_number + second_number)
    } else if symbol == '-'{
        println!("{}", first_number - second_number)
    } else if  symbol == '*'{
        println!("{}", first_number * second_number)
    } else if  symbol == '/'{
        println!("{}", first_number / second_number)
    } else {
        println!("Coś poszło nie tak")
    }

}

fn read_string() -> String{
    let mut input = str::new();
    io::stdin().read_line(&mut input).expect("Can't read user input");

    input
}

fn read_int() -> i32{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input: i32 = input.trim().parse().expect("Unable to transform into integer");

    input
}