#![allow(unused)]
use std::fmt::{Debug, Display};

struct A;
struct Single(A);
struct SingleGen<T>(T);

fn generic_basic() {
    let _s: Single = Single(A);
    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}

// Functions (https://doc.rust-lang.org/rust-by-example/generics/gen_fn.html)
//struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

fn generic_function() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));
    generic(SGen('c'));
}

// Implementation (https://doc.rust-lang.org/rust-by-example/generics/impl.html)
// struct A;
//struct GenericVal<T>(T); // Generic type 'GenericVal'
//impl GenericVal<f32> {}
//impl GenericVal<S> {}
//impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

struct GenVal<T> {
    gen_val: T,
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn generic_implementation() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    println!("{}, {}", x.value(), y.value());
}

// Traits (https://doc.rust-lang.org/rust-by-example/generics/gen_trait.html)
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {        
    }
}

fn generic_traits() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
}

// Bounds (https://doc.rust-lang.org/rust-by-example/generics/bounds.html)

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// The generic 'T' must implement 'Debug'. Regardless of the type, this will work properly
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// The generic 'T' must implement 'HasArea'. Any type which meets the bound can acess 'HasArea's function 'area'
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn generic_bound() {
    let rectangle = Rectangle {length: 3.0, height: 4.0};
    let _triangle = Triangle {length: 3.0, height: 4.0};

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));    
    //print_debug(&_triangle);
    //println!("Area: {}", area(&_triangle));
}

// Multiple bounds (https://doc.rust-lang.org/rust-by-example/generics/multi_bounds.html)

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u:&U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

fn generic_multiple_bounds() {
    let string = "workds";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_types(&array, &vec);
}

// Where caluses (https://doc.rust-lang.org/rust-by-example/generics/where.html)

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn generic_where_clauses() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}


// New Type Idiom (https://doc.rust-lang.org/rust-by-example/generics/new_types.html)

struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn generic_new_type_idiom() {
    let age = Years(5);
    let age_days = age.to_days();

    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // error - it needs Years, not Days
    //println!("Old enough {}", old_enough(&age_days));

    let years = Years(42);
    let _years_as_primitive_1: i64 = years.0;
    let Years(_years_as_primitive_2) = years;
}

// Associated Items (https://doc.rust-lang.org/rust-by-example/generics/assoc_items/the_problem.html)

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _:&B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

trait Contains2 {
    type A;
    type B;
    fn contains2(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first2(&self) -> i32;
    fn last2(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, number_1: &i32, number_2:&i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

impl Contains2 for Container {
    type A = i32;
    type B = i32;
    fn contains2(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first2(&self) -> i32 { self.0 }
    fn last2(&self) -> i32 { self.1 }
}

fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
    container.last() - container.first()
}

fn difference2<C: Contains2>(container: &C) -> i32 {
    container.last2() - container.first2()
}

fn generic_assoicated_item_problem() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}", &number_1, &number_2, 
            container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    println!("The difference is: {}", difference(&container));
}

fn generic_associated_item() {
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}", &number_1, &number_2, 
    container.contains2(&number_1, &number_2));
    println!("First number: {}", container.first2());
    println!("Last number: {}", container.last2());
    println!("The difference is: {}", difference2(&container));
}

// Phantom type parameters (https://doc.rust-lang.org/rust-by-example/generics/phantom.html)

use std::marker::PhantomData;

// Phantom tuple struct which is generic over 'A' with hidden parameter 'B'
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>); 

#[derive(PartialEq)]
struct PhantomStruct<A, B>{ first: A, phantom: PhantomData<B>}

fn generic_phantom_type_parameter() {

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    let _struct: PhantomStruct<char, f32> = PhantomStruct { first: 'Q', phantom: PhantomData };
    let _struct: PhantomStruct<char, f64> = PhantomStruct { first: 'Q', phantom: PhantomData };

    // error - it cannot be compared
    //println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
    //println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);
}

pub fn test_generic() {
    println!("* test_generic");
    generic_basic();
    generic_function();
    generic_implementation();
    generic_traits();
    generic_bound();
    generic_multiple_bounds();
    generic_where_clauses();
    generic_new_type_idiom();
    generic_assoicated_item_problem();
    generic_associated_item();
    generic_phantom_type_parameter();
}

