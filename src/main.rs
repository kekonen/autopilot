



mod lib;
use lib::{Agent, Environment, FlightGear};

fn main() {
	println!("Kek!");

	let mut e = FlightGear::new();

	let mut a = Agent::init(&mut e);

	a.run(|state, action| {
		let mut action = *action;
		action.aileron +=0.01;
		action
	});


}
