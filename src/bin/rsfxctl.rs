fn main() {
    // rfsctl takes args

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No command provided.");
        eprintln!("Use 'rsfxctl --help' for usage information.");
        std::process::exit(1);
    }
    let command = &args[1];
    let pause_file = "/tmp/rsfx_paused";
    
    match command.as_str()  {
        "--help" | "help" | "-h" => {
            print_help();
        }
        "pause" => {
            std::fs::write(pause_file, "").expect("Failed to create pause file");
            println!("Paused.");
        }
        "unpause" => {
            if std::fs::remove_file(pause_file).is_ok() {
                println!("Unpaused.");
            } else {
                println!("Already unpaused.");
            }
        }
        "toggle" => {
            if std::path::Path::new(pause_file).exists() {
                std::fs::remove_file(pause_file).expect("Failed to remove pause file");
                println!("Unpaused.");
            } else {
                std::fs::write(pause_file, "").expect("Failed to create pause file");
                println!("Paused.");   
            }
        }
        "status" => {
            if std::path::Path::new(pause_file).exists() {
                println!("paused.");
            } else {
                println!("unpaused.");
            }
        }
        _ => {
            eprintln!("Error: Unknown command '{}'.", command);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("Usage: rsfxctl <command>");
    println!("Commands:");
    println!("  pause     - Pause the sound effects");
    println!("  unpause   - Unpause the sound effects");
    println!("  toggle    - Toggle the paused state");
    println!("  status    - Show the current paused state");
}