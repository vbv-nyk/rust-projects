fn main() {
    // Number of type i32
    let num: i32 = 10;

    // Floating of type f64
    let deci: f64 = 10.00000;

    //tuples, define multiple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    let (x, y, z) = (1, 2, 3);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let a = [0; 1000];
    print!("{:#?}", a);
}
