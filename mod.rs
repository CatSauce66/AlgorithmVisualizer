use crate::map;

mod d_s;
pub use d_s::DiamondSquareSettings;


#[derive(Clone)]
pub enum Algorithms {
    DiamondSquare(d_s::DiamondSquareSettings),
}

pub trait GenerationAlgorithm {
    fn make_grid(&mut self) -> map::Grid;
    fn setup(&mut self, grid: &mut map::Grid);
    fn run(&mut self, grid: &mut map::Grid);
}