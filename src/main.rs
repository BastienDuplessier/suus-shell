use std::io;

fn main() {
    println!("Welcome into the SUUS Shell.");
    println!("Type anything you want. It may not work.");
    main_loop();
}

fn main_loop() {
    let (command, args) = parse_input();
    match command.as_str() {
        "exit" => exit_loop(),
        "echo" => echo(args),
        command => not_found(command)
    }
}

fn parse_input() -> (String, String) {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let mut iter = line.trim_matches('\n').splitn(2, " ");
    let command = String::from(iter.next().unwrap());
    let args = match iter.next() {
        Some(x) => x,
        None => ""
    };
    (command, String::from(args))
}

fn exit_loop() {
    println!("Goodbye !!");
}

fn not_found(command: &str) {
    println!("Command {} not found", command);
    main_loop()
}

fn echo(text:String) {
    println!("{}", text);
    main_loop()
}
