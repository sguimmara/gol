/// Represent the game state
#[derive(Debug)]
pub struct GameState {
    // the iteration counter
    iteration: usize,
    size: GridSize,
    cells: Vec<Cell>,
}

#[derive(Debug)]
pub struct GridSize {
    width: usize,
    height: usize,
}

impl GridSize {
    fn offset(&self, col: usize, row: usize) -> usize {
        row * self.width + col
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Cell {
    current: CellState,
    next: CellState
}

impl Cell {
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
    }

    pub fn new(alive: bool) -> Self {
        Cell {
            current: CellState::from(alive),
            next: CellState::Dead
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Dead
    }
}

impl CellState {
    fn from(b: bool) -> Self {
        match b {
            true => CellState::Alive,
            false => CellState::Dead,
        }
    }
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = vec![Cell::default(); width * height];
        let grid_size = GridSize { width, height };

        for col in 0..width {
            for row in 0..height {
                let alive: bool = rand::random();
                let offset = grid_size.offset(col, row);
                grid[offset] = Cell::new(alive);
            }
        }

        GameState {
            iteration: 0,
            size: grid_size,
            cells: grid,
        }
    }

    /// performs an iteration of the `GameState`
    pub fn step(&mut self) {
        self.iteration += 1;
        Self::step_cells(self);
    }

    fn step_cells(state: &mut GameState) {
        for col in 0..state.size.width {
            for row in 0..state.size.height {
                let offset = state.size.offset(col, row);
                let live_neighbours = Self::get_live_neighbours(state, col as i32, row as i32);
                let cell = &mut state.cells[offset];
                let current = cell.current;
                let next = Self::get_next_state(current, live_neighbours);
                cell.next = next;
            }
        }

        // finalize the iteration by swapping the current and next states
        for col in 0..state.size.width {
            for row in 0..state.size.height {
                let offset = state.size.offset(col, row);
                let cell = &mut state.cells[offset];
                cell.swap();
            }
        }
    }

    fn get_next_state(current: CellState, live_neighbours: usize) -> CellState {
        match current {
            CellState::Dead => match live_neighbours {
                3 => CellState::Alive,
                _ => CellState::Dead,
            },
            CellState::Alive => match live_neighbours {
                2 => CellState::Alive,
                3 => CellState::Alive,
                _ => CellState::Dead,
            },
        }
    }

    fn get_cell(&self, col: i32, row: i32) -> Option<CellState> {
        if col < 0 || col > self.size.width as i32 - 1 {
            return None
        }
        if row < 0 || row > self.size.height as i32 - 1 {
            return None
        }

        Some(self.cells[self.size.offset(col as usize, row as usize)].current)
    }

    fn get_count(state: Option<CellState>) -> usize {
        match state {
            None => 0,
            Some(CellState::Alive) => 1,
            Some(CellState::Dead) => 0,
        }
    }

    fn get_live_neighbours(&self, col: i32, row: i32) -> usize {
        let ul = Self::get_cell(self, col - 1, row + 1);
        let um = Self::get_cell(self, col + 0, row + 1);
        let ur = Self::get_cell(self, col + 1, row + 1);
        let r = Self::get_cell(self, col + 1, row + 0);
        let l = Self::get_cell(self, col - 1, row + 0);
        let lr = Self::get_cell(self, col + 1, row - 1);
        let lm = Self::get_cell(self, col + 0, row - 1);
        let ll = Self::get_cell(self, col - 1, row - 1);

        return Self::get_count(ul) + Self::get_count(um) + Self::get_count(ur) +
               Self::get_count(r) + Self::get_count(l) +
               Self::get_count(lr) + Self::get_count(lm) + Self::get_count(ll);
    }

    // renders the current generation
    pub fn render(&self) -> String {
        let capacity = self.size.height * self.size.width + self.size.height - 1; // for the newline
        let mut result = String::with_capacity(capacity);

        for row in 0..self.size.height {
            for col in 0..self.size.width {
                let cell = self.get_cell(col as i32, row as i32).unwrap();
                let chr = match cell {
                    CellState::Dead => ' ',
                    CellState::Alive => 'o'
                };

                result.push(chr)
            }

            result.push('\n');
        }

        result.push_str(format!("generation: {}", self.iteration).as_str());

        result
    }
}

#[test]
fn GameState_new_returns_correct_grid_size() {
    assert_eq!(50, GameState::new(5, 10).cells.len());
    assert_eq!(2, GameState::new(1, 2).cells.len());
    assert_eq!(115 * 38, GameState::new(115, 38).cells.len());
}

#[test]
fn get_next_state_test_from_dead() {
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Dead, 1)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Dead, 2)
    );

    assert_eq!(
        CellState::Alive,
        GameState::get_next_state(CellState::Dead, 3)
    );

    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Dead, 4)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Dead, 5)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Dead, 6)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Dead, 7)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Dead, 8)
    );
}

#[test]
fn get_next_state_test_from_alive() {
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Alive, 1)
    );

    assert_eq!(
        CellState::Alive,
        GameState::get_next_state(CellState::Alive, 2)
    );
    assert_eq!(
        CellState::Alive,
        GameState::get_next_state(CellState::Alive, 3)
    );

    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Alive, 4)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Alive, 5)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Alive, 6)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Alive, 7)
    );
    assert_eq!(
        CellState::Dead,
        GameState::get_next_state(CellState::Alive, 8)
    );
}
