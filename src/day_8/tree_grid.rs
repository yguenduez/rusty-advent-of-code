struct TreeGrid {
    inner: Vec<usize>,
    width: usize,
}

impl TreeGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        TreeGrid {
            inner: vec![0; rows * cols],
            width: cols,
        }
    }

    pub fn get_tree_at(&self, row: usize, col: usize) -> usize {
        self.get_at_index(row, col)
    }

    pub fn set_tree_at(&mut self, row: usize, col: usize, tree_height: usize) {
        let tree = self.get_mut_at_index(row, col);
        *tree = tree_height;
    }

    fn get_mut_at_index(&mut self, row: usize, col: usize) -> &mut usize {
        self.assert_input_bounds(row, col);
        &mut self.inner[row * self.width + col]
    }

    fn get_at_index(&self, row: usize, col: usize) -> usize {
        self.assert_input_bounds(row, col);
        self.inner[row * self.width + col]
    }

    fn assert_input_bounds(&self, row: usize, col: usize) {
        if col > self.width || (row + 1 * col + 1) > self.inner.len() {
            panic!("Index out of range!");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day_8::tree_grid::TreeGrid;

    fn build_tree_native() -> Vec<Vec<usize>> {
        vec![vec![0, 1, 2], vec![4, 5, 6], vec![1, 3, 5]]
    }

    #[test]
    fn returns_correct_tree_height() {
        // Given
        let trees = build_tree_native();
        let grid = TreeGrid {
            inner: trees.into_iter().flatten().collect(),
            width: 3usize,
        };

        // When // Then
        assert_eq!(5, grid.get_tree_at(1, 1));
        assert_eq!(0, grid.get_tree_at(0, 0));
        assert_eq!(4, grid.get_tree_at(1, 0));
        assert_eq!(6, grid.get_tree_at(1, 2));
    }

    #[test]
    fn sets_tree_correctly() {
        // Given
        let mut trees = TreeGrid::new(3, 3);

        // When
        trees.set_tree_at(2, 2, 99);

        // Then
        for i in 0..2 {
            for j in 0..2 {
                if i == 2 && j == 2 {
                    assert_eq!(99, trees.get_tree_at(i, j));
                } else {
                    assert_eq!(0, trees.get_tree_at(i, j));
                }
            }
        }
    }
}
