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
    let name1 = String::from("Bob");

    let p1 = Person {
        name: String::new(),
    };

    let now = Instant::now();
    update_name(p1, name1);
    let time = now.elapsed();
    println!("{time:?} to update name to Bob");



    let name12 = String::from("Tom");

    let p12 = Person2 {
        name: &String::new(),
    };

    let now = Instant::now();
    update_name2(p12, &name12);
    let time2 = now.elapsed();
    println!("{time2:?} to update name to Tom");
}