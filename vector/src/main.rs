// struct User {
//     name: String,
//     age: i32,
// }

// enum VectorStorage {
//     number(i8),
//     name(String),
//     user(User),
// }

fn main() {
    let s1 = String::from("Hello");
    let s2 = "World".to_string();
    let s3 = " ".to_string();
    let s4: String = s1 + &s3 + &s2;
    println!("{s4}");

    for char in s4.chars() {
        print!("{char}\t");
    }
}
