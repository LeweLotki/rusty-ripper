pub mod dictionary;
pub mod hasher;

pub trait ContentManager {
    fn load(&mut self) -> ();
    fn display(&self) -> ();
}
