



mod lib;
use lib::{Agent, Environment, FlightGear};

fn main() {
	println!("Kek!");

	// let mut e = FlightGear::new();
	// let result = e.reset();
	// match result {
	// 	EnvResult::Some(state) => println!("state: {:?}", state),
	// 	EnvResult::Done => println!("DONE!"),
	// 	Error => println!("Error"),
	// }

	// loop {
	// 	let action = Action{aileron: 0.0, elevator: 0.0, rudder: 0.0};

	// 	let result = e.step(action);
	// 	match result {
	// 		EnvResult::Some(state) => println!("state: {:?}", state),
	// 		EnvResult::Done => println!("DONE!"),
	// 		Error => break,
	// 	}
	// }
	let mut e = FlightGear::new();

	let mut a = Agent::init(&mut e);

	a.run();


}