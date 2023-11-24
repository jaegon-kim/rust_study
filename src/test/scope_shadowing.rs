

fn test1() {
    let long_lived_binding =  1;
    {
        let _short_lived_binding = 2;        
    }
    println!("long_lived_binding: {}", long_lived_binding);
    //println!("_short_lived_binding: {}", _short_lived_binding);
}

fn test2() {
    let shadowed_binding = 1;
    {
        println!("before being shadowed: {}", shadowed_binding); 
        let shadowed_binding = "abc";
        println!("after being shadowed: {}", shadowed_binding); 
    }
    println!("out side inner block: {}", shadowed_binding);
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

}

pub fn test_scope_shadowing() {
    println!("test_scope_shadowing");
    test1();
    test2();
}
