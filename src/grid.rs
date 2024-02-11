use serde::{de::Error, Deserialize};
use std::fmt::Debug;

pub struct Grid2D<T> {
    arr: Box<[Box<[T]>]>,
}

impl<T> Grid2D<T> {
    pub fn get_mut(&mut self, (i, j): (usize, usize)) -> Option<&mut T> {
        self.arr.get_mut(i)?.get_mut(j)
    }

    pub fn enumerate<'a>(&'a self) -> impl Iterator<Item = (usize, usize, &'a T)> {
        self.arr
            .iter()
            .enumerate()
            .flat_map(|(i, arr)| arr.into_iter().enumerate().map(move |(j, x)| (i, j, x)))
    }

    pub fn neighbours<'a>(
        &'a self,
        (i, j): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize, &'a T)> {
        self.enumerate().filter(move |(ii, jj, _)| {
            ii.abs_diff(i) <= 1 && jj.abs_diff(j) <= 1 && (*ii, *jj) != (i, j)
        })
    }

    pub fn rows(&self) -> &Box<[Box<[T]>]> {
        &self.arr
    }

    pub fn width(&self) -> usize {
        self.arr.len()
    }

    pub fn height(&self) -> usize {
        let Some(row) = self.arr.get(0) else {
            return 0;
        };

        return row.len();
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Grid2D<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr = Box::<[Box<[T]>]>::deserialize(deserializer)?;
        if arr
            .iter()
            .zip(arr.iter().skip(1))
            .any(|(a, b)| a.len() != b.len())
        {
            return Err(Error::custom("Matrix row sizes are inconsistent."));
        }

        Ok(Self { arr })
    }
}

impl<T: Debug> Debug for Grid2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for arr in self.arr.iter() {
            writeln!(f, "{:?}", arr)?;
        }

        Ok(())
    }
}
