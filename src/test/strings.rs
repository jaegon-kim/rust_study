#![allow(unused)]

// There are two types of strings in Rust: String and &str.

fn test_strings_basic() {
    println!("test_strings");

    // A reference to a string allocated in read only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram:{}", pangram);

    for word in pangram.split_whitespace() {
        print!("{}, ", word);
    }
    println!();

    for word in pangram.split_whitespace().rev() {
        print!("{}, ", word);
    }
    println!();

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup(); // remove duplication ...?

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }

    println!("string: {}", string);

    // trimmed string is a slice to the original string
    //  (no new allocation)
    let chars_to_trim: &[char] = &[' ', ',', 'a', 'b', 'c'];

    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters:{}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");

    // Allocate new memory and store the modified string there 
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says:{}", alice);
    println!("Bob says:{}", bob);

}


pub fn test_strings() {
    test_strings_basic();
    
}
