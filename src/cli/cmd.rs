use std::{
    io,
    ops::RangeInclusive,
    path::{Path},
    marker::PhantomData,
};
use indoc::indoc;
use crate::api::{Api};

pub const DEFAULT_ADDR: &'static str = "0.0.0.0";
pub const DEFAULT_PORT: u16 = 4096; 

pub trait Cmd<A: Api> {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn args_count(&self) -> RangeInclusive<usize>;

    fn run_unchecked(&self, cwd: &Path, args: &[&str]) -> io::Result<()>;
    fn run(&self, cwd: &Path, args: &[&str]) -> io::Result<()> {
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
        self.run_unchecked(cwd, args)
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

pub struct New<A: Api> {
    phantom: PhantomData<A>,
}
impl<A: Api> New<A> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}
impl<A: Api> Cmd<A> for New<A> {
    fn name(&self) -> &'static str { "new" }
    fn help(&self) -> &'static str { indoc!("
        new <name> - creates new repository
            <name> - path to the new directory where repository is to be created
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        1..=1
    }
    fn run_unchecked(&self, cwd: &Path, args: &[&str]) -> io::Result<()> {
        A::new(cwd, &Path::new(args[0]))
    }
}

pub struct Init<A: Api> {
    phantom: PhantomData<A>,
}
impl<A: Api> Init<A> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}
impl<A: Api> Cmd<A> for Init<A> {
    fn name(&self) -> &'static str { "init" }
    fn help(&self) -> &'static str { indoc!("
        init - initialize new repository in the current directory
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        0..=0
    }
    fn run_unchecked(&self, cwd: &Path, _args: &[&str]) -> io::Result<()> {
        A::init(cwd)
    }
}

pub struct Listen<A: Api> {
    phantom: PhantomData<A>,
}
impl<A: Api> Listen<A> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}
impl<A: Api> Cmd<A> for Listen<A> {
    fn name(&self) -> &'static str { "listen" }
    fn help(&self) -> &'static str { indoc!("
        listen [addr][:port] - listen for incoming push/pull requests
            [addr][:port] - optional TCP address and port where to listen for incoming connections
                default address is 0.0.0.0, default port is DEFAULT_PORT
    ") }
    fn args_count(&self) -> RangeInclusive<usize> {
        0..=1
    }
    fn run_unchecked(&self, cwd: &Path, args: &[&str]) -> io::Result<()> {
        let addr = match args.get(0) {
            Some(s) => parse_addr(s)?,
            None => (DEFAULT_ADDR, DEFAULT_PORT),
        };
        A::listen(cwd, addr)
    }
}

pub struct Push<A: Api> {
    phantom: PhantomData<A>,
}
impl<A: Api> Push<A> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}
impl<A: Api> Cmd<A> for Push<A> {
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
    fn run_unchecked(&self, cwd: &Path, args: &[&str]) -> io::Result<()> {
        let addr = parse_addr(&args[0])?;
        let name = Path::new(args[1]);
        A::push(cwd, addr, name)
    }
}

pub struct Pull<A: Api> {
    phantom: PhantomData<A>,
}
impl<A: Api> Pull<A> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }
}
impl<A: Api> Cmd<A> for Pull<A> {
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
    fn run_unchecked(&self, cwd: &Path, args: &[&str]) -> io::Result<()> {
        let addr = parse_addr(&args[0])?;
        let name = Path::new(args[1]);
        A::pull(cwd, addr, name)
    }
}
