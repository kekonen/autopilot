#[derive(Debug, Clone, Copy)]
struct A{
	x: f32,
	y: f32,
	z: f32,
}

#[derive(Debug, Clone, Copy)]
struct V{
	x: f32,
	y: f32,
	z: f32,
}

#[derive(Debug, Copy, Clone)]
enum EnvResult<T>{
	Done,
	Some(T)
}


#[derive(Debug, Copy, Clone)]
struct State { // may be Radians for pitch/roll/heading? but for now f32
	pitch:     f32,
	roll:      f32,
	heading:   f32, 
	altitude:  f32,
	latitude:  f32,
	longitude: f32,
	a:           A,
	v:           V, 
}

// impl<T> Clone for State<T> {
//     fn clone(&self) -> State<T> { *self }
// }

#[derive(Debug, Copy, Clone)]
struct Action{
	throttle:      f32,
	aileron:       f32,
	elevator:      f32,
	rudder:        f32,
	aileron_trim:  f32,
	elevator_trim: f32,
	rudder_trim:   f32,
}



// impl<T> Clone for Action<T> {
//     fn clone(&self) -> Action<T> { *self }
// }


trait Environment{
	fn new() -> Self;
	
	fn set_state(&mut self, state: State) -> bool;

    fn reset(self) -> EnvResult<State>;

	fn step(mut self, action: Action) -> EnvResult<State>;

}

#[derive(Debug, Copy, Clone)]
struct FlightGear{
	state: State,
	action: Action,
}

impl Environment for FlightGear {
	fn new() -> FlightGear {
		let s = State {pitch: 0.0, roll: 0.0, heading: 0.0, altitude: 0.0, latitude: 0.0, longitude: 0.0, a: A{x:0.0,y:0.0,z:0.0}, v: V{x:0.0,y:0.0,z:0.0}};
		let a = Action {throttle: 0.0, aileron: 0.0, elevator: 0.0, rudder: 0.0, aileron_trim: 0.0, elevator_trim: 0.0, rudder_trim: 0.0};
		FlightGear{state:s, action:a}
	}

	fn set_state(&mut self, state: State) -> bool {
		self.state = state;
		true
	}

	fn reset(self) -> EnvResult<State> {
		EnvResult::Some(self.state)
		// EnvResult::Done
	}

	fn step(mut self, action: Action) -> EnvResult<State> {
		EnvResult::Some(self.state)	
	}
}



pub trait Agent{

}

mod lib;
use lib::serve;

fn main() {
	println!("Kek!");

	// let s = State::<f32> {pitch: 0.0, roll: 0.0, heading: 0.0, altitude: 0.0, latitude: 0.0, longitude: 0.0, a: A::<f32>{x:0.0,y:0.0,z:0.0}, v: V::<f32>{x:0.0,y:0.0,z:0.0}};
	// let a = Action::<f32> {aileron: 0.0, elevator: 0.0, rudder: 0.0, aileron_trim: 0.0, elevator_trim: 0.0, rudder_trim: 0.0};
	let s = State {pitch: 0.0, roll: 0.0, heading: 0.0, altitude: 0.0, latitude: 0.0, longitude: 0.0, a: A{x:0.0,y:0.0,z:0.0}, v: V{x:0.0,y:0.0,z:0.0}};
	let a = Action {throttle: 0.0, aileron: 0.0, elevator: 0.0, rudder: 0.0, aileron_trim: 0.0, elevator_trim: 0.0, rudder_trim: 0.0};

	let s1 = s.clone();

	println!("lol: {:?}", s1);


	let e = FlightGear::new();
	let result = e.reset();
	match result {
		EnvResult::Some(state) => println!("state: {:?}", state),
		EnvResult::Done => println!("DONE!"),
	}
	let served = serve();
	println!("fg: {:?}", served)

}