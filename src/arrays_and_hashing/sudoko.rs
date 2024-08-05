use std::collections::{HashMap, HashSet};

pub fn is_valid_sodoku(board: Vec<Vec<char>>) -> bool {
    let mut map = HashMap::<char, Sodoku>::with_capacity(9);
    !board
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter(|(_, x)| *x != '.')
                .map(|(x, letter)| (letter, SodokuElement::from(y, x)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .any(|(letter, element)| {
            match map.get_mut(&letter) {
                Some(sodokus) => {
                    if sodokus.exists(element) {
                        return true;
                    };
                }
                None => {
                    map.insert(letter, Sodoku::new(element));
                }
            };
            false
        })
}

struct Sodoku {
    pub rows: HashSet<usize>,
    pub columns: HashSet<usize>,
    pub areas: HashSet<usize>,
}

impl Sodoku {
    fn new(SodokuElement { row, column, area }: SodokuElement) -> Self {
        let mut rows = HashSet::with_capacity(9);
        let mut columns = HashSet::with_capacity(9);
        let mut areas = HashSet::with_capacity(9);

        rows.insert(row);
        columns.insert(column);
        areas.insert(area);

        Self {
            rows,
            columns,
            areas,
        }
    }

    fn exists(&mut self, SodokuElement { row, column, area }: SodokuElement) -> bool {
        let rows = self.rows.insert(row);
        let columns = self.columns.insert(column);
        let areas = self.areas.insert(area);
        !rows || !columns || !areas
    }
}

struct SodokuElement {
    pub row: usize,
    pub column: usize,
    pub area: usize,
}

impl SodokuElement {
    pub fn from(row: usize, column: usize) -> Self {
        let area = (row / 3) * 3 + (column / 3);
        Self { row, column, area }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_valid_sodoku_test() {
        assert!(is_valid_sodoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '.', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '.'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ]));

        assert!(!is_valid_sodoku(vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]));

        assert!(!is_valid_sodoku(vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.']
        ]));
    }
}
