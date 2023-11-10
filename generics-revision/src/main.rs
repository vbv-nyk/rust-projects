fn main() {
    let s2 = String::from("two");
    {
        let s1 = String::from("one");
        let s3 = String::from("three");
        let result = combo(&s1, &s2, &s3);
    }
}

fn combo<'a, 'b, 'c>(s1: &'a str, s2: &'b str, s3: &'c str) -> &'a str {
    // function body
    s2
}
