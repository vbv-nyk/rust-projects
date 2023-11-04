fn second_function(num: i32) -> i32 {
    println!("This is the second function {}", num);
    let result = { num + 10 };
    result
}

fn main() {
    println!("Hello, from function main!");
    let results: i32 = second_function(2);
    println!("{}", results);
}
