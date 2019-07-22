use std::{
    io,
    path::{Path, PathBuf},
    cell::RefCell,
};
use lazy_static::lazy_static;
use crate::{
    api::{Api},
    cli::{self, cmd::{DEFAULT_ADDR, DEFAULT_PORT}},
};

lazy_static! {
    static ref CWD: PathBuf = PathBuf::from("cwd");
}

#[derive(PartialEq, Debug)]
pub enum Call {
    Init(),
    New(PathBuf),
    Listen((String, u16)),
    Push((String, u16), PathBuf),
    Pull((String, u16), PathBuf),
}


thread_local! {
    pub static CALLS: RefCell<Vec<Call>> = RefCell::new(Vec::new());
}


enum TestApi {}
impl Api for TestApi {
    fn new(cwd: &Path, name: &Path) -> io::Result<()> {
        assert_eq!(CWD.as_path(), cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::New(name.to_path_buf()));
        });
        Ok(())
    }
    fn init(cwd: &Path) -> io::Result<()> {
        assert_eq!(CWD.as_path(), cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Init());
        });
        Ok(())
    }

    fn listen(cwd: &Path, addr: (&str, u16)) -> io::Result<()> {
        assert_eq!(CWD.as_path(), cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Listen((addr.0.to_string(), addr.1)));
        });
        Ok(())
    }
    fn push(cwd: &Path, addr: (&str, u16), name: &Path) -> io::Result<()> {
        assert_eq!(CWD.as_path(), cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Push(
                (addr.0.to_string(), addr.1),
                name.to_path_buf(),
            ));
        });
        Ok(())
    }
    fn pull(cwd: &Path, addr: (&str, u16), name: &Path) -> io::Result<()> {
        assert_eq!(CWD.as_path(), cwd);
        CALLS.with(|c| {
            c.borrow_mut().push(Call::Pull(
                (addr.0.to_string(), addr.1),
                name.to_path_buf(),
            ));
        });
        Ok(())
    }
}


fn pop() -> Option<Call> {
    CALLS.with(|c| {
        c.borrow_mut().pop()
    })
}

#[test]
fn empty() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv"]), 1);
    assert!(pop().is_none());
}

#[test]
fn help() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "help"]), 0);
    assert!(pop().is_none());
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "--help"]), 0);
    assert!(pop().is_none());
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "-h"]), 0);
    assert!(pop().is_none());
}
#[test]
fn help_extra() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "help", "extra"]), 1);
    assert!(pop().is_none());
}

#[test]
fn new_name() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "new", "name"]), 0);
    assert_eq!(pop().unwrap(), Call::New(PathBuf::from("name")));
    assert!(pop().is_none());
}
#[test]
fn new_empty() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "new"]), 1);
    assert!(pop().is_none());
}
#[test]
fn new_extra() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "new", "name", "extra"]), 1);
    assert!(pop().is_none());
}

#[test]
fn init() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "init"]), 0);
    assert_eq!(pop().unwrap(), Call::Init());
    assert!(pop().is_none());
}
#[test]
fn init_extra() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "init", "extra"]), 1);
    assert!(pop().is_none());
}

#[test]
fn listen_addr_port() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "listen", "localhost:8000"]), 0);
    assert_eq!(pop().unwrap(), Call::Listen(("localhost".to_string(), 8000)));
    assert!(pop().is_none());
}
#[test]
fn listen_addr() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "listen", "localhost"]), 0);
    assert_eq!(pop().unwrap(), Call::Listen(("localhost".to_string(), DEFAULT_PORT)));
    assert!(pop().is_none());
}
#[test]
fn listen_default() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "listen"]), 0);
    assert_eq!(pop().unwrap(), Call::Listen((DEFAULT_ADDR.to_string(), DEFAULT_PORT)));
    assert!(pop().is_none());
}
#[test]
fn listen_extra() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "listen", "localhost:8000", "extra"]), 1);
    assert!(pop().is_none());
}
#[test]
fn listen_bad_port() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "listen", "localhost:bad"]), 1);
    assert!(pop().is_none());
}

#[test]
fn push_addr_port() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "push", "localhost:8000", "name"]), 0);
    assert_eq!(pop().unwrap(), Call::Push(("localhost".to_string(), 8000), PathBuf::from("name")));
    assert!(pop().is_none());
}
#[test]
fn push_addr() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "push", "localhost", "name"]), 0);
    assert_eq!(pop().unwrap(), Call::Push(("localhost".to_string(), DEFAULT_PORT), PathBuf::from("name")));
    assert!(pop().is_none());
}
#[test]
fn push_addr_only() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "push", "localhost"]), 1);
    assert!(pop().is_none());
}
#[test]
fn push_empty() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "push"]), 1);
    assert!(pop().is_none());
}
#[test]
fn push_extra() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "push", "localhost", "name", "extra"]), 1);
    assert!(pop().is_none());
}

#[test]
fn pull_addr_port() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "pull", "localhost:8000", "name"]), 0);
    assert_eq!(pop().unwrap(), Call::Pull(("localhost".to_string(), 8000), PathBuf::from("name")));
    assert!(pop().is_none());
}
#[test]
fn pull_addr() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "pull", "localhost", "name"]), 0);
    assert_eq!(pop().unwrap(), Call::Pull(("localhost".to_string(), DEFAULT_PORT), PathBuf::from("name")));
    assert!(pop().is_none());
}
#[test]
fn pull_addr_only() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "pull", "localhost"]), 1);
    assert!(pop().is_none());
}
#[test]
fn pull_empty() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "pull"]), 1);
    assert!(pop().is_none());
}
#[test]
fn pull_extra() {
    assert_eq!(cli::run::<TestApi>(&CWD, &["hllv", "pull", "localhost", "name", "extra"]), 1);
    assert!(pop().is_none());
}
