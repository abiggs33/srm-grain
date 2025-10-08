use std::{cmp::Ordering, collections::BinaryHeap};

pub mod ffi;

pub trait Dijkstra {
    // minimum Euclidean distance field using Dijkstra's (modified) algorithm
    fn med_field(&mut self);
}

pub trait FastMarching {
    // finds 1D analogue to surface area
    fn perimeter(&self) -> f32;
}

struct Neighbor {
    dx: isize,
    dy: isize,
    dist: f32,
}

impl Neighbor {
    pub fn build(dx: isize, dy: isize) -> Self {
        let (x, y) = (dx as f32, dy as f32);
        Self {
            dx,
            dy,
            dist: (x * x + y * y).sqrt(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    #[default]
    Unreachable,
    Reached,
    Trial,
    Boundary,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cell {
    pub status: Status,
    pub time: f32, // time arrival that the fuel gest burnt
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            status: Default::default(),
            time: f32::INFINITY,
        }
    }
}

pub struct GrainSlice {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Vec<Cell>>,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    time: f32,
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

impl GrainSlice {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            cells: vec![vec![Cell::default(); width]; height],
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = [usize; 2]> {
        let mut neighbors = Vec::new();

        for [dx, dy] in geometry::DIR8 {
            let (nx, ny) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
            if !self.inbounds(nx, ny) {
                continue;
            }

            neighbors.push([nx, ny]);
        }

        neighbors.into_iter()
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
}

impl Dijkstra for GrainSlice {
    fn med_field(&mut self) {
        let mut heap = BinaryHeap::new();

        // initlize the heap for Dijkstra MED
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x].status == Status::Reached {
                    self.cells[y][x].time = Default::default();
                    heap.push(Node {
                        x,
                        y,
                        time: Default::default(),
                    });
                }
            }
        }

        while let Some(node) = heap.pop() {
            let Node { x, y, time } = node;

            if time > self.cells[y][x].time {
                continue;
            }

            self.cells[y][x].status = Status::Reached;

            let neighbors: Vec<[usize; 2]> = self.neighbors(x, y).collect();
            for [nx, ny] in neighbors {
                if self.cells[ny][nx].status != Status::Unreachable {
                    continue;
                }

                let dx = (nx as f32 - x as f32) * 1.0; // this needs to be input
                let dy = (ny as f32 - y as f32) * 1.0;
                let distance = (dx * dx + dy * dy).sqrt();
                let new_time = time + distance;

                if new_time < self.cells[ny][nx].time {
                    self.cells[ny][nx].time = new_time;
                    self.cells[ny][nx].status = Status::Trial;
                    heap.push(Node {
                        x: nx,
                        y: ny,
                        time: new_time,
                    });
                }
            }
        }
    }
}

#[allow(dead_code)]
mod geometry {
    #[rustfmt::skip]
    pub const DIR4: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

    #[rustfmt::skip]
    pub const DIR8: [[isize; 2]; 8] = [
        // Von-Neumann
        [-1, 0 ],
        [1 , 0 ],
        [0 , -1],
        [0 , 1 ],
        // Moore
        [1 , 1 ],
        [1 , -1],
        [-1, 1 ],
        [-1, -1],
    ];
}
