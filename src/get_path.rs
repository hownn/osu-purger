use std::io;

pub fn get_path() -> String {
    let mut path = String::new();

    println!("Input the path to your osu! song directory:");
    // user inputs path to song directory
    io::stdin() 
        .read_line(&mut path)
        .expect("Invalid path.");
    // checks if path containts osu!\Songs as that's the standard directory
    if !path.as_str().contains("osu!\\Songs") {
        println!("\n'{}' doesn't contain 'osu!\\Songs\', are you sure this is the correct path?\ny/n", path.trim());
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Invalid response");

        // panics if user indicates it's the wrong path or does an invalid input
        match response.as_str().trim() {
            "y" => path,
            "n" => panic!("Wrong path"),
            _ => panic!("Invalid input"),
        }
    } else {
        path
    }
}
