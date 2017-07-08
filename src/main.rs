const SIZE: usize = 3;

type World = [[bool; SIZE]; SIZE];

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

//show_world is a placeholder, just uses side-effects to pretty print the world
//TODO: Piston?
fn show_world(w: &World) {
    for row in w.iter() {
        for cell in row.iter() {
            match cell {
                &false => print!(" . "),
                &true => print!(" X "),
            }
        }
        println!("");
    }
}

//get_cell takes a Coord and a World and returns the status of that cell
fn get_cell(c: &Coord, w: &World) -> bool {
    if c.x >= SIZE || c.y >= SIZE {
        panic!("Coord not in World! moore_sum");
    };
    w[c.x][c.y]
}

//moore_sum takes a Coord and a World and
//returns the sum of live cells in the given point's Moore neighborhood
fn moore_sum(c: &Coord, w: &World) -> u32 {
    if c.x >= SIZE || c.y >= SIZE {
        panic!("Coord not in World! moore_sum");
    };
    //collect all cells here
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
        if c.x > 0 && c.y < SIZE - 1 {
            get_cell(&Coord { x: c.x - 1, y: c.y + 1 }, w)
        } else {
            false
        },
        if c.y > 0 {
            get_cell(&Coord { x: c.x, y: c.y - 1 }, w)
        } else {
            false
        },
        if c.y < SIZE - 1 {
            get_cell(&Coord { x: c.x, y: c.y + 1 }, w)
        } else {
            false
        },
        if c.x < SIZE - 1 && c.y > 0 {
            get_cell(&Coord { x: c.x + 1, y: c.y - 1 }, w)
        } else {
            false
        },
        if c.x < SIZE - 1 {
            get_cell(&Coord { x: c.x + 1, y: c.y }, w)
        } else {
            false
        },
        if c.x < SIZE - 1 && c.y < SIZE - 1 {
            get_cell(&Coord { x: c.x + 1, y: c.y + 1 }, w)
        } else {
            false
        },
    ];

    //return total of live cells
    //TODO: i feel like there's a better way to express this
    neighborhood.iter().fold(
        0u32,
        |sum, &val| if &val == &true {
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
            0 | 1 => false, //lonely
            2 | 3 => true, //overcrowded
            _ => false, //ALIVE
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
    let mut ret = [[false; SIZE]; SIZE];
    for (x, row) in w.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            ret[x][y] = tick_cell(&Coord{x: x, y: y}, w);
        }
    }
    ret
}

fn main() {
    let blinker = [
        [false, false, false],
        [true, true, true],
        [false, false, false],
    ];
    show_world(&blinker);
    println!();
    show_world(&tick_world(&blinker));
    println!();
    show_world(&tick_world(&tick_world(&blinker)));
}
