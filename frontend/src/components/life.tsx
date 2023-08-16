import { Universe } from "wasm-game-of-life";
import { useEffect, useState } from "react";

export default function GameOfLife() {
  const [universe, setUniverse] = useState(Universe.new_empty(0, 0));

  useEffect(() => {
    setUniverse(Universe.new_random(100, 50));
  }, []);

  useEffect(() => {
    const interval = setInterval(
      () =>
        setUniverse((u) => {
          console.time("Universe.step");
          const newUniverse = u.step();
          console.timeEnd("Universe.step");
          return newUniverse;
        }),
      50
    );

    return () => clearInterval(interval);
  }, []);

  console.time("Universe.render");
  const renderedUniverse = universe.render();
  console.timeEnd("Universe.render");

  return (
    <div>
      <pre>{renderedUniverse}</pre>
    </div>
  );
}
