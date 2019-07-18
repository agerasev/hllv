mod cmd;
mod lib;

use std::{env, io, process::exit};
use indoc::indoc;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cmds: Vec<Box<dyn cmd::Cmd>> = vec![
        Box::new(cmd::New::new()),
        Box::new(cmd::Init::new()),
        Box::new(cmd::Listen::new()),
        Box::new(cmd::Push::new()),
        Box::new(cmd::Pull::new()),
    ];

    args.get(1).ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("No commands found, use 'hllv help' to list available commands"),
    ))
    .and_then(|name| {
        match name.as_str() {
            "help" | "--help" | "-h" => {
                println!("{}{}", indoc!("
                    Available commands:
                    help or --help or -h - shows this message
                "), cmds.iter().map(|cmd| cmd.help()).collect::<Vec<_>>().join(""));
                Ok(())
            },
            name => {
                let mut res = None::<io::Result<()>>;
                for cmd in cmds {
                    if cmd.name() == name {
                        res = Some(cmd.run(&args[2..]));
                        break;
                    }
                }
                res.unwrap_or(Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown command '{}', use 'hllv help' to list available commands", name),
                )))
            }
        }
    })
    .unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    })
}
