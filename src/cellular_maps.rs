#![feature(rand)]
#![feature(core)]

#![feature(test)]
extern crate test;

use std::rand;
use std::rand::Rng;

#[stable]
pub struct CellularMap {
    width               : u32,
    height              : u32,
    map                 : Vec<u8>
}

#[stable]
pub enum EvolveStrategy {
  Default,  // The standard evolution rules.
  Cleaning, // Aggressive cleaning. Remove a lot of "single" occupied tiles.
}

#[unstable]
impl CellularMap {

    /// Create a new `CellularMap` instance.
    ///
    /// # Arguments
    ///
    /// * `w` - The desired map width.
    /// * `h` - The desired map height.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cellular_maps::CellularMap;
    ///
    /// // Create a 30x30 celular map.
    /// let mut cm = CellularMap::new(30u,30u);
    /// ```
    ///
    #[stable]
    pub fn new(w: u32, h: u32) -> CellularMap {
        let mut arraymap: Vec<u8> = Vec::with_capacity((w*h) as usize);
        for _ in (0..w*h) {
            arraymap.push(0);
        }
        CellularMap { width: w, height: h, map: arraymap }
    }

    /// Get the map width.
    #[stable]
    pub fn get_width(self: &CellularMap) -> u32 {
        self.width
    }

    /// Get the map height.
    #[stable]
    pub fn get_height(self: &CellularMap) -> u32 {
        self.height
    }

    /// Get the element in position `<r,c>`.
    #[stable]
    pub fn get_element(self: &CellularMap, r: u32, c: u32) -> u8 {
        return self.map[self.get_index(r,c)];
    }

    /// Initialize a random `CellularMap`.
    #[unstable]
    pub fn random_fill(self: &mut CellularMap, wall_prob: u32) {
        for index in (0..self.width*self.height) {
            let (c,r) = (index % self.width, index/self.width);
            self.map[index as usize] =
                if self.is_on_border(r,c) { 1 } else
                {
                    let map_middle = self.height / 2;
                    if r == map_middle { 0 } else {
                        let value = rand::thread_rng().gen_range(0,100);
                        if value < wall_prob { 1 } else { 0 }
                    }
                };
        }
    }

    #[unstable]
    pub fn evolve_default(self: &mut CellularMap) {
        self.evolve(EvolveStrategy::Default)
    }

    /// Evolve the `CellularMap` according the automata rules.
    #[unstable]
    pub fn evolve(self: &mut CellularMap, strategy: EvolveStrategy) {
        for r in (0..self.height) {
            for c in (0..self.width) {
                let value = self.place_logic(r,c);
                let index = self.get_index(r,c);
                self.map[index] = value;
            }
        }
    }

    /// Implements the wall evolution automata rules for a given position `<r,c>`.
    fn place_logic(self: &mut CellularMap, r: u32, c: u32) -> u8 {
        let num_wall1 = self.count_adjacent_wall(r,c,1,1);
        let num_wall2 = self.count_adjacent_wall(r,c,2,2);

        let index = self.get_index(r,c);
        if self.map[index] == 1u8 {
            if num_wall1 >= 3 { 1 } else { 0 }
        } else {
            if num_wall1 >= 5 || num_wall2 <= 2 { 1 } else { 0 }
        }
    }

    /// Count the number of walls adjacent to `<r,c>` in a given radius `scopex` - `scopey`.
    fn count_adjacent_wall(self: &mut CellularMap, r: u32, c: u32, scopex: u32, scopey: u32) -> u32 {
        let endx = c + scopex + 1;
        let endy = r + scopey + 1;

        let startx = if scopex > c { 0 }            else { c - scopex };
        let underx = if scopex > c { scopex - c }   else { 0 };

        let starty = if scopey > r { 0 }            else { r - scopey };
        let undery = if scopey > r { scopey - r }   else { 0 };

        let mut wallcounter = underx * (2*scopex+1) + undery * (2*scopey+1) - undery*underx;

        for iy in (starty..endy) {
            for ix in (startx..endx) {
                if (ix != c || iy != r) && self.is_wall(iy,ix) {
                    wallcounter+=1;
                }
            }
        }
        return wallcounter;
    }

    /// Check if a given position `<r,c>` is a wall.
    fn is_wall(self: &CellularMap,  r: u32, c: u32) -> bool {
        let index = self.get_index(r,c);
        self.is_out_of_bound(r,c) ||  self.map[index] == 1
    }

    /// Check if a given position `<r,c>` is out of bound.
    fn is_out_of_bound(self: &CellularMap,  r: u32, c: u32) -> bool {
        c>self.width - 1 || r> self.height - 1
    }

    /// Check if a given position `<r,c>` is on the map border.
    fn is_on_border(self: &CellularMap,  r: u32, c: u32) -> bool {
        c == 0 || r == 0 || c == self.width - 1 || r == self.height - 1
    }

    /// Get the row-major index for the given position.
    fn get_index(self: &CellularMap, r: u32, c: u32) -> usize {
        (c + r*self.width) as usize
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[test]
    fn constructor_test() {
        let cm = CellularMap::new(12us,12us);

        assert!(12us == cm.width);
        assert!(12us == cm.height);
    }

    #[test]
    fn get_element_test() {
        let mut cm = CellularMap::new(12us,12us);
        cm.map[4] = 2u8;
        assert_eq!(2u8, cm.get_element(0,4));
    }

    #[bench]
    fn evolve_bench(b:&mut Bencher) {
        let mut cm = CellularMap::new(30us,30us);
        cm.random_fill(40us);
        b.iter(|| {
            cm.evolve();
        });
    }
}
