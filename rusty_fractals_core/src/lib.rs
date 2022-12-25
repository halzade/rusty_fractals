mod engine;
mod machine;
pub mod fractal;
pub mod fractal_stats;
mod target;
pub mod mathematician;
pub mod mem;
pub mod mem_collatz;
pub mod mem_phoenix;
mod log;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
