use kit::math::*;

/// Represents a triangular game board. The first row is a single space in length
/// and each subsequen row is one unit longer until the final row which will
/// have a number of spaces equal to the `size` of the board.
struct Board<T> {
  // pub size: f32,
// pub is_empty: boolean,
// rows: Vec<Vec<T>>,
}

impl Board {
  // pub fn new(size: f32, source?: Board<T>) {
  //   self.__rows = Vec(size);
  //   for y in 0..size {
  //     self.__rows[y] = [];
  //     for x in 0..y {
  //       self.__rows[y][x] = source ? source.__rows[y][x] : null;
  //     }
  //   }

  //   Board {size, rows, is_empty: true}
  // }

  // fn get({ x, y }: BoardCoords): Option<T> {
  //   if x > y || y > self.size return NONE;
  //   return Some(self.rows[y][x]);
  // }

  // set({ x, y }: BoardCoords, value: ?T): Board<T> {
  //   assert!(x <= y && y <= self.size, "invalid board position");
  //   const nextBoard = new Board(self.size, this);
  //   nextBoard.__rows[y][x] = value;
  //   return nextBoard;
  // }

  // wipe(): Board<T> {
  //   return new Board(self.size);
  // }

  // forEach(cb: (coords: BoardCoords, value: ?T) => void): void {
  //   for y in 0..self.size; y++) {
  //     const row = self.__rows[y];
  //     const rowSize = row.length;
  //     for x in 0..rowSize; x++) {
  //       const coords = { x, y };
  //       const value = self.__rows[y][x];
  //       cb(coords, value);
  //     }
  //   }
  // }

  // any(cb: (coords: BoardCoords, value: ?T) => boolean): boolean {
  //   for y in 0..self.size; y++) {
  //     const row = self.__rows[y];
  //     const rowSize = row.length;
  //     for x in 0..rowSize; x++) {
  //       const coords = { x, y };
  //       const value = self.__rows[y][x];
  //       if (cb(coords, value)) return true;
  //     }
  //   }
  //   return false;
  // }

  // every(cb: (coords: BoardCoords, value: ?T) => boolean): boolean {
  //   for y in 0..self.size; y++) {
  //     const row = self.__rows[y];
  //     const rowSize = row.length;
  //     for x in 0..rowSize; x++) {
  //       const coords = { x, y };
  //       const value = self.__rows[y][x];
  //       if (!cb(coords, value)) return false;
  //     }
  //   }
  //   return true;
  // }

  // pub fn map(cb: (coords: BoardCoords, value: ?T) =>?T): Board<T> {
  //   const nextBoard = self.wipe();
  //   self.forEach((pos, value) => {
  //     nextBoard.__rows[pos.y][pos.x] = cb(pos, value);
  //   });
  //   return nextBoard;
  // }

  // pub fn toString(): string {
  //   return self.__rows.map(row => row.toString()).join('\n');
  // }
}
