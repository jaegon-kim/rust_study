#![allow(unused)]


// Modules - Visibility 
// https://doc.rust-lang.org/rust-by-example/mod/visibility.html

mod my_mod {
    fn private_function() {
        println!("called test::modules::my_mod::private_function()");
    }

    pub fn pub_function() {
        println!("called test::modules::my_mod::pub_function()");
        private_function();
    }

    // public netsted module
    pub mod nested {

        pub fn function() {
            println!("called `test::modules::my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `test::modules::my_mod::nested::private_function()`");
        }

        pub(in crate::test::modules::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();

    }

    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // private nested module
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}


fn test_visibility() {
    println!("* test_visibility");
    function();
    my_mod::pub_function();
    //test::modules::my_mod::pub_function();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();
    // error - below is a private function
    //my_mod::nested::public_function_in_my_mod();
    // error - below is a private function
    //my_mod::private_function();
    // errors - belows are a private module access
    //my_mod::private_nested::function();
    //my_mod::private_nested::restricted_function();

}

// Module - Struct Visibility
// https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html

mod my {
    pub struct OpenBox<T> {
        pub contents: T, // public field of generic type 'T'
    }

    pub struct ClosedBox<T> {
        contents: T, // private field of generic type 'T'
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn test_struct_visibility() {
    println!("* test_struct_visibility");

    let open_box = my::OpenBox {
        contents: "public information"
    };

    println!("The open box contains: {}", open_box.contents);
    
    // error! ClosedBox.contents is a private field
    //let closed_box = my::ClosedBox { 
    //    contents: "classified information" 
    //};

    let _closed_box = my::ClosedBox::new("classified information");
    // error _closed_box.contents is a private field
    //println!("The closed box contains: {}", _closed_box.contents);

}


pub fn test_modules() {
    test_visibility();
    test_struct_visibility();
}