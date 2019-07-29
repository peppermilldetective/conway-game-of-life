# Conway's Game of Life

The classic ruleset of Conway's Game of Life implemented using [rust](https://www.rust-lang.org/) and [glium](https://github.com/glium/glium).

Conway's Game of Life Rules: From [Wikipedia](https://en.wikipedia.org/wiki/Conway's_Game_of_Life)

1. Any live cell with fewer than two live neighbors dies, as if by underpopulation.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by overpopulation.
4. Any dead cell with exactly three live becomes a live cell, as if by reproduction.

TODO:

* Order logic to make more modular and easier to understand.
* Set fixed timesteps to prevent the program from running too fast.
* Rewrite to draw the cells in batches rather than one by one.
* Add mouse/keyboard support for custom drawing of cells and such.
* Make more performant.
