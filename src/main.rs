use std::io;
use std::io::Write;

fn main() {
    println!("Welcome into the SUUS Shell.");
    println!("Type anything you want. It may not work.");
    main_loop();
}

fn main_loop() {
    print_prompt();
    let (command, args) = parse_input();
    match command.as_str() {
        "exit" => exit_loop(),
        command => {
            match command {
                "echo" => echo(args),
                cmd => not_found(cmd)
            };
            main_loop()
        }
    }
}

fn print_prompt() {
    let current_dir = std::env::current_dir().unwrap();
    print!("{}", current_dir.display());
    print!(" |> ");
    io::stdout().flush().ok();
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
}

fn echo(text:String) {
    println!("{}", text);
}
