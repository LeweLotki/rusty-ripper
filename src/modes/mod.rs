pub mod dictionary;
pub mod hashes;

pub trait ContentManager {
    fn load(&mut self) -> ();
    fn display(&self) -> ();
}
