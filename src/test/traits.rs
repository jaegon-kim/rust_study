#[allow(unused)]
#[allow(dead_code)]

// https://doc.rust-lang.org/rust-by-example/trait.html
// Trait - a collection of methods defeind for a unknown typ: Self

struct Sheep {
    naked: bool,
    name: &'static str,
}

struct Wolf {
    hungry: bool,
    name: &'static str,
}

trait Animal {
    // Associated function signature; 'Self' refers to the implementor type
    fn new(name: &'static str) -> Self;
    // Method signatures
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    #[allow(dead_code)]
    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep {name: name, naked: false}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaah?"
        } else {
            "baaaah!"
        }
    }
    // Default trait methods can be overridden.
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

impl Wolf {
    fn is_hungry(&self) -> bool {
        self.hungry
    }

    #[allow(dead_code)]
    fn eat(&mut self) {
        if self.is_hungry() {
            // Implementor methods can use the implementor's trait methods
            println!("{} is hugry...", self.name());
            self.hungry = false;
        } else {
            println!("{} is full, not hungry", self.name);
            
        }
    }
}

impl Animal for Wolf {
    fn new(name: &'static str) -> Self {
        Wolf {name: name, hungry: true}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_hungry() {
            "Aoooooooh?"
        } else {
            "Aoooooooh!"
        }
    }
    // Default trait methods can be overridden.
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

#[allow(dead_code)]
fn test_traits_basic() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();  // Amimal's method
    dolly.shear(); // Sheep's method
    dolly.talk(); // Amimal's method

    let mut chita: Wolf = Animal::new("Chita");
    chita.talk();  // Amimal's method
    chita.eat(); // Wolf's method
    chita.talk(); // Amimal's method
}

// https://doc.rust-lang.org/rust-by-example/trait/derive.html
// providing basic implementations for some traits via the #[derive] attribute
// - Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// - Clone, to create T from &T via a copy.
// - Copy, to give a type 'copy semantics' instead of 'move semantics'.
// - Hash, to compute a hash from &T.
// - Default, to create an empty instance of a data type.
// - Debug, to format a value using the {:?} formatter.

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    #[allow(dead_code)]
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// no additional attributes
#[allow(dead_code)]
struct Seconds(i32);

#[allow(dead_code)]
fn test_traits_derive() {

    let _one_second = Seconds(1);
    // Error: `Seconds` cannot be formatted using `{:?}`
    //println!("One second looks like: {:?}", _one_second);

    // Error: binary operation `==` cannot be applied to type `Seconds`
    //let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);
    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter", cmp);

}

// Returning Traits with dyn
// https://doc.rust-lang.org/rust-by-example/trait/dyn.html

struct Sheep2 {}

struct Cow2 {}

trait Animal2 {
    fn noise(&self) -> &'static str;
}

impl Animal2 for Sheep2 {
    fn noise(&self) -> &'static str {
        "baaaah!"
    }
}

impl Animal2 for Cow2 {
    fn noise(&self) -> &'static str {
        "mooooo!"
    }
}

#[allow(dead_code)]
fn random_animal(random_number: f64) -> Box<dyn Animal2> {
    if random_number < 0.5 {
        Box::new(Sheep2 {})
    } else {
        Box::new(Cow2 {})
    }
}

#[allow(dead_code)]
fn test_return_traits_with_dyn() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("{}", animal.noise());
}

// Operator overloading
// https://doc.rust-lang.org/rust-by-example/trait/ops.html
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}

impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

#[allow(dead_code)]
fn test_operator_overloading() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

// Drop
// https://doc.rust-lang.org/rust-by-example/trait/drop.html

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    #[allow(dead_code)]
    fn drop(&mut self) {
        println!(" > Dropping {}", self.name);
    }
}

#[allow(dead_code)]
fn test_drop() {
    let _a = Droppable { name: "a" };
    let _e = Droppable { name: "e" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exite block B");

        println!("Exiting block A");        
    }
    println!("Just exite block A");

    drop(_a);
    println!("end of the main function");

}

// Interator 
// https://doc.rust-lang.org/rust-by-example/trait/iter.html

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

#[allow(dead_code)]
fn fibonacci() -> Fibonacci {
    Fibonacci{
        curr: 0,
        next: 1,
    }
}

#[allow(dead_code)]
fn test_iterator() {
    let mut sequence = 0..3;
    println!("Four consecutive 'next' for 0..3");
    println!(" > {:?}", sequence.next());
    println!(" > {:?}", sequence.next());
    println!(" > {:?}", sequence.next());
    println!(" > {:?}", sequence.next());

    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }

}

// impl Trait
// https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html
// impl tarit can be used in 2 location 1. argument typ, 2. return type

#[allow(dead_code)]
fn _parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(
            |line| {
                // For each line in the source
                line.map(
                    |line| {
                        line.split(',')
                            .map(
                                |entry| String::from(entry.trim())
                            )
                            .collect()
                    }
                )
            }
        )
        .collect() // Collect all lines into <Vec<Vec<String>>>
}

// As an argument type
#[allow(dead_code)]
fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(
            |line| {
                // For each line in the source
                line.map(
                    |line| {
                        line.split(',')
                            .map(
                                |entry| String::from(entry.trim())
                            )
                            .collect()
                    }
                )
            }
        )
        .collect() // Collect all lines into <Vec<Vec<String>>>
}

// As a return type
use std::iter;
use std::vec::IntoIter;

// returns iterator - return type is complicated
#[allow(dead_code)]
fn combine_vecs_explicit_return_type(v: Vec<i32>, u: Vec<i32>,)
    -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
        v.into_iter().chain(u.into_iter()).cycle()
}

#[allow(dead_code)]
fn combine_vecs(v: Vec<i32>, u: Vec<i32>,)
    -> impl Iterator<Item=i32> {
        v.into_iter().chain(u.into_iter().cycle())
}

#[allow(dead_code)]
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y};
    closure
}

#[allow(dead_code)]
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

#[allow(dead_code)]
fn test_impl_trait() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);

    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());
    println!("{:?}", v3.next());

    let plus_one = make_adder_function(1);
    println!("plus_one rslt: {:?}", plus_one(2));

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    println!("double_positives: {:?}", doubles.collect::<Vec<i32>>());

}

// https://doc.rust-lang.org/rust-by-example/trait/clone.html

#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)] // No 'Copy'
struct Pair(Box<i32>, Box<i32>);

// Error - the trait `Copy` cannot be implemented for this type
//#[derive(Clone, Debug, Copy)]
//struct Pair2(Box<i32>, Box<i32>);

#[allow(dead_code)]
fn test_clone() {

    let unit = Unit; // instantiate 'unit'
    let copied_unit = unit; // copy (no resource move)

    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair); // instantiate pair

    let moved_pair = pair; // move resources (as no deive 'Copy' for Pair)
    println!("moved_pair: {:?}", moved_pair);
    // error - pair is already moved
    //println!("original: {:?}", pair);

    //let pair_copiable = Pair2(Box::new(3), Box::new(4));
    //println!("original: {:?}", pair_copiable); // instantiate pair
    //let copied_pair = pair_copiable;
    //println!("copied_pair: {:?}", copied_pair);
    //println!("original: {:?}", pair_copiable);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // error - value borrowed here after move
    //println!("dropped moved_pair: {:?}", moved_pair);
    println!("clone: {:?}", cloned_pair);

}

// Supertaits https://doc.rust-lang.org/rust-by-example/trait/supertraits.html
trait Person {
    fn name(&self) -> String;
}

// Person is a super trait of student
trait Student:Person {
    fn university(&self) -> String;    
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

#[allow(dead_code)]
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

#[allow(dead_code)]
fn test_supertraits() {

}

// Disambiguating overlapping traits
// https://doc.rust-lang.org/rust-by-example/trait/disambiguating.html

trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

#[allow(dead_code)]
fn test_disambiguating_overlapping_trait() {
    let form = Form {
        username: "rustocean".to_owned(),
        age: 28,
    };
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustocean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
    println!("{}, {}", username, age);
}

#[allow(dead_code)]
pub fn test_traits() {
    println!("test_traits");
    test_traits_basic();
    test_traits_derive();
    test_return_traits_with_dyn();
    test_operator_overloading();
    test_drop();
    test_iterator();
    test_impl_trait();
    test_clone();
    test_supertraits();
    test_disambiguating_overlapping_trait();
}


