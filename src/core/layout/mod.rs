mod column;
mod grid;
mod row;

pub trait Layout {
    fn build(&self);
}
