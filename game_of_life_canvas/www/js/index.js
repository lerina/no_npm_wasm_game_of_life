import init, { Universe, Cell } from "../pkg/game_of_life_canvas.js";

async function run() {
    const wasm = await init().catch(console.error);
                //init().catch(e => console.error("Error importing `game_of_life.js`:", e));
    const memory = wasm.memory;

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the universe, and get its width and height.
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const pre = document.getElementById("game-of-life-pre");
// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

//------
// To draw the grid between cells, 
// we draw a set of equally-spaced horizontal lines, 
// and a set of equally-spaced vertical lines. 
// These lines criss-cross to form the grid.
const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

// We can directly access WebAssembly's linear memory via memory, 
// which is defined in the raw wasm module wasm_game_of_life_bg.wasm. 
// To draw the cells, we get a pointer to the universe's cells, 
// construct a Uint8Array overlaying the cells buffer, iterate over each cell, 
// and draw a white or black rectangle depending on whether the cell is dead 
// or alive, respectively. 
// By working with pointers and overlays, 
// we avoid copying the cells across the boundary on every tick.
const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};
//---
const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();

  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
};

  requestAnimationFrame(renderLoop);
}//^--run

//-------------------
run();
