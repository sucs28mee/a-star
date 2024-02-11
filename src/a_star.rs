use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    rc::Rc,
};

use crate::{ext::Unfolding, grid::Grid2D};

struct Node {
    coords: (usize, usize),
    g_cost: usize,
    h_cost: usize,
    prev: Option<Rc<Node>>,
}

impl Node {
    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .f_cost()
            .partial_cmp(&self.f_cost())
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        other.f_cost().eq(&self.f_cost())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f_cost().partial_cmp(&self.f_cost())
    }
}

pub trait IsTraversable {
    fn is_traversable(&self) -> bool;
}

pub fn shortest_path<T: IsTraversable>(
    grid: &Grid2D<T>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    fn distance((ai, aj): (usize, usize), (bi, bj): (usize, usize)) -> usize {
        ai.abs_diff(bi) + aj.abs_diff(bj)
    }

    fn valid_coords<T>((i, j): (usize, usize), grid: &Grid2D<T>) -> bool {
        (0..grid.width()).contains(&i) && (0..grid.height()).contains(&j)
    }

    if !valid_coords(start, grid) || !valid_coords(end, grid) {
        return None;
    }

    let mut open = BinaryHeap::from([Node {
        coords: start,
        g_cost: 0usize,
        h_cost: distance(start, end),
        prev: None,
    }]);
    let mut closed = HashSet::new();

    while let Some(current_node) = open.pop() {
        if current_node.coords == end {
            return Some({
                let mut path = Some(&current_node)
                    .unfold(|node| node.prev.as_deref())
                    .map(|node| node.coords)
                    .collect::<Vec<_>>();
                path.reverse();
                path
            });
        }

        let current_node = Rc::new(current_node);
        closed.insert(current_node.coords);
        for (i, j, x) in grid.neighbours(current_node.coords) {
            if !x.is_traversable() || closed.contains(&(i, j)) {
                continue;
            }

            let new_node = Node {
                coords: (i, j),
                g_cost: current_node.g_cost + distance((i, j), current_node.coords),
                h_cost: distance((i, j), end),
                prev: Some(current_node.clone()),
            };

            if open
                .iter()
                .find(|node| node.coords == (i, j))
                .map_or(true, |old_node: &Node| {
                    new_node.f_cost() < old_node.f_cost()
                })
            {
                open.push(new_node);
            };
        }
    }

    None
}
