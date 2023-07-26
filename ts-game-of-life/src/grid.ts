export type Grid<T> = {
  items: T[];
  columns: number;
  rows: number;
};

export const convertIndexToCoordinates = (
  columns: number,
  index: number
): [number, number] => {
  return [index % columns, Math.floor(index / columns)];
};

export const convertCoordinatesToIndex = (
  columns: number,
  coordinates: [number, number]
): number => {
  return coordinates[0] + columns * coordinates[1];
};

export const createGrid = <T>(
  columns: number,
  rows: number,
  initialiser: (coordinates: [number, number]) => T
): Grid<T> => {
  const items = Array.from({ length: columns * rows }, (_, index) =>
    initialiser(convertIndexToCoordinates(columns, index))
  );

  return {
    items,
    columns,
    rows,
  };
};

export const getGridItem = <T>(
  grid: Grid<T>,
  coordinates: [number, number]
): T | undefined => {
  if (
    coordinates[0] < 0 ||
    coordinates[0] >= grid.columns ||
    coordinates[1] < 0 ||
    coordinates[1] >= grid.rows
  ) {
    return undefined;
  }

  return grid.items[convertCoordinatesToIndex(grid.columns, coordinates)];
};
