#![feature(globs)]

extern crate test;

use std::rand;
use std::rand::Rng;

#[stable]
pub struct CellularMap<'a> {
    width               : uint,
    height              : uint,
    map                 : Vec<u8>
}

#[unstable]
impl <'a> CellularMap<'a> {

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
    pub fn new(w: uint, h: uint) -> CellularMap<'a> {
        let mut arraymap: Vec<u8> = Vec::with_capacity(w*h);
        for _ in range(0,w*h) {
            arraymap.push(0);
        }
        CellularMap { width: w, height: h, map: arraymap }
    }

    /// Get the map width.
    #[stable]
    pub fn get_width(self: &CellularMap<'a>) -> uint {
        self.width
    }

    /// Get the map height.
    #[stable]
    pub fn get_height(self: &CellularMap<'a>) -> uint {
        self.height
    }

    /// Get the element in position `<r,c>`.
    #[stable]
    pub fn get_element(self: &CellularMap<'a>, r: uint, c: uint) -> u8 {
        return self.map[self.get_index(r,c)];
    }

    /// Initialize a random `CellularMap`.
    #[unstable]
    pub fn random_fill(self: &mut CellularMap<'a>, wall_prob: uint) {
        let mut rng = rand::thread_rng();

        for c in range(0u,self.width) {
            for r in range(0u,self.height) {
                let index = self.get_index(r,c);
                if self.is_on_border(r,c) {
                    self.map[index] = 1;
                } else {
                    let map_middle = self.height / 2;

                    if r == map_middle {
                        self.map[index] = 0;
                    } else {
                        let value = rng.gen_range(0u,100u);
                        if  value < wall_prob {
                            self.map[index] = 1;
                        }
                    }
                }
            }
        }
    }

    /// Evolve the `CellularMap` according the automata rules.
    #[unstable]
    pub fn evolve(self: &mut CellularMap<'a>) {
        for r in range(0u,self.height) {
            for c in range(0u, self.width) {
                let value = self.place_logic(r,c);
                let index = self.get_index(r,c);
                self.map[index] = value;
            }
        }
    }

    /// Implements the wall evolution automata rules for a given position `<r,c>`.
    fn place_logic(self: &mut CellularMap<'a>, r: uint, c: uint) -> u8 {
        let num_wall = self.count_adjacent_wall(r,c,1,1);
        let num_wall2 = self.count_adjacent_wall(r,c,2,2);
        //println!("{} {} num {}",x,y,num_wall);

        let index = self.get_index(r,c);
        if self.map[index] == 1u8 {
            //println!("WALL");
            if num_wall >= 3 {
                return 1;
            } else {
                return 0;
            }
        } else {
            //println!("NOPE");
            if num_wall >= 5 || num_wall2 <= 2 {
                return 1;
            }
        }
        return 0;
    }

    /// Count the number of walls adjacent to `<r,c>` in a given radius `scopex` - `scopey`.
    fn count_adjacent_wall(self: &mut CellularMap<'a>, r: uint, c: uint, scopex: uint, scopey: uint) -> uint {
        let startx : uint;
        let starty : uint;
        let endx = c + scopex + 1;
        let endy = r + scopey + 1;
        let mut wallcounter : uint;
        let mut underx = 0u;
        let mut undery = 0u;

        if  scopex > c {
            startx = 0;
            underx = scopex - c;
        } else {
            startx = c - scopex;
        }

        if  scopey > r {
            starty = 0;
            undery = scopey - r;
        } else {
            starty = r - scopey;
        }
        wallcounter = underx * (2*scopex+1) + undery * (2*scopey+1) - undery*underx;
        
        for iy in range(starty,endy) {
            for ix in range(startx,endx) {
                if ix != c || iy != r {
                    if self.is_wall(iy,ix) {
                    wallcounter+=1;
                    }
                }
            }
        }
        return wallcounter;
    }

    /// Check if a given position `<r,c>` is a wall.
    fn is_wall(self: &CellularMap<'a>,  r: uint, c: uint) -> bool {
        let index = self.get_index(r,c);
        self.is_out_of_bound(r,c) ||  self.map[index] == 1
    }

    /// Check if a given position `<r,c>` is out of bound.
    fn is_out_of_bound(self: &CellularMap<'a>,  r: uint, c: uint) -> bool {
        c>self.width - 1 || r> self.height - 1
    }

    /// Check if a given position `<r,c>` is on the map border.
    fn is_on_border(self: &CellularMap<'a>,  r: uint, c: uint) -> bool {
        c == 0 || r == 0 || c == self.width - 1 || r == self.height - 1
    }

    /// Get the row-major index for the given position.
    fn get_index(self: &CellularMap<'a>, r: uint, c: uint) -> uint {
        c + r*self.width
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[test]
    fn constructor_test() {
        let cm = CellularMap::new(12u,12u);

        assert!(12u == cm.width);
        assert!(12u == cm.height);
    }

    #[test]
    fn get_element_test() {
        let mut cm = CellularMap::new(12u,12u);
        cm.map[4] = 2u8;
        assert_eq!(2u8, cm.get_element(0,4));
    }

    #[bench]
    fn evolve_bench(b:&mut Bencher) {
        let mut cm = CellularMap::new(30u,30u);
        cm.random_fill(40u);
        b.iter(|| {
            cm.evolve();
        });
    }
}
