
use std::env;
use std::ffi::OsString;
//use std::convert::From; //don't need this, the error was confusing!

fn main() {
    let z=env::var_os("RUST_BACKTRACE");
    let q=OsString::from("2"); //src/main.rs:8:9: 8:10 warning: unused variable: `q`, #[warn(unused_variables)] on by default
    println!("val={}", match z {
        Some(q) => { //XXX: this does not use the above 'q' ! and no warning!
//            println!("{}", q.as_ref());
            println!("{}", q.to_string_lossy());
            2 
        },
        None => 1,
    })

}

// vim note: = , the indent command can take motions. So, gg to get the start of the file, = to indent, G to the end of the file, gg=G.
// src: https://stackoverflow.com/questions/506075/how-do-i-fix-the-indentation-of-an-entire-file-in-vi

