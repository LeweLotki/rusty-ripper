pub mod dictionary;
pub mod hasher;
pub mod passwords;

pub trait ContentManager {
    fn load(&mut self) -> ();
    fn display(&self) -> ();
}
