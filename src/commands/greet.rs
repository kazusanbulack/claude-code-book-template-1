pub fn handle(name: Option<&str>) {
    match name {
        Some(name) => println!("Hello, {}! Welcome to DevTask! 👋", name),
        None => println!("Hello from DevTask! 👋"),
    }
}