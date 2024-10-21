pub fn greet(name: &String, loud: bool) {
    if loud {
        println!("HELLO, {}!", name.to_uppercase());
    } else {
        println!("Hello, {}!", name);
    }
}

