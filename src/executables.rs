
// https://doc.rust-lang.org/rust-by-example/crates/using_lib.html

// build library like belows 
// make docker-run CMD="rustc --crate-type=lib src/rary.rs"
// compile
// make docker-run CMD="rustc src/executables.rs --extern rary=library.rlib"
// make docker-run CMD="./executables"

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}