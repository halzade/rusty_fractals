use crate::calc::CalculationType::StaticImage;
use crate::calc::OrbitType::Infinite;

#[derive(PartialEq)]
pub enum OrbitType {
    Finite,
    Infinite,
}

#[derive(PartialEq)]
pub enum CalculationType {
    StaticImage,
    InfiniteVideoZoom,
}

pub struct CalculationConfig {
    pub calc_type: CalculationType,
    pub orbits: OrbitType, // fractal::finite_orbits / infinite_orbits
    pub update_max: u32,
    pub update_min: u32,
}

pub fn init_trivial() -> CalculationConfig {
    CalculationConfig {
        calc_type: StaticImage,
        orbits: Infinite,
        update_max: 10,
        update_min: 1,
    }
}
