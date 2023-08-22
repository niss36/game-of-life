import { Universe } from "wasm-game-of-life";
import { useEffect, useRef } from "react";

const CELL_SIZE = 5;
const GRID_COLOUR = "#CCCCCC";
const DEAD_COLOUR = "#FFFFFF";
const ALIVE_COLOUR = "#000000";

export default function CanvasGameOfLife() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const columns = 500;
  const rows = 500;

  useEffect(() => {
    let universe = Universe.new_random(columns, rows);

    const interval = setInterval(() => {
      console.time("Universe.step");
      const newUniverse = universe.step();
      console.timeEnd("Universe.step");

      universe.free();
      universe = newUniverse;

      if (canvasRef.current === null) {
        return;
      }

      const context = canvasRef.current.getContext("2d");
      if (context === null) {
        return;
      }

      console.time("Universe.render_to_canvas");
      universe.render_to_canvas(
        context,
        CELL_SIZE,
        GRID_COLOUR,
        DEAD_COLOUR,
        ALIVE_COLOUR
      );
      console.timeEnd("Universe.render_to_canvas");
    }, 50);

    return () => {
      clearInterval(interval);
      universe.free();
    };
  }, []);

  return (
    <div>
      <canvas
        width={columns * (CELL_SIZE + 1) + 1}
        height={rows * (CELL_SIZE + 1) + 1}
        ref={canvasRef}
      />
    </div>
  );
}
