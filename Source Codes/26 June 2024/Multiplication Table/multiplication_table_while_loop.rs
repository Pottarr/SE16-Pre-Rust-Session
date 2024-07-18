fn main() {
    let mut j = 2;
    while j <= 12 {
        println!("Table {j}");
        let mut i = 1;
        while i <= 10 {
            println!("{j} * {i} = {}", j*i);
            i += 1;
        }
        j += 1;
    }
}