
#![forbid(non_shorthand_field_patterns)]

#![warn(dead_code)]

#![warn(trivial_casts)]

#![allow(missing_docs)]

#![warn(unsafe_code)]


#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]

#![warn(box_pointers)]

#![warn(trivial_numeric_casts)]
#![warn(unstable_features)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

#![feature(plugin)]

#![plugin(clippy)] //thanks to arc on irc #rust for suggesting clippy!

#![deny(clippy)]  //this doesn't work (it should imply the below, but it doesn't, so the below is needed!)) well the readme was inconsistent!
//#![deny(shadow_unrelated)]
#![deny(clippy_pedantic)] //this includes the above.

fn main() {
    let x=10;
    println!("{}",x);
    let x=11; //done via clippy: want a (lint check) warning here
    println!("{}",x);
    {
        let x=12; //done via clippy: want a (lint check) warning here
        println!("{}",x);
    }
    println!("{}",x);
}

