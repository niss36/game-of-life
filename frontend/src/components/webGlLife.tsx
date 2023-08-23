import { Universe } from "wasm-game-of-life";
import { useEffect, useRef } from "react";

const CELL_SIZE = 5;

export default function WebGlGameOfLife() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const columns = 500;
  const rows = 500;

  useEffect(() => {
    if (canvasRef.current === null) {
      console.error("Canvas element missing");
      return;
    }

    const context = canvasRef.current.getContext("webgl2");
    if (context === null) {
      console.error("Failed to create rendering context");
      return;
    }

    let universe = Universe.new_random(columns, rows);
    const programInfo = universe.setup_web_gl(context, CELL_SIZE);

    const interval = setInterval(() => {
      console.time("Universe.step");
      const newUniverse = universe.step();
      console.timeEnd("Universe.step");

      universe.free();
      universe = newUniverse;

      console.time("Universe.render_to_web_gl");
      universe.render_to_web_gl(context, programInfo);
      console.timeEnd("Universe.render_to_web_gl");
    }, 50);

    return () => {
      clearInterval(interval);
      universe.free();
      programInfo.free();
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
