use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

use vector::Vector;
fn main() {
    // We need this window object to create a window
    let window = Window::new_centered("Title", (640, 480)).unwrap();
    
    let win: MyWindowHandler = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0)
    };

    window.run_loop(win);
}

struct MyWindowHandler {
    p: Pendulum,
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        self.p.update();
        self.p.draw(graphics);

        helper.request_redraw();
    }

   // If desired, on_mouse_move(), on_key_down(), etc...
}

struct Pendulum{

    origin: vector::Vector, // This vector is the position of the pendulum

    position: vector::Vector, // This vector is the position of the balls

    angle: f32, // Pendulum Angle

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, // The length of the pendulum
    m: f32, // Ball mass
    g: f32, // The gravity
}

impl Pendulum {
    fn new(x:f32, y:f32, r:f32) -> Pendulum{
        Pendulum { 
            // Setting origin of the pendulum
            origin: Vector::new(x, y), 

            // We'll set the position when we update the pendulum
            // The initial value is 0.0
            position: Vector::new(0.0, 0.0), 
            angle: 1.0,                             // In radians
            angular_velocity: 0.0,                  // Dont have initial moviment
            angular_acceleration: 0.0,              // Dont have initial acceleration
            
            r: r, 
            m: 1.0, 
            g: 1.5, 
        }
    }

    fn update(&mut self){
        // Pendunlum equation for angular acceleration
        self.angular_acceleration = -1.0*self.g*self.angle.sin() /self.r;

        // The angular velocity depends of the angular accelaration
        self.angular_velocity += self.angular_acceleration;

        // The angle is the angle plus the angular velocity 
        self.angle += self.angular_velocity;

        // The position is the polar coordinates translated to cartesian coordiantes
        self.position.set(self.r*self.angle.sin(), self.r*self.angle.cos());

        // The final position of the ball in the canvas
        // Pendulum plus the position vector.
        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}

mod vector { // isolate sequencitial code

    pub struct Vector{
        pub x: f32,
        pub y: f32,
    }

    impl Vector {
        pub fn new(x: f32, y: f32) -> Vector{ // Associated function
            Vector { 
                x, 
                y,
            }
        }

        pub fn add(&mut self, other: &Vector) -> &Vector{ // Method function -> self
            self.x += other.x;
            self.y += other.y;
            self 
        }

        pub fn set(&mut self, x: f32, y: f32) -> &Vector{
            self.x = x;
            self.y = y;
            self
        }
    }
}