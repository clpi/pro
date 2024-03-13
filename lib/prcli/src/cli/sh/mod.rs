use std::{
    io::{
        self, prelude::*, BufRead, Read, Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock, Write
}, ops::{Deref, DerefMut}};

/// Shell is a struct that contains the standard input, output, and error streams.
#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Hash)]
pub struct Sh<O: Write = Stdout, I: Read = Stdin, E: Write = Stderr> {
    /// Default stdout
    pub out: O,
    /// Default stdin
    pub sin: I,
    /// Default stderr
    pub err: E,
}
impl<'a> Into<StderrLock<'a>> for Sh<> {
    fn into(self) -> StderrLock<'a> {
        self.err.lock()
    }
}
impl<'a> Into<StdinLock<'a>> for Sh<> {
    fn into(self) -> StdinLock<'a> {
        self.sin.lock()
    }
}
impl<'a> Into<StdoutLock<'a>> for Sh<> {
    fn into(self) -> StdoutLock<'a> {
        self.out.lock()
    }
}
impl AsRef<Stdin> for Sh<> {
    fn as_ref(&self) -> &Stdin {
        &self.sin
    }
}
impl AsRef<Stderr> for Sh<> {
    fn as_ref(&self) -> &Stderr {
        &self.err
    }
}
impl AsRef<Stdout> for Sh<> {
    fn as_ref(&self) -> &Stdout {
        &self.out
    }
}
impl<O: Write, I: Read, E: Write> Deref for Sh<O, I, E> {
    type Target = O;
    fn deref(&self) -> &Self::Target {
        &self.out
    }
}
impl<O: Write, I: Read, E: Write> DerefMut for Sh<O, I, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.out
    }
}
impl Default for Sh<Stdout, Stdin, Stderr> {
    fn default() -> Self {
        Self {
            out: std::io::stdout(),
            err: std::io::stderr(),
            sin: std::io::stdin(),
        }
    }
}

impl<O: Write, I: Read, E: Write> Sh<O, I, E> {
    fn new(out: O, sin: I, err: E) -> Self {
        Self { out, sin, err }
    }
}

mod tests {
    use super::*;

    #[test]
    fn works() {
        let s = Sh::default();
        let r = s.err.lock().flush().unwrap();
        println!("{:?}", r);
        let b: &[u8] = &[1, 2, 3].as_slice();
        s.out.lock().write_all(b).unwrap();

    }
}