use std::{cmp::Ordering, collections::BinaryHeap};

use crate::{Dijkstra, Domain, Grid2, Status};

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub time: f32,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.time == other.time
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // this is intentionally reversed to mimic a min-heap
        other.time.total_cmp(&self.time)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Dijkstra for Domain {
    fn distance_field(&mut self) {
        let mut heap = BinaryHeap::new();

        // initlize the heap for Dijkstra MED
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(cell) = self.get_cell(x, y) {
                    if cell.status != Status::None {
                        continue;
                    }
                    if let Some(cell_mut) = self.get_cell_mut(x, y) {
                        cell_mut.time = Default::default();
                        heap.push(Node { x, y, time: Default::default() })
                    }
                }
            }
        }

        while let Some(Node { x, y, time }) = heap.pop() {
            if let Some(cell) = self.get_cell(x, y)
                && time > cell.time
            {
                continue;
            }

            if let Some(cell_mut) = self.get_cell_mut(x, y) {
                cell_mut.status = Status::None;
            }

            for neighbor in self.neighbors(x, y) {
                let (nx, ny) = self.index(x, y, neighbor.dx, neighbor.dy);
                if let Some(cell) = self.get_cell(nx, ny) {
                    if cell.status != Status::Solid {
                        continue;
                    }

                    let new_time = time + neighbor.dist;

                    if let Some(cell_mut) = self.get_cell_mut(nx, ny)
                        && new_time < cell_mut.time
                    {
                        cell_mut.time = new_time;
                        cell_mut.status = Status::Front;
                        heap.push(Node { x: nx, y: ny, time: new_time });
                    }
                }
            }
        }
    }
}
