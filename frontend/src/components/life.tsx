import { Universe } from "wasm-game-of-life";
import { useEffect, useState } from "react";

export default function GameOfLife() {
  const [renderedUniverse, setRenderedUniverse] = useState("");

  useEffect(() => {
    let universe = Universe.new_random(500, 500);

    const interval = setInterval(() => {
      console.time("Universe.step");
      const newUniverse = universe.step();
      console.timeEnd("Universe.step");

      universe.free();
      universe = newUniverse;

      console.time("Universe.render");
      setRenderedUniverse(universe.render_to_text());
      console.timeEnd("Universe.render");
    }, 50);

    return () => {
      clearInterval(interval);
      universe.free();
    };
  }, []);

  return (
    <div>
      <pre>{renderedUniverse}</pre>
    </div>
  );
}
