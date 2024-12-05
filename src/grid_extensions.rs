use grid::Grid;

pub trait GridExtensions<T> {
    fn surrounding(&self, row: usize, col: usize) -> Vec<(usize, usize)>;
    fn lines_from(&self, row: usize, col: usize, count: usize) -> Vec<Vec<((usize, usize), &T)>>;
}

impl<T> GridExtensions<T> for Grid<T> {
    fn surrounding(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut surrounds = Vec::new();

        if row >= self.size().0 || col >= self.size().1 {
            return surrounds;
        }

        let x_below = row < self.size().0 - 1;
        let y_below = col < self.size().1 - 1;
        let x_above = row > 0;
        let y_above = col > 0;

        if x_below {
            surrounds.push((row + 1, col));
        }
        if y_below {
            surrounds.push((row, col + 1));
        }
        if x_above {
            surrounds.push((row - 1, col));
        }
        if y_above {
            surrounds.push((row, col - 1));
        }
        if x_below && y_below {
            surrounds.push((row + 1, col + 1));
        }
        if x_above && y_above {
            surrounds.push((row - 1, col - 1));
        }
        if x_below && y_above {
            surrounds.push((row + 1, col - 1));
        }
        if x_above && y_below {
            surrounds.push((row - 1, col + 1));
        }

        surrounds
    }

    fn lines_from(&self, row: usize, col: usize, length: usize) -> Vec<Vec<((usize, usize), &T)>> {
        let lines = [
            coords_with_value(self, coords_above(row, col).take(length)).collect::<Vec<_>>(),
            coords_with_value(self, coords_above_right(row, col).take(length)).collect(),
            coords_with_value(self, coords_right(row, col).take(length)).collect(),
            coords_with_value(self, coords_below_right(row, col).take(length)).collect(),
            coords_with_value(self, coords_below(row, col).take(length)).collect(),
            coords_with_value(self, coords_below_left(row, col).take(length)).collect(),
            coords_with_value(self, coords_left(row, col).take(length)).collect(),
            coords_with_value(self, coords_above_left(row, col).take(length)).collect(),
        ]
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect();

        lines
    }
}

#[test]
fn test_lines_from() {
    let grid = Grid::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
    let lines = grid.lines_from(1, 1, 2);
    assert_eq!(lines.len(), 8);
    assert!(lines.iter().all(|l| l[0].0 == (1, 1)));
    assert!(lines.iter().all(|l| l.len() == 2));
    println!("{:?}", lines);
}

fn coords_with_value<T>(
    grid: &Grid<T>,
    coords: impl Iterator<Item = (usize, usize)>,
) -> impl Iterator<Item = ((usize, usize), &T)> {
    coords
        .map(|(row, col)| ((row, col), grid.get(row, col)))
        .filter(|(_, value)| value.is_some())
        .map(|(coord, value)| {
            (
                coord,
                value.expect("if this breaks, is_some has a bug in it"),
            )
        })
}

fn down_from(n: usize) -> impl Iterator<Item = usize> {
    (0..=n).rev()
}

fn up_from(n: usize) -> impl Iterator<Item = usize> {
    n..
}

fn coords_above(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    add_snd(down_from(row), col)
}

fn coords_below(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    add_snd(up_from(row), col)
}

fn coords_left(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    add_fst(down_from(col), row)
}

fn coords_right(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    add_fst(up_from(col), row)
}

fn coords_above_left(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    coords_above(row, col)
        .zip(coords_left(row, col))
        .map(|((r1, _), (_, c2))| (r1, c2))
}

fn coords_above_right(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    coords_above(row, col)
        .zip(coords_right(row, col))
        .map(|((r1, _), (_, c2))| (r1, c2))
}

fn coords_below_right(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    coords_below(row, col)
        .zip(coords_right(row, col))
        .map(|((r1, _), (_, c2))| (r1, c2))
}

fn coords_below_left(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    coords_below(row, col)
        .zip(coords_left(row, col))
        .map(|((r1, _), (_, c2))| (r1, c2))
}

fn add_fst<S, F>(snds: impl Iterator<Item = S>, fst: F) -> impl Iterator<Item = (F, S)>
where
    F: Clone,
{
    snds.map(move |s| (fst.clone(), s))
}

fn add_snd<S, F>(fsts: impl Iterator<Item = F>, snd: S) -> impl Iterator<Item = (F, S)>
where
    S: Clone,
{
    fsts.map(move |f| (f, snd.clone()))
}
