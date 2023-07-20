use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("analyze_slice");
    println!(" slice[0] = {}, len: {}", slice[0], slice.len());
}


pub fn test_array_slice() {
    println!("test_array_slice");

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("xs[0] = {}, len: {}", xs[0], xs.len());

    let ys: [i32; 500] = [0; 500];
    println!("ys[0] = {}, len: {}", ys[0], ys.len());
    println!("ys[10] = {}", ys[10]);
    println!("size of ys = {}", mem::size_of_val(&ys));

    analyze_slice(&xs);
    analyze_slice(&xs[1..4]);
    analyze_slice(&ys[10..12]);
    

}

