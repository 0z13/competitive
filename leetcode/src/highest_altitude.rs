pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut max : i32 = 0;
    let mut current_altitude= 0;
    for i in gain.iter() {
        current_altitude = current_altitude + i;

        if current_altitude > max {
            max = current_altitude;
        }
    }
    max
}

