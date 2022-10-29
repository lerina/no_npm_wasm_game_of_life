import init, { Universe } from "../pkg/game_of_life_pre.js";

async function run() {
    const wasm = await init().catch(console.error);
                //init().catch(e => console.error("Error importing `game_of_life.js`:", e));

    const pre = document.getElementById("game-of-life-canvas");
    const universe = Universe.new();

    const renderLoop = () => {
      pre.textContent = universe.render();
      universe.tick();

      requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
}//^--run

run();
