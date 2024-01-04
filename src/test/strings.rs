#![allow(unused)]

use std::str;

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

fn test_literals_escapes() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                    can span multiple lines.
                    The linebreak and indentation here ->\
                    <- can be escaped too!";
    println!("{}", long_string);

    // Raw string
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // Quoting raw string
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

}

fn test_bytestring() {
    // // Byte arrays don't have the `Display` trait, so printing them is a bit limited
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
    like with normal raw strings"#;

    /*
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
    */

}


pub fn test_strings() {
    test_strings_basic();
    test_literals_escapes();
    test_bytestring();  
}
