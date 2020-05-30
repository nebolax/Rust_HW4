use crate::Figures::Ferzy::Ferzy;

pub trait Figure {
    fn danger_zone(&self) -> Vec<(i32, i32)>;
}