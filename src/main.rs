
extern crate pid_control;

use pid_control::PIDController;
use pid_control::Controller;



mod lib;
use lib::{Agent, Environment, FlightGear, PossibleAction};

fn main() {
	println!("Kek!");

	let mut e = FlightGear::new();

	let mut a = Agent::init(&mut e);

	let mut roll_pid = PIDController::new(0.003, 0.002, 0.003);
	// let mut roll_pid = PIDController::new(0.003, 0.003, 0.004);
	// let mut roll_pid = PIDController::new(0.01, 0.003, 0.004);
    roll_pid.set_target(0.0);

	let mut pitch_pid = PIDController::new(-0.003, -0.0007, -0.01);
	// let mut pitch_pid = PIDController::new(-0.003, -0.001, -0.008);
	// let mut pitch_pid = PIDController::new(-0.003, -0.0017, -0.01);
	// let mut pitch_pid = PIDController::new(-0.0003, -0.0001, -0.01);
	// let mut pitch_pid = PIDController::new(-0.0003, -0.001, -0.01);
    pitch_pid.set_target(0.0);



	a.run(|state, paction| {
		let paction = *paction;
		let paction = match paction {
			PossibleAction::Some(mut action) => {
				// println!("action : {:?}", action);
				let roll_input = roll_pid.update(state.roll as f64, 0.1) as f32;
				println!("c: {}", roll_input);
				action.aileron += roll_input;

				let pitch_input = pitch_pid.update(state.pitch as f64, 0.1) as f32;
				println!("c: {}", pitch_input);
				action.elevator += pitch_input;

				PossibleAction::Some(action)
			},
			PossibleAction::None => PossibleAction::None,
		};

		println!("p: {}, r: {}, h: {}, action: {:?}", format!("{:.*}", 2, state.pitch), format!("{:.*}", 2, state.roll), format!("{:.*}", 2, state.heading), paction);

		paction
		// PossibleAction::None

	});


}
