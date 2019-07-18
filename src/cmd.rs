use std::{io, env, ops::RangeInclusive};
use indoc::indoc;
use crate::{lib as hllv};

pub const DEFAULT_ADDR: &'static str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 4096; 

pub trait Cmd {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn args_count(&self) -> RangeInclusive<usize>;

    fn run_unchecked(&self, args: &[&str]) -> io::Result<()>;
    fn run(&self, args: &[&str]) -> io::Result<()> {
        if args.len() < *self.args_count().start() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{}: too few arguments", self.name()),
            ));
        }
        if args.len() > *self.args_count().end() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("{}: too many arguments", self.name()),
            ));
        }
        self.run_unchecked(args)
    }
}

fn parse_addr(s: &str) -> io::Result<(&str, u16)> {
    let ss = s.rsplitn(2, ':').collect::<Vec<_>>();
    let mut rs = ss.iter().rev();
    let addr = rs.next().unwrap();
    let port = match rs.next() {
        Some(ps) => ps.parse::<u16>().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e)
        }),
        None => Ok(DEFAULT_PORT),
    }?;
    Ok((addr, port))
}

pub struct New {

}
impl New {
    pub fn new() -> Self {
        Self {}
    }
}
impl Cmd for New {
    fn name(&self) -> &'static str { "new" }
    fn help(&self) -> &'static str { indoc!("
        new <name> - creates new repository
            <name> - path to the new directory where repository is to be created
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        1..=1
    }
    fn run_unchecked(&self, args: &[&str]) -> io::Result<()> {
        hllv::new(&env::current_dir()?, args[0])
    }
}

pub struct Init {

}
impl Init {
    pub fn new() -> Self {
        Self {}
    }
}
impl Cmd for Init {
    fn name(&self) -> &'static str { "init" }
    fn help(&self) -> &'static str { indoc!("
        init - initialize new repository in the current directory
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        0..=0
    }
    fn run_unchecked(&self, _args: &[&str]) -> io::Result<()> {
        hllv::init(&env::current_dir()?)
    }
}

pub struct Listen {

}
impl Listen {
    pub fn new() -> Self {
        Self {}
    }
}
impl Cmd for Listen {
    fn name(&self) -> &'static str { "listen" }
    fn help(&self) -> &'static str { indoc!("
        listen [addr][:port] - listen for incoming push/pull requests
            [addr][:port] - optional TCP address and port where to listen for incoming connections
                default address is 0.0.0.0, default port is DEFAULT_PORT
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        0..=1
    }
    fn run_unchecked(&self, args: &[&str]) -> io::Result<()> {
        let addr = match args.get(0) {
            Some(s) => parse_addr(s)?,
            None => (DEFAULT_ADDR, DEFAULT_PORT),
        };
        hllv::listen(&env::current_dir()?, addr)
    }
}

pub struct Push {

}
impl Push {
    pub fn new() -> Self {
        Self {}
    }
}
impl Cmd for Push {
    fn name(&self) -> &'static str { "push" }
    fn help(&self) -> &'static str { indoc!("
        push <addr>[:port] <name> - update remote repository with local changes
            <addr>[:port] - TCP address and optional port to connect to remote listener
                default port is DEFAULT_PORT
            <name> - path to remote repository is relative to its listener
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        2..=2
    }
    fn run_unchecked(&self, args: &[&str]) -> io::Result<()> {
        let addr = parse_addr(&args[0])?;
        let name = args[1];
        hllv::push(&env::current_dir()?, addr, name)
    }
}

pub struct Pull {

}
impl Pull {
    pub fn new() -> Self {
        Self {}
    }
}
impl Cmd for Pull {
    fn name(&self) -> &'static str { "pull" }
    fn help(&self) -> &'static str { indoc!("
        pull <addr>[:port] <name> - retrieve updates from remote repository
            <addr>[:port] - TCP address and optional port to connect to remote listener
                default port is DEFAULT_PORT
            <name> - path to remote repository is relative to its listener
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        2..=2
    }
    fn run_unchecked(&self, args: &[&str]) -> io::Result<()> {
        let addr = parse_addr(&args[0])?;
        let name = args[1];
        hllv::pull(&env::current_dir()?, addr, name)
    }
}
