fn main() {
    let height = 6;
    for i in 1..=height {
        for _j in 1..=(height-i) {
            print!(" ");
        }
        for _k in 1..=i {
            print!("*");
        }
        println!("");
    }
}