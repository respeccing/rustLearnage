
use std::env;
use std::ffi::OsString;

//use core::convert::From;//inexistent
use std::convert::From; //don't need this, the error remains

//Rust has a keyword, match, that allows you to replace complicated if/else groupings with something more powerful. Check it out:
fn main() {
    let z=env::var_os("RUST_BACKTRACE");
    let q=OsString::from("1");
    println!("val={}", match z {
        Some(OsString::from("1")) => 2, //odd error when using this (not knowing): src/main.rs:11:14: 11:33 error: no associated item named `from` found for type `std::ffi::os_str::OsString` in the current scope
//        Some(q) => 2, //works
        None => 1,
    })

}

// vim note: = , the indent command can take motions. So, gg to get the start of the file, = to indent, G to the end of the file, gg=G.
// src: https://stackoverflow.com/questions/506075/how-do-i-fix-the-indentation-of-an-entire-file-in-vi

