enum Command {
    Init,
    Add,
    Commit,
    Push,
    Checkout,
    Switch,
    Invalid,
}

impl Command {
    fn new(value: &str) -> Self {
        match value {
            "init" => Command::Init,
            "add" => Command::Add,
            "commit" => Command::Commit,
            "push" => Command::Push,
            "checkout" => Command::Checkout,
            "switch" => Command::Switch,
            _ => Command::Invalid,
        }
    }
}

// identifies which command to invoke & calls associated method
fn main() {
    let cmd = match std::env::args().nth(1) {
        Some(cmd) => dbg!(cmd),
        None => {
            println!("No command given.");
            return;
        }
    };

    let args = parse_args();

    match Command::new(&cmd) {
        Command::Init => cmd_init(args),
        Command::Add => cmd_add(args),
        Command::Commit => cmd_commit(args),
        Command::Push => cmd_push(args),
        Command::Checkout => cmd_checkout(args),
        Command::Switch => cmd_switch(args),
        Command::Invalid => println!("Un-recognized command."),
    }
}

// identifies all additional command line arguments provided
fn parse_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        return args[2..].to_vec();
    }
    return Vec::new();
}

fn cmd_init(_args: Vec<String>) {
    println!("TODO")
}

fn cmd_add(_args: Vec<String>) {
    println!("TODO");
}

fn cmd_commit(_args: Vec<String>) {
    println!("TODO");
}

fn cmd_push(_args: Vec<String>) {
    println!("TODO");
}

fn cmd_switch(_args: Vec<String>) {
    println!("TODO");
}

fn cmd_checkout(_args: Vec<String>) {
    println!("TODO");
}
