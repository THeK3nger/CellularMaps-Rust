# Cellular Map

### What is

Cellular Map is a simple implementation of a procedura map generator in Rust. It uses the cellular automata technique in order to create structures similar to caves or natuaral environments.

## An Example

	##################################################
	####.....###...###...#############################
	###.......#...........#..#########################
	###.......................##################...###
	###........................################.....##
	####........................##############.......#
	#####.................#.....##############.......#
	######...............###....########..####......##
	######...............####..########.....##.....###
	######..............##############............####
	#######..###......################.............###
	#############.....#########..######............###
	#############.....#######.....######...........###
	##############...#######........####...##......###
	#############.....#####..........####..##.......##
	##########.......................####............#
	#########........................###.............#
	#########...........##............##............##
	########............##..........................##
	######..........................................##
	#####...........................................##
	#####..........................#.........####..###
	######........................###.......######..##
	#######.....................#####.......#####....#
	######......................####.........#.......#
	######...........................................#
	######..........##..................##...........#
	#####....#####..##.................###...........#
	###.....######...##...#............##...........##
	##......#####....########......................###
	#.......####......########..##..................##
	#........###......########..##...................#
	#........###......########.......................#
	##.........#.......######......#.................#
	##..........#......######.....###...............##
	#..........###.....######.....####..............##
	#..........##.......######.....###...............#
	##.........##........######.....###.....##.......#
	###...................#####......###...###......##
	####...................#####.....###....#.......##
	####....................######.................###
	###......................#####...............#####
	###................##....###......##........######
	##................###.............###......#######
	##...............####.............####....########
	###..............#####............######..########
	#######...#......########.........################
	############.....#########....#..#################
	##############..###########..#####################
	##################################################

## Usage

The usage for the class is shown in the example main provided in the source:

```rust
extern crate cellular_maps;

use cellular_maps::CellularMap;

fn main() {
	let mut cm = CellularMap::new(30u,30u);
	cm.random_fill(40u);
	print_map(&cm);
	cm.evolve();
	cm.evolve();
	cm.evolve();
	print_map(&cm);
}
```

Easy. The result is stored in the class and can be accessed how shown in the print function.

```cpp
fn print_map(map: &CellularMap) {
	let mut res_string = "".to_string();
	for c in range(0u,(map.get_width())) {
		for r in range(0u,map.get_height()) {
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
```

More Easy.

### Dependencies. 

None.

### License

License for an algorithm? Nah. Public domain.
