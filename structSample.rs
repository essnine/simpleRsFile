// a struct with named fields,
// functionally identical to C structs. fields accessed with
// the structName.field syntax
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

// a tuple struct, with components accessed 
// like regular tuple's components
struct Point2D(u32,u32);

// a unit struct, commonly used as markers, as a part of
// Rust's traits feature
struct Unit;

fn main() {
    // classic struct, named fields. order doesn't matter.
    let person = Person {
        name: String::from("Suraj"),
        likes_oranges: true,
        age: 26
    };

    // tuple struct, order does matter.
    let origin = Point2D(0,0);

    // a unit struct.
    let unit = Unit;

    println!("{}",person.name);
    println!("{}",person.likes_oranges);
    println!("{}",person.age);

    enum WebEvent {
        // unit-like
        PageLoad,
        PageUnload,
        // can contain chars and strings
        KeyPress(char),
        Paste(String),
        // or include tuple structs
        Click { x: i64, y: i64},
    }
}