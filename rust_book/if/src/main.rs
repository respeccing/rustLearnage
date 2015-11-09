#![allow(unused_variables)]

fn main() {
    println!("Hello, world!");
    let x = 5;

    let y = if x == 5 { 10 } else { 15 }; // y: i32
//    let yy = if x == 5 { 10 }; //fail, need else
    let zz = if x == 5 { () };
    let z: () = if x == 5 { () };
//    let q: i32 = if x == 5 { 10 }; //fail, need else
//    let q: i32 = if x != 5 { 10 }; //fail, need else
    let r: i32 = if x != 5 { 10 } else { 15 };
}
