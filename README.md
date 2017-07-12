# Conway's Game of Life
Loops forever - use ctrl-c to end.

invoke with the desired world file: `cargo run blinker` or `cargo run glider`
World files must be square, any non-'0' char is parsed as "true" for the starting state.

Included examples are a 3x3 blinker, 9x9 world with a simple glider, and a 20x20 world with a simple LWSS
