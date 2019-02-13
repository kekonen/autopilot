
extern crate pid_control;

use pid_control::PIDController;
use pid_control::Controller;

mod navigation;
use navigation::{Navigation, Location};

mod lib;
use lib::{Agent, Environment, FlightGear, PossibleAction};


fn main() {
	println!("Kek!");

	let mut n = Navigation::new( ); 
	n.set_dest_location(Location::new(21.35437 , -157.71158)); // home: 21.32525 , -157.94319     close airport: 21.35437 , -157.71158      next island airport: 19.754154, -156.044102

	let mut e = FlightGear::new();

	let mut a = Agent::init(&mut e);

	let mut roll_pid = PIDController::new(0.08, 0.01, 0.003);
	
	let desired_roll_angle = 0.0;
	roll_pid.set_limits(-1.0,1.0);
    roll_pid.set_target(desired_roll_angle);

	let mut pitch_pid = PIDController::new(-0.01, -0.001, 0.01);
	
	pitch_pid.set_limits(-1.0,1.0);
    pitch_pid.set_target(0.0);

	let mut heading_pid = PIDController::new(-0.06, -0.03, 0.04);
	heading_pid.set_limits(-1.0,1.0);
    heading_pid.set_target(0.0);

	let mut bank_turn_pid = PIDController::new(-0.3, -0.00, -0.0);
	
	bank_turn_pid.set_limits(-45.0,45.0);
	bank_turn_pid.set_target(0.0);


	a.run(|state, paction| {
		let paction = *paction;
		let paction = match paction {
			PossibleAction::Some(mut action) => {
				let current_location = Location::new(state.gps_latitude, state.gps_longitude);

				// if (n.get_dest_location().distance_to(&current_location) < 1.0) {
					
				// }

				let dest_delta_heading = n.get_delta_heading_to_destination(&current_location, state.heading);

				let some_input_for_logging;
				if dest_delta_heading > 4.0 || dest_delta_heading < -4.0 {
					let turn_input = bank_turn_pid.update(dest_delta_heading as f64, 0.1) as f32;
					roll_pid.set_target(turn_input as f64);
					some_input_for_logging = turn_input;
					action.rudder = 0.0;
				} else {
					let yaw_input = heading_pid.update(dest_delta_heading as f64, 0.1) as f32;
					action.rudder = yaw_input;
					some_input_for_logging = yaw_input;
					roll_pid.set_target(0.0);
				}

				let roll_input = roll_pid.update(state.roll as f64, 0.1) as f32;
				// println!("ri: {}, ra: {}", roll_input, state.roll);
				action.aileron = roll_input;

				let pitch_input = pitch_pid.update(state.pitch as f64, 0.1) as f32;
				// println!("pi: {}, pa: {}", pitch_input, state.pitch);
				action.elevator = pitch_input;

				println!("ti: {}, dra: {}, dh: {}, ri: {}, pi: {}, dest: {}", format!("{:.*}", 4, some_input_for_logging), format!("{:.*}", 2, desired_roll_angle), format!("{:.*}", 2, dest_delta_heading), format!("{:.*}", 4, roll_input), format!("{:.*}", 4, pitch_input), format!("{:.*}", 4, n.get_dest_location().distance_to(&current_location)));

				PossibleAction::Some(action)
				// PossibleAction::None
			},
			PossibleAction::None => PossibleAction::None,
		};

		// println!("p: {}, r: {}, h: {}, action: {:?}", format!("{:.*}", 2, state.pitch), format!("{:.*}", 2, state.roll), format!("{:.*}", 2, state.heading), paction);
		// println!(", n: {}", n.get_delta_heading_to_destination(state.gps_latitude, state.gps_longitude, state.heading));

		paction
		// PossibleAction::None

	});


}
