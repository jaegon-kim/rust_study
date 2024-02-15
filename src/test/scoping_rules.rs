#![allow(unused)]

// RAII(Resource Acquisition is Initialization)
// https://doc.rust-lang.org/rust-by-example/scope/raii.html

fn create_box() {
    let _box1 = Box::new(3i32);
    // _box1 is destroyed here, and memory gets freed
}

fn test_raii() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
        // _box3 is destroyed here, and memory gets freed
    }

    for _ in 0u32..1_000 {
        create_box();
    }
}

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // 'c' is destroyed and the memory freed
}

fn test_ownership_and_move() {
    
    let x = 5u32; // _Stack_ allocated integer
    let y = x; // No resource move

    // stack에 잡히는 Primitive Type은 ownership이 이전되지 않는다.
    // x가 y로 복사되어 copy가 된다.
    println!("x is {}, and y is {}", x, y);

    // 그러나 heap에 잡히는 메모리는 ownership이 이전된다.
    let a = Box::new(5i32); // 'a' is a pointer to a _heap_ 
    println!(" a contains: {}", a); 

    let b = a; // Move 'a' into 'b'. Both a, and b are pointers
                         // to the same _heap_, but b owns it
    // error !
    // value borrowed here after move
    // a의 ownership이 이전되었기 때문에, a를 접근하면 컴파일 타임에 에러가 발생한다.
    //println!(" a contains: {}", a); 

    println!(" a was moved to b, b contains: {}", b);  // b has the _heap_

    destroy_box(b);
 
    // b lost ownership for _heap_
    //println!(" b contains: {}", b); 

    // String은 Heap에 잡힌다.
    let str_1 = String::from("Hello");
    println!("str_1:{}", str_1);

    let str_2 = str_1;
    // str_1의 ownership이 str_2로 이전 되었으므로, 컴파일 타임에 에러가 발생한다.
    //println!("str_1:{}", str_1);

    println!("str_2:{}", str_2);
}

// Mutability of data can be changed when ownership is transfered
// https://doc.rust-lang.org/rust-by-example/scope/move/mut.html
fn test_mutability() {

    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);
 
    // error - mutability error
    //*immutable_box = 4;

    let mut mutable_box = immutable_box;
    // error - immutable_box lost ownership
    //println!("immutable_box contains {}", immutable_box);

    println!("mutable_box contains {}", mutable_box);

    *mutable_box = 4;

    println!("mutable_box contains {}", mutable_box);
}


// Partial Moves
// https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html
fn test_partial_moves() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // name is moved, but age is a ref from 'person'
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // error - partiall moved ('person' cannot access 'name')
    //println!("The person struct is {:?}", person);

    // person still has ownership for 'age' as 'ref' keyword is used 
    println!("The person.age is {:?}", person.age);
}

// Borrowing
// https://doc.rust-lang.org/rust-by-example/scope/borrow.html
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn test_borrowing() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow (ownership is not taken)
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference again
        let _ref_to_i32: &i32 = &boxed_i32;

        // move
        //eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}


// Mutability (https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html)

#[derive(Clone, Copy)]
struct Book {
    // reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.author, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn test_borrowing_mutability() {

    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 19279,
    };

    // Copy happens
    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);
    borrow_book(&immutabook);
    borrow_book(&mutabook);

    // error
    //new_edition(&mut immutabook);
}

// https://doc.rust-lang.org/rust-by-example/scope/borrow/alias.html

struct Point { 
    x: i32, 
    y: i32, 
    z: i32
}

fn test_borrowing_aliasing() {
    let mut point = Point {
        x: 0,
        y: 0,
        z: 0,
    };
    let borrowed_point = &point;
    let another_borrow = &point;

    println!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x, borrowed_point.y, borrowed_point.z);
    
    // error     
    //let mutable_borrow = &mut point;

    // Error By this borrowing accessing, the above '&mut point' is not allowed 
    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, borrowed_point.y, borrowed_point.z);

    // now it can be borrowed as mutable
    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Errors 
    //let another_mut_borrow = &point;
    //let y = &point.y;
    //println!("Point Z coordinate is {}", point.z); // immutable access

    // Error - By this mutable borrowing accessing, immutable access is not allowed
    println!("Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    let new_borrowed_point = &point;
    println!("Point has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);   

}

// The ref pattern 
// https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html

#[derive(Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
}

fn test_ref_pattern() {
    let c: char = 'Q';

    // Below both are same (left side expression & right side expression)
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point2 {
        x: 0,
        y: 0,
    };

    let _copy_of_x = {
        let Point2 {
            x: ref ref_to_x,
            y: _
        } = point;
        *ref_to_x;
    };

    // mutable copy of 'point'
    let mut mutable_point = point;

    {
        let Point2 {
            x: _,
            y: ref mut mut_ref_to_y
        } = mutable_point;
        *mut_ref_to_y;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);


}

// Lifetimes (https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
fn test_lifetimes() {

    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
        // end of borrow1 lifetime
    }
    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
        // end of borrow2 lifetime
    }
    println!("i: {}", i);
    // end of i - lifetime
}

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/explicit.html

// 'print_refs' takes 2 references to 'i32' which have different lifeyimes 'a, and 'b
// These 2 lifeyimes must both be at least as long as the function 'print_refs;
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;
    // ERROR: `_x` does not live long enough
    //let y: &'a i32 = &_x;
}

fn test_explicit_lifetime_annotation() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);
    failed_borrow();
}

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/fn.html

fn print_one<'a>(x: &'a i32) {
    println!("`print_one`: x is {}", x);    
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

fn test_lifetimes_functions() {
    let x = 7;
    let y = 9;
    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

struct Owner(i32);

impl Owner {
    fn add_one<'a> (&'a mut self) {
        self.0 += 1;
    }
    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

fn test_lifetimes_methods() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

//https://doc.rust-lang.org/rust-by-example/scope/lifetime/struct.html

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn test_lifetimes_struct() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

//https://doc.rust-lang.org/rust-by-example/scope/lifetime/trait.html
/*
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        };
        borrowed
    }
}

fn test_lifetimes_trait() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}
*/

//https://doc.rust-lang.org/rust-by-example/scope/lifetime/lifetime_bounds.html
use std::fmt::Debug;
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T) where T:Debug {
    println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where T:Debug + 'a {
    println!("print_ref: t is {:?}", t);
}

fn test_lifetimes_bound() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/lifetime_coercion.html

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn test_lifetimes_corecion() {
    let first = 2;

    {
        let second = 3; // Shorter lifetime
        
        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn test_lifetimes_static() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);
}

extern crate rand;
use rand::Fill;

fn random_vec() -> &'static [usize; 100] {
    let mut rng = rand::thread_rng();
    let mut boxed = Box::new([0; 100]);
    boxed.try_fill(&mut rng).unwrap();
    Box::leak(boxed)
}

fn test_lifetimes_static_2() {
    let first: &'static [usize; 100] = random_vec();
    let second: &'static [usize; 100] = random_vec();
    assert_ne!(first, second)
}

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html

fn elided_input(x: &i32) {
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("`annotated_input`: {}", x);
}

fn elided_pass(x: &i32) -> &i32 { 
    x 
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { 
    x
}

fn test_lifetimes_elision() {
    let x = 3;
    elided_input(&x);
    annotated_input(&x);
    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));

}


pub fn test_scoping_ruiles() {
    println!("test_scoping_ruiles");
    // test_raii();
     test_ownership_and_move();
    // test_mutability();
    // test_partial_moves();
    // test_borrowing();
    // test_borrowing_mutability();
    // test_borrowing_aliasing();
    // test_ref_pattern();
    // test_lifetimes();
    // test_explicit_lifetime_annotation();
    // test_lifetimes_functions();
    // test_lifetimes_methods();
    // test_lifetimes_struct();
    // test_lifetimes_bound();
    // test_lifetimes_corecion();
}