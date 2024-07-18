fn main() {
    for j in  2..=12 {
        println!("Table {j}");
        for i in 1..=10 {
            println!("{j} * {i} = {}", j*i);
        }
    } 
}