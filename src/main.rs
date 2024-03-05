use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[2].as_str() {
        "echo" => {
            if args.len() != 4 {
                panic!("You wrong.");
            }
            println!("{}", args[3])
        },
        &_ => {
            panic!("Haha!")
        }
    }
}