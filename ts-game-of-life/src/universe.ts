import { Cell, displayCell, generateRandomCell, parseCell } from "cell";
import { Grid, createGrid, getGridItem } from "grid";

enum Offset {
  TopLeft = "TopLeft",
  Top = "Top",
  TopRight = "TopRight",
  Left = "Left",
  Right = "Right",
  BottomLeft = "BottomLeft",
  Bottom = "Bottom",
  BottomRight = "BottomRight",
}

const applyOffset = (
  offset: Offset,
  coordinates: [number, number]
): [number, number] => {
  const [x, y] = coordinates;

  switch (offset) {
    case Offset.TopLeft:
      return [x - 1, y - 1];
    case Offset.Top:
      return [x, y - 1];
    case Offset.TopRight:
      return [x + 1, y - 1];
    case Offset.Left:
      return [x - 1, y];
    case Offset.Right:
      return [x + 1, y];
    case Offset.BottomLeft:
      return [x - 1, y + 1];
    case Offset.Bottom:
      return [x, y + 1];
    case Offset.BottomRight:
      return [x + 1, y + 1];
  }
};

export class Universe {
  private cells: Grid<Cell>;

  constructor(cells: Grid<Cell>) {
    this.cells = cells;
  }

  static new_empty(columns: number, rows: number): Universe {
    return new Universe(createGrid(columns, rows, () => Cell.Dead));
  }

  static new_random(columns: number, rows: number): Universe {
    return new Universe(createGrid(columns, rows, () => generateRandomCell()));
  }

  countLiveNeighbours(coordinates: [number, number]): number {
    const neighbourCoordinates = Object.values(Offset).map((offset) =>
      applyOffset(offset, coordinates)
    );

    const neighbourStates = neighbourCoordinates.map((coordinates) =>
      getGridItem(this.cells, coordinates)
    );

    const numberOfLiveNeighbours = neighbourStates.reduce(
      (total, cell) => (cell === undefined ? total : total + cell),
      0
    );

    return numberOfLiveNeighbours;
  }

  getNewState(coordinates: [number, number]): Cell {
    const numberOfLiveNeighbours = this.countLiveNeighbours(coordinates);

    const currentState = getGridItem(this.cells, coordinates);

    if (currentState === Cell.Alive && numberOfLiveNeighbours === 2) {
      return Cell.Alive;
    }
    if (currentState === Cell.Alive && numberOfLiveNeighbours === 3) {
      return Cell.Alive;
    }
    if (currentState === Cell.Dead && numberOfLiveNeighbours === 3) {
      return Cell.Alive;
    }

    return Cell.Dead;
  }

  step(): Universe {
    return new Universe(
      createGrid(this.cells.columns, this.cells.rows, (coordinates) =>
        this.getNewState(coordinates)
      )
    );
  }

  static parse(value: string): Universe {
    const cells = [];
    let optionalColumns = undefined;

    for (const row of value.trimEnd().split("\n")) {
      if (optionalColumns === undefined) {
        optionalColumns = row.length;
      } else if (optionalColumns !== row.length) {
        throw new Error(
          `Mismatched row lengths, expected ${optionalColumns} but got ${row.length}`
        );
      }

      const rowCells = [...row].map((char) => parseCell(char));

      cells.push(...rowCells);
    }

    const columns = optionalColumns ?? 0;
    const rows = columns === 0 ? 0 : cells.length / columns;

    return new Universe({
      items: cells,
      columns,
      rows,
    });
  }

  render(): string {
    const rows = Array.from({ length: this.cells.rows }, (_, rowIndex) => {
      const row = this.cells.items.slice(
        rowIndex * this.cells.columns,
        (rowIndex + 1) * this.cells.columns
      );

      return row.map((cell) => displayCell(cell)).join("") + "\n";
    });

    return rows.join("");
  }
}
