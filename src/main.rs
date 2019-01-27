



mod lib;
use lib::serve;
use lib::{Environment, FlightGear, Server, EnvResult, State, Action};
// use lib::EnvResult;

fn main() {
	println!("Kek!");

	// let s = State::<f32> {pitch: 0.0, roll: 0.0, heading: 0.0, altitude: 0.0, latitude: 0.0, longitude: 0.0, a: A::<f32>{x:0.0,y:0.0,z:0.0}, v: V::<f32>{x:0.0,y:0.0,z:0.0}};
	// let a = Action::<f32> {aileron: 0.0, elevator: 0.0, rudder: 0.0, aileron_trim: 0.0, elevator_trim: 0.0, rudder_trim: 0.0};
	// let s = State {pitch: 0.0, roll: 0.0, heading: 0.0, altitude: 0.0, latitude: 0.0, longitude: 0.0, a: A{x:0.0,y:0.0,z:0.0}, v: V{x:0.0,y:0.0,z:0.0}};
	// let a = Action {throttle: 0.0, aileron: 0.0, elevator: 0.0, rudder: 0.0, aileron_trim: 0.0, elevator_trim: 0.0, rudder_trim: 0.0};

	// let s1 = s.clone();

	// println!("lol: {:?}", s1);


	let mut e = FlightGear::new();
	let result = e.reset();
	match result {
		EnvResult::Some(state) => println!("state: {:?}", state),
		EnvResult::Done => println!("DONE!"),
		Error => println!("Error"),
	}

	loop {
		let action = Action{aileron: 0.0, elevator: 0.0, rudder: 0.0};

		let result = e.step(action);
		match result {
			EnvResult::Some(state) => println!("state: {:?}", state),
			EnvResult::Done => println!("DONE!"),
			Error => break,
		}
	}
	
	// let served = serve();
	// println!("fg: {:?}", served)
	// let s1 = Server::init("127.0.0.1:34255");
	// let s2 = Server::init("127.0.0.1:34255");
	// let kek = s1.receive().unwrap();
	// println!("{}", kek);

	// let result = s1.send(&kek, "127.0.0.1:");
	// println!("Result: {:?}", result);

}