pub enum OrbitType {
    Finite,
    Infinite,
}

pub struct CalculationConfig {
    pub orbits: OrbitType, // fractal::finite_orbits / infinite_orbits
    pub update_max: u32,
    pub update_min: u32,
}
