use std::env;

fn main() {
    println!("Hello, World!");

    if let Ok(test_echo) = env::var("TEST_ECHO") {
        println!("TEST_ECHO: {}", test_echo);
    }
}
