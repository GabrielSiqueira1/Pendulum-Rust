fn main() {
    println!("Hello, world!");
}

struct Pendulum{
    angle: f32, // Pendulum Angle

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, // The length of the pendulum
    m: f32, // Ball mass
    g: f32, // The gravity
}

impl Pendulum {
    fn new(){

    }

    fn update(){

    }

    fn draw() {

    }
}

mod vector { // isolate sequencitial code

    struct Vector{
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector{
            Vector { 
                x, 
                y,
            }
        }

        pub fn add(){

        }

        pub fn set(){

        }
    }
}