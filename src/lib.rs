use std::net::UdpSocket;
use std::str;
use std::net::{SocketAddr}; // ToSocketAddrs

use std::str::FromStr;
use std::num::ParseFloatError;

// #![feature(rustc_private)]
// #[macro_use] extern crate log;

// pub fn serve() -> std::io::Result<()> {
//     {
//         let socket = UdpSocket::bind("127.0.0.1:34254")?;
//         //  ncat -v localhost 34254 -u

//         // Receives a single datagram message on the socket. If `buf` is too small to hold
//         // the message, it will be cut off.
//         let mut buf = [0; 10];
//         let (amt, src) = socket.recv_from(&mut buf)?;

//         // Redeclare `buf` as slice of the received data and send reverse data back to origin.
//         let buf = &mut buf[..amt];
//         buf.reverse();
//         socket.send_to(buf, &src)?;
//     } // the socket is closed here
//     Ok(())
// }

#[derive(Debug)]
pub struct Server{
    address: SocketAddr,
    socket: UdpSocket,
}


impl Server{
    pub fn init(address: SocketAddr) -> Server {
        let socket = UdpSocket::bind(address).unwrap();

        Server{address: address, socket: socket}
    }

    pub fn receive(&self) -> std::io::Result<String> {
        let mut buf = [0; 1024];
        let (amt, _src) = self.socket.recv_from(&mut buf)?;
        // println!("add: {:?}", src);

        let buf = &mut buf[..amt];
        let buf_str = str::from_utf8(&buf).unwrap();
        Ok(buf_str.to_string())
    }

    pub fn send(&self, message: &str, address: &SocketAddr) -> std::io::Result<()> {
        {
            self.socket.send_to(message.as_bytes(), address)?;
        }
        Ok(())
    }
}


// #[derive(Debug, Clone, Copy)]
// pub struct A{
// 	x: f32,
// 	y: f32,
// 	z: f32,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct V{
// 	x: f32,
// 	y: f32,
// 	z: f32,
// }

#[derive(Debug, Copy, Clone)]
pub enum EnvResult<T>{
	Done,
	Some(T),
    Error,
}


#[derive(Debug, Copy, Clone)]
pub struct State {
    pub pitch:              f32,
    pub roll:               f32,
    pub heading:            f32,
    pub turn_rate:          f32,
    pub g:                  f32,
    pub air_speed:          f32,
    pub altitude:           f32,
    pub vertical_speed:     f32,
    pub gps_vertical_speed: f32,
    pub gps_altitude:       f32,
    pub gps_latitude:       f32,
    pub gps_longitude:      f32,
    pub gps_ground_speed:   f32,
    pub ax:                 f32,
    pub ay:                 f32,
    pub az:                 f32,
    pub arx:                f32,
    pub ary:                f32,
    pub arz:                f32,
    pub vx:                 f32,
    pub vy:                 f32,
    pub vz:                 f32,
    pub vrx:                f32,
    pub vry:                f32,
    pub vrz:                f32
}

impl FromStr for State {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<f32> = s.trim().split(';')
        .map(|x| x.parse().unwrap())
        .collect();

        Ok(State {
            pitch: s[0],
            roll: s[1],
            heading: s[2],
            turn_rate: s[3],
            g: s[4],
            air_speed: s[5],
            altitude: s[6],
            vertical_speed: s[7],
            gps_vertical_speed: s[8],
            gps_altitude: s[9],
            gps_latitude: s[10],
            gps_longitude: s[11],
            gps_ground_speed: s[12],
            ax: s[13],
            ay: s[14],
            az: s[15],
            arx: s[16],
            ary: s[17],
            arz: s[18],
            vx: s[19],
            vy: s[20],
            vz: s[21],
            vrx: s[22],
            vry: s[23],
            vrz: s[24]
        })
    }
} 

#[derive(Debug, Copy, Clone)]
pub enum PossibleAction<Action>{
	Some(Action),
    None,
}

#[derive(Debug, Copy, Clone)]
pub struct Action{
	pub aileron:       f32,
	pub elevator:      f32,
	pub rudder:        f32,
    // throttle:      f32,
	// aileron_trim:  f32,
	// elevator_trim: f32,
	// rudder_trim:   f32,
}

impl ToString for Action {
    fn to_string(&self) -> String {
        format!("{};{};{}\n", self.aileron, self.elevator, self.rudder)
    }
}

impl Action {
    pub fn new() -> Self {
        Action {aileron: 0.0, elevator: 0.0, rudder: 0.0}
    }
}



pub trait Environment{
	fn new() -> Self;
	
    fn reset(&self) -> EnvResult<State>;

	fn step(&mut self, paction: PossibleAction<Action>) -> EnvResult<State>;

}

#[derive(Debug)]
pub struct FlightGear{
	// state: State,
	// action: Action,
	my_address: SocketAddr,
	fg_address: SocketAddr,
    server: Server,
}

impl Environment for FlightGear {
	fn new() -> FlightGear {

        let my_port = 1337;
        let fg_port = 1338;

        let my_address = SocketAddr::from(([127, 0, 0, 1], my_port));
        let fg_address = SocketAddr::from(([127, 0, 0, 1], fg_port));

        let server = Server::init(my_address); // "127.0.0.1:34255"
		FlightGear{my_address, fg_address, server}
	}

	fn reset(&self) -> EnvResult<State> {
        loop{
            let incoming = self.server.receive();

            match incoming {
                Ok(message) => return EnvResult::Some(State::from_str(&message).unwrap()),
                _ => return EnvResult::Done,
            }
        }
	}

	fn step(&mut self, paction: PossibleAction<Action>) -> EnvResult<State> {
        
        match paction {
            PossibleAction::Some(action) => self.server.send(&action.to_string(), &self.fg_address).unwrap(),
            PossibleAction::None => (),
        }
        

        loop{
            let incoming = self.server.receive();

            match incoming {
                Ok(message) => return EnvResult::Some(State::from_str(&message).unwrap()),
                _ => return EnvResult::Done,
            }
        }
	}
}



pub struct Agent<'a, T: Environment>{
    env: &'a mut T,
}

impl<'a, T> Agent<'a, T>
    where T: Environment {
    pub fn init(env: &'a mut T) -> Self {
        Agent{env: env}
    }

    pub fn run<F>(&mut self, mut f: F) where
        F: FnMut(&State, &mut PossibleAction<Action>) -> PossibleAction<Action>{
        
        let result = self.env.reset();
        let mut state = match result {
            EnvResult::Some(state) => state,
            _ => return
            // EnvResult::Done => println!("DONE!"),
            // EnvResult::Error => println!("Error"),
        };
        
        let mut paction = PossibleAction::Some(Action::new());

        loop {
            paction = f(&state, &mut paction);

            // let action = match paction {
            //     PossibleAction::Some(action) => action,
            //     PossibleAction::None => _,
            // }

            // let action = Action{aileron: 0.0, elevator: 0.0, rudder: 0.0};
            let result = self.env.step(paction);
            state = match result {
                EnvResult::Some(state) => state,
                EnvResult::Done => break,
                EnvResult::Error => break,
            };
            
        }
    }
}
