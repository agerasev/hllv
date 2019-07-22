pub trait Manager {
    fn listen<A: ToSocketAddrs>(&mut self, addr: A, path: &Path) -> io::Result<()>;
}
