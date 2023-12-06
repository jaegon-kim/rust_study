#![allow(unused)]

// Default argument parsing
fn get_arguments_default() {
    let args: Vec<_> = std::env::args().collect();
    println!("args(default): {:?}", args);

    for arg in args {
        println!(" - {}", arg);
    }
}


use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    // Gives name
    #[arg(short, long, default_value_t = String::from("Default Name"))]
    name: String,
    // Gives age
    #[arg(short, long, default_value_t = 1)]
    age: usize,
}

fn get_arguments_with_clap() {
    let args = Args::parse();
    println!("args(clap): {:?}", args);
    println!("         - name: {}", args.name);
    println!("         - age: {}", args.age);   
}

pub fn test_argument() {
    println!("test_argument");

    get_arguments_default();
    get_arguments_with_clap();

}