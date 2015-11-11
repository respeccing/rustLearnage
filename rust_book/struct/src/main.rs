//src: https://doc.rust-lang.org/nightly/book/structs.html

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, struct world!");
    let origin = Point { y: 2, x: 1 }; // origin: Point
	let o1 = Point { x:1, y:2 };
//	let o2 = Point { 1,2 };//not allowed, good!
    println!("The origin is at ({}, {})", origin.x, origin.y);
	println!("{:?}",o1);
}
