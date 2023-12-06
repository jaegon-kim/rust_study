


fn used_function() {

}

#[allow(dead_code)]
fn unused_function() {

}

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("On Linux");
}

#[allow(dead_code)]
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("'Not' On Linux");
}

#[allow(dead_code)]
pub fn test_attributes() {
    println!("test_attributes");
    used_function();
    are_you_on_linux();
}
