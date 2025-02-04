use std::time::Instant;

struct Person {
    name: String,
}

struct Person2<'a> {
    name: &'a String
}

// Update value of name
fn update_name(mut p: Person, name: String) -> Person {
    p.name = name;
    p
}

// Update reference to name (much faster)
fn update_name2<'a>(mut p: Person2<'a>, name: &'a String) -> Person2<'a> {
    p.name = name;
    p
}

fn main() {
    // Update the "name" value on the Person struct
    let bob = String::from("Bob");

    let person1 = Person {
        name: String::new(),
    };

    let now = Instant::now();
    update_name(person1, bob);
    let time = now.elapsed();
    println!("{time:?} to update name to Bob");


    // Now instead of updating the value, simply update the reference
    // This is much faster than the value assignment
    let tom = String::from("Tom");

    let person2 = Person2 {
        name: &String::new(),
    };

    let now = Instant::now();
    update_name2(person2, &tom);
    let time2 = now.elapsed();
    println!("{time2:?} to update name to Tom");
}