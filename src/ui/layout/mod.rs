pub mod column;
pub mod fill;
pub mod grid;
pub mod row;
pub mod split;

pub trait Layout {
    fn build(&self);
}
