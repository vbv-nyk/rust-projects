struct User<T> {
    name: String,
    age: T,
}

impl<T> User<T> {
    fn get_name<T1>(secondUser: T1) -> T1 {
        secondUser
    }
}

impl1 User<i32> {}

fn main() {
    println!("Hello, world!");
}
