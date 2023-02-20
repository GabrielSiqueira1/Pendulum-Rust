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