use std::io;

fn main() {
    println!("Welcome into the SUUS Shell.");
    println!("Type anything you want. It may not work.");
    main_loop();
}

fn main_loop() {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    match line.trim_matches('\n') {
        "exit" => exit_loop(),
        command => not_found(command)
    }
}

fn exit_loop() {
    println!("Goodbye !!");
}

fn not_found(command: &str) {
    println!("Command {} not found", command);
    main_loop()
}
