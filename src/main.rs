use std::{env, thread, time};
use std::fs::File;
use std::io::prelude::*;

//const DATA: &str = "data/"; //TODO move examples to 'data/'

type World = Vec<Vec<bool>>;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

//show_world is a placeholder, just uses side-effects to pretty print the world
//TODO: Piston?
fn show_world(w: &World) {
    println!("");
    for row in w.iter() {
        for cell in row.iter() {
            if *cell {
                print!(" X ")
            } else {
               print! (" . ")
            }
        }
        println!("");
    }
}

//get_cell takes a Coord and a World and returns the status of that cell
fn get_cell(c: &Coord, w: &World) -> bool {
    let s = w.len();
    if c.x >= s || c.y >= s {
        panic!("Coord not in World! get_cell");
    };
    w[c.x][c.y]
}

//moore_sum takes a Coord and a World and
//returns the sum of live cells in the given point's Moore neighborhood
fn moore_sum(c: &Coord, w: &World) -> u8 {
    let s = w.len();
    if c.x >= s || c.y >= s {
        panic!("Coord not in World! moore_sum");
    };
    //collect all cells here, returning false for cells past the boundary
    let neighborhood = [
        if c.x > 0 && c.y > 0 {
            get_cell(&Coord { x: c.x - 1, y: c.y - 1 }, w)
        } else {
            false
        },
        if c.x > 0 {
            get_cell(&Coord { x: c.x - 1, y: c.y }, w)
        } else {
            false
        },
        if c.x > 0 && c.y < s - 1 {
            get_cell(&Coord { x: c.x - 1, y: c.y + 1 }, w)
        } else {
            false
        },
        if c.y > 0 {
            get_cell(&Coord { x: c.x, y: c.y - 1 }, w)
        } else {
            false
        },
        if c.y < s - 1 {
            get_cell(&Coord { x: c.x, y: c.y + 1 }, w)
        } else {
            false
        },
        if c.x < s - 1 && c.y > 0 {
            get_cell(&Coord { x: c.x + 1, y: c.y - 1 }, w)
        } else {
            false
        },
        if c.x < s - 1 {
            get_cell(&Coord { x: c.x + 1, y: c.y }, w)
        } else {
            false
        },
        if c.x < s - 1 && c.y < s - 1 {
            get_cell(&Coord { x: c.x + 1, y: c.y + 1 }, w)
        } else {
            false
        },
    ];

    //return total of live cells
    neighborhood.iter().fold(
        0u8,
        |sum, &val| if val {
            sum + 1
        } else {
            sum
        },
    )
}

//tick_cell takes a Coord and a World and returns the next state of the cell
fn tick_cell(c: &Coord, w: &World) -> bool {
    let s = moore_sum(c, w);
    if get_cell(c, w) {
        match s {
            2 | 3 => true, //ALIVE
            _ => false, //overcrowded or starved
        }
    } else {
        match s {
            3 => true, //3 gives birth
            _ => false, //barren
        }
    }
}

//tick_world takes a reference to a World and returns the next World
fn tick_world(w: &World) -> World {
    let s = w.len();
    let mut ret = vec![vec![false; s]; s];
    for (x, row) in w.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            ret[x][y] = tick_cell(&Coord{x: x, y: y}, w);
        }
    }
    ret
}

fn advance_and_show(w: &World) -> World {
    let next = tick_world(w);
    show_world(&next);
    thread::sleep(time::Duration::from_millis(100));
    next
}

//open_world opens the specified world and returns it as a string along with the size
fn open_world() -> (String, usize) {
    //TODO validate arg input
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).expect("not found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("could not read file");

    //make sure it's square
    let mut rows = 0;
    let mut total = 0;

    for c in contents.chars() {
        if c == '\n' {
            rows += 1;
            //start new sub-array
        } else {
            total += 1;
            //record false or true
        }
    }
    if rows * rows != total {
        panic!("not a square!  fix input file");
    }
    (contents, rows)
}

//parse_world returns the world data structure
fn parse_world(w: &str, s: usize) -> World {
    let mut ret = vec![vec![false; s]; s];
    for (x, line) in w.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c != '0' { //take any non-zero value as "true"
                ret[x][y] = true;
            }
        }
    }
   ret
}

//init loads and validates the world
fn init() -> World {
    let (sworld, size) = open_world();
    parse_world(&sworld, size)
}

//run loops through generations without terminating
//TODO quit if no change
fn run(w: World) {
    show_world(&w);
    let mut current = w;
    loop {
        current = advance_and_show(&current);
    }
}

fn main() {
    //TODO put into lib.rs - pass in SIZE as param there after loading
    let world = init();
    run(world);

}
