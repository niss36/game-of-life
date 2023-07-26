import { Cell } from "cell";
import { Universe } from "universe";
import { it, expect, describe } from "vitest";

const mockUniverseWithPadding = `\
....
.##.
.##.
....`;

const mockUniverseWithoutPadding = `\
.#
#.`;

describe("The function to count live neighbours of a cell", () => {
  it("returns the correct number", () => {
    const universe = Universe.parse(mockUniverseWithPadding);

    expect(universe.countLiveNeighbours([2, 1])).toStrictEqual(3);
  });

  it("returns the correct number across the edge of the universe", () => {
    const universe = Universe.parse(mockUniverseWithoutPadding);

    expect(universe.countLiveNeighbours([0, 0])).toStrictEqual(4);
  });
});

describe("The function to get the new state of a cell", () => {
  it("returns the correct state", () => {
    const universe = Universe.parse(mockUniverseWithPadding);

    expect(universe.getNewState([1, 1])).toStrictEqual(Cell.Alive);
  });
});
