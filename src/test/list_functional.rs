use List::*;

enum List {
    Cons(u32, Box<List>), Nil,
}

impl List {

    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.to_string())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

pub fn test_list_functional() {
    println!("test_list_functional");

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("list length: {}", list.len());
    println!("{}", list.to_string());
}
