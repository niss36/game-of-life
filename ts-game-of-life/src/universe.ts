import { Cell, displayCell, generateRandomCell, parseCell } from "cell";
import { Grid, createGrid, getGridItem, getGridItemWrapping } from "grid";

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
    let count = 0;

    for (const columnOffset of [-1, 0, 1]) {
      for (const rowOffset of [-1, 0, 1]) {
        if (columnOffset === 0 && rowOffset === 0) {
          continue;
        }

        const neighbourCoordinates: [number, number] = [
          coordinates[0] + columnOffset,
          coordinates[1] + rowOffset,
        ];

        count += getGridItemWrapping(this.cells, neighbourCoordinates);
      }
    }

    return count;
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

  render_to_text(): string {
    const rows = Array.from({ length: this.cells.rows }, (_, rowIndex) => {
      const row = this.cells.items.slice(
        rowIndex * this.cells.columns,
        (rowIndex + 1) * this.cells.columns
      );

      return row.map((cell) => displayCell(cell)).join("") + "\n";
    });

    return rows.join("");
  }

  free(): void {
    // No-op because JS is garbage collected
  }
}
