fn main() {
    let sphere_rad = 1.0;
    let cylinder_rad = 1.0;
    let cylinder_h = 1.0;
    let cone_rad = 1.0;
    let cone_h = 1.0;

    // In this case, we use 3.14 as a value of Pi.
    let sphere_vol = (4.0/3.0)*3.14*sphere_rad*sphere_rad*sphere_rad;
    let cylinder_vol = 3.14*cylinder_rad*cylinder_rad*cylinder_h;
    let cone_vol = (1.0/3.0)*3.14*cone_rad*cone_rad*cone_h;

    println!("The volume of the sphere is {sphere_vol:.1}");
    println!("The volume of the cylinder is {cylinder_vol:.1}");
    println!("The volume of the cone is {cone_vol:.1}");
}