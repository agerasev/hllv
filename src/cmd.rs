use std::io;

use indoc::indoc;

pub trait Cmd {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn run(&self, args: &[String]) -> io::Result<()>;
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
    fn run(&self, args: &[String]) -> io::Result<()> {
        unimplemented!();
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
    fn run(&self, args: &[String]) -> io::Result<()> {
        unimplemented!();
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
                default address is 0.0.0.0, default port is 4096
    ") }
    fn run(&self, args: &[String]) -> io::Result<()> {
        unimplemented!();
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
                default port is 4096
            <name> - path to remote repository is relative to its listener
    ") }
    fn run(&self, args: &[String]) -> io::Result<()> {
        unimplemented!();
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
                default port is 4096
            <name> - path to remote repository is relative to its listener
    ") }
    fn run(&self, args: &[String]) -> io::Result<()> {
        unimplemented!();
    }
}
