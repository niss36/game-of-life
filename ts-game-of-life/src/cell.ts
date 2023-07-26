export enum Cell {
  Dead = 0,
  Alive = 1,
}

export const generateRandomCell = (): Cell => {
  return Math.random() > 0.5 ? Cell.Alive : Cell.Dead;
};

export const parseCell = (value: string): Cell => {
  switch (value) {
    case ".":
      return Cell.Dead;

    case "#":
      return Cell.Alive;

    default:
      throw new Error(
        `Invalid cell value '${value}', expected one of '.' or '#'`
      );
  }
};

export const displayCell = (cell: Cell): string => {
  switch (cell) {
    case Cell.Dead:
      return ".";
    case Cell.Alive:
      return "#";
  }
};
