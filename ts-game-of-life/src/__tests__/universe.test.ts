import { Cell } from "cell";
import { Universe } from "universe";
import { it, expect, describe } from "vitest";

describe("The function to count live neighbours of a cell", () => {
  it("returns the correct number", () => {
    const universe = Universe.parse("##\n##");

    expect(universe.countLiveNeighbours([0, 0])).toStrictEqual(3);
  });
});

describe("The function to get the new state of a cell", () => {
  it("returns the correct state", () => {
    const universe = Universe.parse("##\n##");

    expect(universe.getNewState([0, 0])).toStrictEqual(Cell.Alive);
  });
});
