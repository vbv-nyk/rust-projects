use std::collections::HashMap;

fn main() {
    let mut inventory: HashMap<String, i32> = HashMap::new();

    let items = vec![
        "Candy".to_string(),
        "Candy".to_string(),
        "Chocolate".to_string(),
        "Vanilla".to_string(),
        "Vanilla".to_string(),
        "Chocolate".to_string(),
    ];

    for item in items {
        let count = inventory.entry(item).or_insert(0);
        *count += 1;
    }

    for (item, count) in inventory {
        println!("You have {count} number of {item}");
    }
}
