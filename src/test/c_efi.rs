#![allow(unused)]

use libc::size_t;

// snappy is called by -lsnappy 
// (it is installed by using apt-get install libsnappy-dev in Dockerfile)

#[link(name = "snappy")]
extern {
    fn snappy_max_compressed_length(source_length: size_t) -> size_t;
}

pub fn test_c_efi() {    
    println!("test_c_efi");
    unsafe {
        println!(" calling c library result = {}", snappy_max_compressed_length(10));
    }
}

