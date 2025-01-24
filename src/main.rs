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

fn main() {
    parse_args();
    // need method for identifying commands from CLI
}

// identifies commands given by the user, and invokes the appropriate logic
fn parse_args() {
    //let cmd = std::env::args().nth(1).expect("No command given!").as_str();
    let cmd = String::from("init");

    match Command::new(&cmd) {
        Command::Init => cmd_init(),
        Command::Add => cmd_add(),
        Command::Commit => cmd_commit(),
        Command::Push => cmd_push(),
        Command::Checkout => cmd_checkout(),
        Command::Switch => cmd_switch(),
        Command::Invalid => println!("Un-recognized command."),
    }
}

fn cmd_init() {}

fn cmd_add() {}

fn cmd_commit() {}

fn cmd_push() {}

fn cmd_switch() {}

fn cmd_checkout() {}
