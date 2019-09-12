

fn main() {
    let argument = std::env::args().nth(1)
            .and_then(|arg| arg.parse::<i64>().ok())
            .unwrap_or_else(|| {
                println!("must supply an integer argument");
                std::process::exit(1);
            });

    let result = testing_rust::factorial(argument);
    println!("{}", result);
}
