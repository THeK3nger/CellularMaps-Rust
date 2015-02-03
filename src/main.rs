#![feature(core)]

extern crate cellular_maps;

use cellular_maps::CellularMap;

fn print_map(map: &CellularMap) {
    let mut res_string = "".to_string();
    for c in (0us..(map.get_width())) {
        for r in (0us..map.get_height()) {
            if map.get_element(r,c) == 0 {
                res_string.push_str(".");
            } else if map.get_element(r,c) == 1 {
                res_string.push_str("#");
            } else {
                res_string.push_str("@")
            }
        }
        res_string.push_str("\n");
    }
    println!("{}",res_string);
}

fn main() {
    let mut cm = CellularMap::new(30us,35us);
    cm.random_fill(40us);
    print_map(&cm);
    cm.evolve();
    cm.evolve();
    cm.evolve();
    print_map(&cm);
}
