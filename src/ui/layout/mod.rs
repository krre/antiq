mod column;
mod grid;
mod row;
mod split;

pub trait Layout {
    fn build(&self);
}
