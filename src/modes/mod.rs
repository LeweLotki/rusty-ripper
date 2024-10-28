pub mod dictionary;
pub mod hasher;
pub mod passwords;
pub mod retriver;

pub trait ContentManager {
    fn display(&self) -> ();
}
