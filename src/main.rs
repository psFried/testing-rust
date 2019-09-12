

fn main() {
    let argument = std::env::args().nth(1)
            .unwrap_or_else(|| {
                println!("must supply an integer argument");
                std::process::exit(1);
            });

    let result = testing_rust::factorial_of_str(argument.as_bytes());
    match result {
        Ok(num) => println!("{}", num),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}

