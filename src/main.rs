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
        let Some(start) = grid
            .enumerate()
            .find(|(_, _, x)| **x == 2)
            .map(|(i, j, _)| (i, j)) else 
        {
            println!("Start not found!");
            continue;
        };

        let Some(end) = grid
            .enumerate()
            .find(|(_, _, x)| **x == 3)
            .map(|(i, j, _)| (i, j)) else 
        {
            println!("End not found!");
            continue;
        };

        let instant = Instant::now();
        let Some(path) = a_star::shortest_path(&grid, start, end) else {
            println!("Path not found!");
            continue;
        };
        
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
