
extern crate pid_control;

use pid_control::PIDController;
use pid_control::Controller;

mod navigation;
use navigation::{Navigation};

mod lib;
use lib::{Agent, Environment, FlightGear, PossibleAction};


fn main() {
	println!("Kek!");

	let mut n = Navigation::new( 19.754154, -156.044102 ); // home: 21.32525 , -157.94319     close airport: 21.35437 , -157.71158      next island airport: 19.754154, -156.044102

	let mut e = FlightGear::new();

	let mut a = Agent::init(&mut e);

	let mut roll_pid = PIDController::new(0.08, 0.01, 0.003);
	// let mut roll_pid = PIDController::new(0.003, 0.003, 0.004);
	// let mut roll_pid = PIDController::new(0.01, 0.003, 0.004);
	let mut desired_roll_angle = 0.0;
	roll_pid.set_limits(-1.0,1.0);
    roll_pid.set_target(desired_roll_angle);

	let mut pitch_pid = PIDController::new(-0.01, -0.001, 0.01);
	// let mut pitch_pid = PIDController::new(-0.003, -0.001, -0.008);
	// let mut pitch_pid = PIDController::new(-0.003, -0.0017, -0.01);
	// let mut pitch_pid = PIDController::new(-0.0003, -0.0001, -0.01);
	// let mut pitch_pid = PIDController::new(-0.0003, -0.001, -0.01);
	pitch_pid.set_limits(-1.0,1.0);
    pitch_pid.set_target(0.0);

	// let mut heading_pid = PIDController::new(-0.01, -0.001, 0.01);

	let mut bank_turn_pid = PIDController::new(-0.3, -0.00, -0.0);
	// let mut bank_turn_pid = PIDController::new(-0.1, -0.06, -0.005); //better
	// let mut bank_turn_pid = PIDController::new(-0.08, -0.07, -0.09);  //strange but close
	// let mut bank_turn_pid = PIDController::new(-0.003, 0.0001, 0.0001); // but after reaching end goes bad
	bank_turn_pid.set_limits(-45.0,45.0);
	bank_turn_pid.set_target(0.0);


	a.run(|state, paction| {
		let paction = *paction;
		let paction = match paction {
			PossibleAction::Some(mut action) => {
				let dest_delta_heading = n.get_delta_heading_to_destination(state.gps_latitude, state.gps_longitude, state.heading);

				// if dest_delta_heading > 4.0 {
				// 	let turn_input = bank_turn_pid.update(dest_delta_heading as f64, 0.1) as f32;
				// 	roll_pid.set_target(turn_input as f64);
				// } else {

				// }
				let turn_input = bank_turn_pid.update(dest_delta_heading as f64, 0.1) as f32;
				roll_pid.set_target(turn_input as f64);
				
				// println!("action : {:?}", action);
				let roll_input = roll_pid.update(state.roll as f64, 0.1) as f32;
				// println!("ri: {}, ra: {}", roll_input, state.roll);
				action.aileron = roll_input;

				let pitch_input = pitch_pid.update(state.pitch as f64, 0.1) as f32;
				// println!("pi: {}, pa: {}", pitch_input, state.pitch);
				action.elevator = pitch_input;

				println!("ti: {}, dra: {}, dh: {}, ri: {}, pi: {}", format!("{:.*}", 4, turn_input), format!("{:.*}", 2, desired_roll_angle), format!("{:.*}", 2, dest_delta_heading), format!("{:.*}", 4, roll_input), format!("{:.*}", 4, pitch_input));

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
