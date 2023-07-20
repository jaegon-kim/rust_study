#![allow(unused)]

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x:i64, y: i64 }
}

fn inspect(event: WebEvent) {
    /*
    match(event) {
        WebEvent::PageLoad => println!("PageLoad"),
        WebEvent::PageUnload => println!("PageUnload"),
    }
    */

}


pub fn test_enum() {
    println!("test_enum");
}

