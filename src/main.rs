use seahorse::{error::FlagError, App, Command, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("lmt [args]")
        .action(default_action)
        .command(add_command());

    app.run(args);
}

fn default_action(c: &Context) {
    println!("Hello, {:?}", c.args);
}

fn add_command() -> Command {
    Command::new("add")
        .description("add project or service to the system")
        .alias("a")
        .usage("lmt add [args]")
        .action(add_action)
        .flag(
            Flag::new("type", FlagType::String)
                .description("type flag(ex. lmt add --type=project|service)")
                .alias("t"),
        )
}

fn add_action(c: &Context) {
    match c.string_flag("type") {
        Ok(t) => {
            match &*t {
                "project" => println!("add project"),
                "service" => println!("add service"),
                _ => panic!("undefined territory..."),
            };
        }

        Err(e) => match e {
            FlagError::Undefined => panic!("undefined territory..."),
            FlagError::ArgumentError => panic!("argument error..."),
            FlagError::NotFound => panic!("not found flag..."),
            FlagError::ValueTypeError => panic!("value type mismatch..."),
            FlagError::TypeError => panic!("flag type mismatch..."),
        },
    }
}
