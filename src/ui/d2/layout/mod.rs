mod column;
mod fill;
mod grid;
mod row;
mod split;

pub use column::Column2D;
pub use fill::Fill2D;
pub use grid::Grid2D;
pub use row::Row2D;
pub use split::Split2D;

use crate::ui::layout::Layout;

pub trait Layout2D: Layout {}
