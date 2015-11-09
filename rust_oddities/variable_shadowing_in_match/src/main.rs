#![forbid(non_shorthand_field_patterns)]

#![warn(dead_code)]
#![forbid(unsafe_code)]

#![allow(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

#![warn(box_pointers)]

#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
//#![warn(unstable_features)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

#![feature(plugin)]
#![plugin(clippy)]

#![deny(clippy)]
#![deny(clippy_pedantic)]


use std::env;
//use std::ffi::OsString;
//use std::convert::From; //don't need this, the error was confusing!

fn main() {
    let z=env::var_os("RUST_BACKTRACE");
    let q="to be shadowed";
    println!("q={}",q);
//    let q=OsString::from("2"); //src/main.rs:8:9: 8:10 warning: unused variable: `q`, #[warn(unused_variables)] on by default
    //XXX: they do mention shadowing: https://doc.rust-lang.org/nightly/book/patterns.html
    println!("val={}", match z {
        Some(q) => { //XXX: this does not use the above 'q' ! and no warning! clippy doesn't catch this, yet!
//            println!("{}", q.as_ref());
//            println!("{}", q.to_string_lossy());
            println!("z={:?}",q);
            1 
        },
        None => 0,
    });


//    let z=1;//well, since they don't catch it here, ofc they won't inside match! but maybe there's a flag that can be turned on! to warn/err! XXX:catched by clippy!
//    println!("{}",z);

 /*   let g=1;
    println!("g={}", match z {
        Some(g) => g,
        None => "0"
    });*/


}

// vim note: = , the indent command can take motions. So, gg to get the start of the file, = to indent, G to the end of the file, gg=G.
// src: https://stackoverflow.com/questions/506075/how-do-i-fix-the-indentation-of-an-entire-file-in-vi

