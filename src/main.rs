use std::time::Instant;

use a_star::IsTraversable;
use grid::Grid2D;
mod a_star;
mod ext;
mod grid;

impl IsTraversable for u8 {
    fn is_traversable(&self) -> bool {
        *self != 1
    }
}

fn main() {
    let grids =
        serde_json::from_slice::<Vec<Grid2D<u8>>>(include_bytes!("../labyrinths.json")).unwrap();

    for mut grid in grids.into_iter() {
        let start = grid
            .enumerate()
            .find(|(_, _, x)| **x == 2)
            .map(|(i, j, _)| (i, j))
            .expect("Start not found!");

        let end = grid
            .enumerate()
            .find(|(_, _, x)| **x == 3)
            .map(|(i, j, _)| (i, j))
            .expect("End not found!");

        let instant = Instant::now();
        let path = a_star::shortest_path(&grid, start, end).unwrap();
        println!("\nT: {:?}", instant.elapsed());

        for coord in path {
            *grid.get_mut(coord).unwrap() = 4;
        }

        for row in grid.rows().into_iter() {
            print!("| ");
            for x in row.into_iter() {
                match x {
                    1 => print!("⬛"),
                    4 => print!("⚪"),
                    _ => print!("  "),
                }
            }
            println!(" |")
        }
    }
}
