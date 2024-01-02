#![allow(unused)]

//#[cfg(panic = "abort")]
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("Aaaaaa!!!!!");
    }

    if beverage == "apple" {
        if cfg!(panic="abort") {
            println!("This is not your party");
        } else {
            println!("Split it");
        }
    }

    println!("Refreshing {}", beverage);
}



fn test_panic() {
    drink("water");
    //drink("lemonade");
    drink("apple");
    drink("still water");
}

pub fn test_error_handle() {
    println!("test_error_handle");
    test_panic();
}