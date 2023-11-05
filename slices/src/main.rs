fn main() {
    let mut a = [1, 2, 3, 4, 5, 6];
    let b = &a[1..=3];
    println!("{:?}", b);

    let name = String::from("Vaibhav Nayak");
    let ln = &name[8..=12];
    print!("{ln}");
}
