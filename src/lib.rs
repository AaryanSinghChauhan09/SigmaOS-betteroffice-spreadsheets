use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CollabSheet { cells: HashMap<(u32, u32), f64> }
impl CollabSheet {
    pub fn new() -> Self { Self::default() }
    pub fn set(&mut self, r: u32, c: u32, val: f64) { self.cells.insert((r, c), val); }
    pub fn get(&self, r: u32, c: u32) -> Option<f64> { self.cells.get(&(r, c)).copied() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_sheet() {
        let mut s = CollabSheet::new();
        s.set(0, 0, 42.0);
        assert_eq!(s.get(0, 0), Some(42.0));
    }
}
