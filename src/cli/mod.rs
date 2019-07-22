mod cmd;
#[cfg(test)]
mod test;

use std::{
    io,
    path::{Path},
};
use indoc::indoc;
use crate::api::{Api};

pub fn run<A: Api>(cwd: &Path, args: &[&str]) -> i32 {
    let cmds: Vec<Box<dyn cmd::Cmd<A>>> = vec![
        Box::new(cmd::New::<A>::new()),
        Box::new(cmd::Init::<A>::new()),
        Box::new(cmd::Listen::<A>::new()),
        Box::new(cmd::Push::<A>::new()),
        Box::new(cmd::Pull::<A>::new()),
    ];

    args.get(1).ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("No commands found, use '{} help' to list available commands", args[0]),
    ))
    .and_then(|name| {
        match *name {
            "help" | "--help" | "-h" => {
                if args.len() == 2 {
                    println!("{}{}", indoc!("
                        Available commands:
                        help or --help or -h - shows this message
                    "), cmds.iter().map(|cmd| cmd.help()).collect::<Vec<_>>().join(""));
                    Ok(())
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("Unexpected argument for 'help' command"),
                    ))
                }
            },
            name => {
                let mut res = None::<io::Result<()>>;
                for cmd in cmds {
                    if cmd.name() == name {
                        res = Some(cmd.run(&cwd, &args[2..]));
                        break;
                    }
                }
                res.unwrap_or(Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unknown command '{}', use '{} help' to list available commands", name, args[0]),
                )))
            }
        }
    })
    .map(|()| 0)
    .unwrap_or_else(|e| {
        eprintln!("{}", e);
        1
    })
}
