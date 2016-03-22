/// from: https://raw.githubusercontent.com/chriskrycho/newrustacean.com/master/src/e002.rs


/// This struct is simple but useful to see how borrowing and moving work.
#[derive(Debug)]
pub struct Circle {
    /// X position of the circle's origin.
    pub x: f64,
    /// Y position of the circle's origin
    pub y: f64,
    /// Radius of the circle
    pub r: f64,

    pub w: Boxx,
}

#[derive(Debug)]
pub struct Boxx {
    pub x: f64,
    pub y: f64,
}

//const nil:Circle= Circle{x:1,y:2,r:3,w:nul};
//const nul:Circle= Circle{x:1,y:2,r:3,w:nil};

impl Circle {
    /// Creates a `Circle` instance centered on the "origin" (x = 0, y = 0).
    fn origin() -> Circle {
        Circle {
            x: 1.2,
            y: 3.4,
            r: 5.6,
            w: Boxx{x:1.0, y:2.0},
        }
    }

    pub fn by_take(self) -> f64 {
        println!("Taking ownership, not just borrowing a reference. INTENSE.");
        self.x
    }
}

fn main() {
    // Constructors can have default behavior, like this one.
    let immutable = Circle::origin();

    let by_take_x = immutable.by_take();//TODO: what happened with y,z? or even x? they got destroyed? so the return is a copy/clone then? since self.x is returned!
    println!("By take: {}", by_take_x);
    //println!("Does the circle still exist? {:?}", immutable);//no
}

