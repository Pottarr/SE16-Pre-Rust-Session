fn main() {
    for j in  2..13 {
        println!("Table {j}");
        for i in 1..11 {
            println!("{j} * {i} = {}", j*i);
        }
    } 
}