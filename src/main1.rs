extern crate pid_control;

use pid_control::PIDController;
use pid_control::Controller;

fn main() {
    let mut p = PIDController::new(0.1, 0.2, 0.5);

    p.set_target(0.5);

    let mut noise: f64 = 0.05;
    let mut val: f64 = 0.0;
    let mut actual: f64 = 0.0;
    let mut control_input = 0.0;

    loop {
    	if noise > 400.0 {
    		break;
    	}

    	noise += 0.05;

    	val = noise.sin();

    	actual = val * control_input;

    	control_input = p.update(actual, 1.0);

    	println!("{} {} {}", val, actual, control_input);
    }
}
