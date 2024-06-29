fn main() {
    let speed = 120;
    let min_speed = 40;
    let max_speed = 120;

    if speed < min_speed {
        println!("you are driving below speed limit by {} km/hr", min_speed-speed);
    } else if { speed <= min_speed || speed >= max_speed } {
        println!("you are driving within the speed limit");
    } else {
        println!("you are driving above speed limit by {} km/hr", speed-max_speed);
    }
}