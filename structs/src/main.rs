//tuple structs

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    username: String,
    active: bool,
}

impl User {
    fn print_details(&self) {
        print!(
            "The user details are, {} {} {} {}",
            self.name, self.age, self.username, self.active,
        );
    }

    fn change_active(&mut self) {
        self.active = !self.active
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn age(&self) -> u8 {
        self.age
    }

    fn active(&self) -> bool {
        self.active
    }

    fn username(&self) -> String {
        self.username.clone()
    }

    fn check_duplicate(&self, user2: User) -> bool {
        self.username == user2.username
    }

    fn build_user(name: String, age: u8, username: String) -> User {
        User {
            name,
            age,
            username,
            active: true,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let user = User {
        name: String::from("Vaibhav"),
        age: 12,
        username: String::from("vbvnyk"),
        active: true,
    };
    print!("{}", user.name);

    let mut new_user = User::build_user(String::from("Vaibhav"), 19, String::from("vbvnyk"));
    new_user.name = user.name;
    // print!("{:#?}", new_user);
    new_user.print_details();
    new_user.change_active();

    let color = (1, 2, 3);
    print!("{}{}{}", color.0, color.1, color.2);

    struct Color(i32, i32, i32);

    let color = Color(1, 2, 3);

    print!("{}", new_user.name());

    let second_user = User {
        name: String::from("Vaibhav"),
        age: 12,
        username: String::from("vbvnyk"),
        active: true,
    };
    if new_user.check_duplicate(second_user) {
        print!("Same Users");
    }
}
