#[derive(Debug)]
pub struct Grid<T> {
    content: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    pub fn new(content: &str, transform: impl Fn(char) -> T) -> Self {
        // count number of lines (ignoring empty lines) and length with of first line
        let height = content.lines().filter(|line| !line.is_empty()).count();
        let width = content.lines().next().unwrap().len();

        let content: Vec<T> = content.chars()
            .filter(|c| c.is_alphanumeric())
            .map(transform)
            .collect();

        // make sure we have an actual grid
        debug_assert_eq!(content.len(), height*width);

        Self {
            content,
            width,
            height,
        }
    }

    pub fn at(&self, col: usize, row: usize) -> Option<T> {
        let start = col + row * self.width;
        self.content.get(start).copied()
    }

    pub fn entry(&self, col: usize, row: usize) -> GridEntry<'_, T> {
        GridEntry { grid: self, col, row }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

/// Helper for working with offsets
#[derive(Debug)]
pub struct GridEntry<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    row: usize,
}

impl<'a, T: Copy> GridEntry<'a, T> {
    pub fn at_offset(&self, col_offset: isize, row_offset: isize) -> Option<T> {
        let Some(true_col) = self.col.checked_add_signed(col_offset)
            else { return None };
        let Some(true_row) = self.row.checked_add_signed(row_offset)
            else { return None };

        if true_col >= self.grid.width() { return None }
        if true_row >= self.grid.height() { return None }

        self.grid.at(true_col, true_row)
    }
}
