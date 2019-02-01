use std::f32;

#[derive(Debug)]
pub struct Navigation{
    dest_latitude: f32,
    dest_longitude: f32,
}


impl Navigation{
    pub fn new( dest_latitude: f32, dest_longitude: f32 ) -> Self {
        Navigation { dest_latitude, dest_longitude }
    }

    pub fn get_delta_heading_to_destination(&mut self, new_latitude: f32, new_longitude: f32, heading: f32) -> f32 {
        // delta_lat = self.dest_latitude - gps_latitude
        let delta_latitude = self.dest_latitude - new_latitude;

		// delta_long = self.dest_longitude - gps_longitude 
        let delta_longitude = self.dest_longitude - new_longitude;

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
}
