use seahorse::{error::FlagError, App, Command, Context, Flag, FlagType};
use std::env;

#[derive(Debug, Hash)]

struct ProjectStruct {
    name: String,
    description: String,
}

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
        .usage("lmt add --type=<type> --name=<name> --description=<description>")
        .action(add_action)
        .flag(
            Flag::new("type", FlagType::String)
                .description("type ex. --type=project|service || -t=project|service")
                .alias("t"),
        )
        .flag(
            Flag::new("name", FlagType::String)
                .description("name ex. --name=project_name || -n=project_name")
                .alias("n"),
        )
        .flag(
            Flag::new("description", FlagType::String)
                .description(
                    "description ex. --description=project_description || -d=project_description",
                )
                .alias("d"),
        )
}

fn add_action(c: &Context) {
    match c.string_flag("type") {
        Ok(t) => {
            match &*t {
                "project" => {
                    let project: ProjectStruct = ProjectStruct {
                        name: c.string_flag("name").unwrap(),
                        description: c.string_flag("description").unwrap(),
                    };

                    print!("{:?}", project.name);
                    print!("{:?}", project.description);
                }
                "service" => println!("Hello adding service, {:?}", c.string_flag("name")),
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
