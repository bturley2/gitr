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
    // TODO separate argument parsing and command invocation
    parse_args();
}

// identifies commands given by the user, and invokes the appropriate logic
fn parse_args() {
    let err_msg = "Un-recognized command.";

    let cmd = match std::env::args().nth(1) {
        Some(cmd) => dbg!(cmd),
        None => {
            println!("{err_msg}");
            return;
        }
    };

    match Command::new(&cmd) {
        Command::Init => cmd_init(),
        Command::Add => cmd_add(),
        Command::Commit => cmd_commit(),
        Command::Push => cmd_push(),
        Command::Checkout => cmd_checkout(),
        Command::Switch => cmd_switch(),
        Command::Invalid => println!("{err_msg}"),
    }
}

fn cmd_init() {
    println!("TODO")
}

fn cmd_add() {
    println!("TODO");
}

fn cmd_commit() {
    println!("TODO");
}

fn cmd_push() {
    println!("TODO");
}

fn cmd_switch() {
    println!("TODO");
}

fn cmd_checkout() {
    println!("TODO");
}
