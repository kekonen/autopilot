
// extern crate pid_control;

// use pid_control::PIDController;
// use pid_control::Controller;



mod lib;
use lib::{Agent, Environment, FlightGear, PossibleAction};

fn main() {
	println!("Kek!");

	let mut e = FlightGear::new();

	let mut a = Agent::init(&mut e);

	// let mut p = PIDController::new(0.1, 0.2, 0.5);

    // p.set_target(0.);


	a.run(|state, paction| {
		let paction = *paction;
		// match paction {
		// 	PossibleAction::Some(mut action) => {
				
		// 		action.aileron +=0.01;
		// 		return PossibleAction::Some(action)
		// 	},
		// 	PossibleAction::None => return PossibleAction::None,
		// }
		PossibleAction::None

	});


}
