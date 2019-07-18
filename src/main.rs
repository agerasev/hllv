mod cmd;

#[cfg(not(test))]
mod lib;


use std::{env, io, process::exit};
use indoc::indoc;

fn run(args: &[&str]) -> i32 {
    let cmds: Vec<Box<dyn cmd::Cmd>> = vec![
        Box::new(cmd::New::new()),
        Box::new(cmd::Init::new()),
        Box::new(cmd::Listen::new()),
        Box::new(cmd::Push::new()),
        Box::new(cmd::Pull::new()),
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
                        res = Some(cmd.run(&args[2..]));
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    exit(run(&args_ref));
}

#[cfg(test)]
mod lib {
    use std::{io, env, path::Path, cell::RefCell};

    #[derive(PartialEq, Debug)]
    pub enum Call {
        New(String),
        Init(),
        Listen((String, u16)),
        Push((String, u16), String),
        Pull((String, u16), String),
    }

    thread_local! {
        pub static CALLS: RefCell<Vec<Call>> = RefCell::new(Vec::new());
    }


    pub fn new(cwd: &Path, name: &str) -> io::Result<()> {
        assert_eq!(&env::current_dir()?, cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::New(name.to_string()));
        });
        Ok(())
    }

    pub fn init(cwd: &Path) -> io::Result<()> {
        assert_eq!(&env::current_dir()?, cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Init());
        });
        Ok(())
    }

    pub fn listen(cwd: &Path, addr: (&str, u16)) -> io::Result<()> {
        assert_eq!(&env::current_dir()?, cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Listen((addr.0.to_string(), addr.1)));
        });
        Ok(())
    }

    pub fn push(cwd: &Path, addr: (&str, u16), name: &str) -> io::Result<()> {
        assert_eq!(&env::current_dir()?, cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Push(
                (addr.0.to_string(), addr.1),
                name.to_string(),
            ));
        });
        Ok(())
    }

    pub fn pull(cwd: &Path, addr: (&str, u16), name: &str) -> io::Result<()> {
        assert_eq!(&env::current_dir()?, cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Pull(
                (addr.0.to_string(), addr.1),
                name.to_string(),
            ));
        });
        Ok(())
    }
}

#[cfg(test)]
mod cli_test {
    use super::run;
    use super::lib::{Call, CALLS};
    use super::cmd::{DEFAULT_ADDR, DEFAULT_PORT};

    fn pop() -> Option<Call> {
        CALLS.with(|c| {
            c.borrow_mut().pop()
        })
    }

    #[test]
    fn empty() {
        assert_eq!(run(&["hllv"]), 1);
        assert!(pop().is_none());
    }

    #[test]
    fn help() {
        assert_eq!(run(&["hllv", "help"]), 0);
        assert!(pop().is_none());
        assert_eq!(run(&["hllv", "--help"]), 0);
        assert!(pop().is_none());
        assert_eq!(run(&["hllv", "-h"]), 0);
        assert!(pop().is_none());
    }
    #[test]
    fn help_extra() {
        assert_eq!(run(&["hllv", "help", "extra"]), 1);
        assert!(pop().is_none());
    }

    #[test]
    fn new_name() {
        assert_eq!(run(&["hllv", "new", "name"]), 0);
        assert_eq!(pop().unwrap(), Call::New("name".to_string()));
        assert!(pop().is_none());
    }
    #[test]
    fn new_empty() {
        assert_eq!(run(&["hllv", "new"]), 1);
        assert!(pop().is_none());
    }
    #[test]
    fn new_extra() {
        assert_eq!(run(&["hllv", "new", "name", "extra"]), 1);
        assert!(pop().is_none());
    }

    #[test]
    fn init() {
        assert_eq!(run(&["hllv", "init"]), 0);
        assert_eq!(pop().unwrap(), Call::Init());
        assert!(pop().is_none());
    }
    #[test]
    fn init_extra() {
        assert_eq!(run(&["hllv", "init", "extra"]), 1);
        assert!(pop().is_none());
    }

    #[test]
    fn listen_addr_port() {
        assert_eq!(run(&["hllv", "listen", "localhost:8000"]), 0);
        assert_eq!(pop().unwrap(), Call::Listen(("localhost".to_string(), 8000)));
        assert!(pop().is_none());
    }
    #[test]
    fn listen_addr() {
        assert_eq!(run(&["hllv", "listen", "localhost"]), 0);
        assert_eq!(pop().unwrap(), Call::Listen(("localhost".to_string(), DEFAULT_PORT)));
        assert!(pop().is_none());
    }
    #[test]
    fn listen_default() {
        assert_eq!(run(&["hllv", "listen"]), 0);
        assert_eq!(pop().unwrap(), Call::Listen((DEFAULT_ADDR.to_string(), DEFAULT_PORT)));
        assert!(pop().is_none());
    }
    #[test]
    fn listen_extra() {
        assert_eq!(run(&["hllv", "listen", "localhost:8000", "extra"]), 1);
        assert!(pop().is_none());
    }

    #[test]
    fn push_addr_port() {
        assert_eq!(run(&["hllv", "push", "localhost:8000", "name"]), 0);
        assert_eq!(pop().unwrap(), Call::Push(("localhost".to_string(), 8000), "name".to_string()));
        assert!(pop().is_none());
    }
    #[test]
    fn push_addr() {
        assert_eq!(run(&["hllv", "push", "localhost", "name"]), 0);
        assert_eq!(pop().unwrap(), Call::Push(("localhost".to_string(), DEFAULT_PORT), "name".to_string()));
        assert!(pop().is_none());
    }
    #[test]
    fn push_addr_only() {
        assert_eq!(run(&["hllv", "push", "localhost"]), 1);
        assert!(pop().is_none());
    }
    #[test]
    fn push_empty() {
        assert_eq!(run(&["hllv", "push"]), 1);
        assert!(pop().is_none());
    }
    #[test]
    fn push_extra() {
        assert_eq!(run(&["hllv", "push", "localhost", "name", "extra"]), 1);
        assert!(pop().is_none());
    }

    #[test]
    fn pull_addr_port() {
        assert_eq!(run(&["hllv", "pull", "localhost:8000", "name"]), 0);
        assert_eq!(pop().unwrap(), Call::Pull(("localhost".to_string(), 8000), "name".to_string()));
        assert!(pop().is_none());
    }
    #[test]
    fn pull_addr() {
        assert_eq!(run(&["hllv", "pull", "localhost", "name"]), 0);
        assert_eq!(pop().unwrap(), Call::Pull(("localhost".to_string(), DEFAULT_PORT), "name".to_string()));
        assert!(pop().is_none());
    }
    #[test]
    fn pull_addr_only() {
        assert_eq!(run(&["hllv", "pull", "localhost"]), 1);
        assert!(pop().is_none());
    }
    #[test]
    fn pull_empty() {
        assert_eq!(run(&["hllv", "pull"]), 1);
        assert!(pop().is_none());
    }
    #[test]
    fn pull_extra() {
        assert_eq!(run(&["hllv", "pull", "localhost", "name", "extra"]), 1);
        assert!(pop().is_none());
    }
}
