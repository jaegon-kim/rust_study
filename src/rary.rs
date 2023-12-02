
// https://doc.rust-lang.org/rust-by-example/crates/lib.html
// Creating a Library
// 
// build like belows 
// make docker-run CMD="rustc --crate-type=lib src/rary.rs"

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");
    private_function();
}