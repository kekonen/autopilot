use std::f32;

#[derive(Debug, PartialEq)]
pub struct Location{
    latitude: f32,
    longitude: f32,
}

impl Location{
    pub fn new(latitude: f32, longitude: f32) -> Self{
        Location {latitude, longitude}
    }

    pub fn distance_to(&self, other: &Location) -> f32 {
        let d2r = f32::consts::PI / 180.0;

        let dlong = (self.longitude - other.longitude) * d2r;
        let dlat = (self.latitude - other.latitude) * d2r;

        let a = (dlat/2.0).sin().powi(2) +  (other.latitude * d2r).cos() * (self.latitude * d2r).cos() * (dlong/2.0).sin().powi(2);
        let c = 2.0 * (a.sqrt().atan2(1.0 - a));

        6367.0 * c
    }
}


#[derive(Debug)]
pub struct Navigation{
    dest_location: Location,
}


impl Navigation{
    pub fn new( ) -> Self {
        Navigation { dest_location: Location::new(0.0, 0.0)}
    }

    pub fn get_delta_heading_to_destination(&mut self, current_location: &Location, heading: f32) -> f32 {
        // delta_lat = self.dest_latitude - gps_latitude
        let delta_latitude = self.dest_location.latitude - current_location.latitude;

		// delta_long = self.dest_longitude - gps_longitude 
        let delta_longitude = self.dest_location.longitude - current_location.longitude;

		// rad = math.atan2(delta_lat, delta_long)

        let rad = delta_latitude.atan2(delta_longitude);

		// gps_heading = rad * 180 / math.pi
        let gps_heading = rad * 180.0 / f32::consts::PI;

		// destination_heading = (450 - int(gps_heading)) % 360 # degrees
        let destination_heading = (450.0 - gps_heading) % 360.0;

		// delta_destination_heading = destination_heading - heading
        let mut delta_destination_heading = destination_heading - heading;

		// # print(delta_destination_heading)
		// if delta_destination_heading >  180: delta_destination_heading -=360
        // if delta_destination_heading < -180: delta_destination_heading +=360
        if delta_destination_heading > 180.0 { delta_destination_heading -= 360.0 }
        if delta_destination_heading < -180.0 { delta_destination_heading += 360.0 }

        delta_destination_heading
    }

    pub fn get_dest_location(&mut self) -> &Location{
        &self.dest_location
    }

    pub fn set_dest_location(&mut self, new_location: Location) {
        self.dest_location = new_location;
    }
}
