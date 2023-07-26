import {
  convertIndexToCoordinates,
  convertCoordinatesToIndex,
  createGrid,
  getGridItem,
  getGridItemWrapping,
} from "grid";
import { it, expect, describe } from "vitest";

describe("The function to convert an index to coordinates", () => {
  it("returns the correct coordinates when index < columns", () => {
    expect(convertIndexToCoordinates(5, 1)).toStrictEqual([1, 0]);
  });

  it("returns the correct coordinates when index >= columns", () => {
    expect(convertIndexToCoordinates(5, 11)).toStrictEqual([1, 2]);
  });
});

describe("The function to convert coordinates to an index", () => {
  it("returns the correct index", () => {
    expect(convertCoordinatesToIndex(5, [1, 2])).toStrictEqual(11);
  });
});

describe("The function to get an item from a grid", () => {
  it("returns the correct item", () => {
    const grid = createGrid(5, 5, (coordinates) => coordinates);

    expect(getGridItem(grid, [1, 2])).toStrictEqual([1, 2]);
  });

  it("returns undefined when the coordinates are out of bounds", () => {
    const grid = createGrid(5, 5, () => 0);

    expect(getGridItem(grid, [5, 3])).toBeUndefined();
  });
});

describe("The function to get an item from a grid wrapping around the edges", () => {
  it("returns the correct item", () => {
    const grid = createGrid(5, 5, (coordinates) => coordinates);

    expect(getGridItemWrapping(grid, [1, 2])).toStrictEqual([1, 2]);
  });

  it("returns the correct item when the coordinates are out of bounds", () => {
    const grid = createGrid(5, 5, (coordinates) => coordinates);

    expect(getGridItemWrapping(grid, [6, 7])).toStrictEqual([1, 2]);
  });
});
