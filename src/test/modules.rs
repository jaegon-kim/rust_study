#![allow(unused)]


mod my_mod {
    fn private_function() {
        println!("called test::modules::my_mod::private_function()");
    }

    pub fn pub_function() {
        println!("called test::modules::my_mod::pub_function()");
        private_function();
    }

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

pub fn test_modules() {
    println!("test_modules");
    function();
    my_mod::pub_function();
    //test::modules::my_mod::pub_function();

    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();

}