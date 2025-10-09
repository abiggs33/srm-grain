use crate::geometry::{Neighbor2, relations::DIR8};

pub mod area_marching;
pub mod distance_field;
pub mod ffi;
pub mod geometry;

pub trait Dijkstra {
    // minimum Euclidean distance field using Dijkstra's (modified) algorithm
    fn distance_field(&mut self);
}

pub trait FastMarching {
    // finds 1D analogue to surface area
    fn trial_area(&self, elapsed_time: f32) -> f32;
}

pub trait Grid2 {
    fn width(&self) -> usize;

    fn height(&self) -> usize;

    fn inbounds(&self, x: usize, y: usize) -> bool;

    fn index(&self, x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize);

    fn neighbors(&self, x: usize, y: usize) -> Vec<Neighbor2>;

    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell>;

    fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Solid,
    None,
    Front,
    Boundary,
}

impl Default for Status {
    fn default() -> Self {
        Self::Solid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cell {
    pub status: Status,
    pub time: f32, // time arrival that the fuel gest burnt
}

impl Default for Cell {
    fn default() -> Self {
        Self { status: Default::default(), time: f32::INFINITY }
    }
}

pub struct Domain {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Domain {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            cells: vec![vec![Cell::default(); width]; height],
        }
    }
}

impl Grid2 for Domain {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn inbounds(&self, x: usize, y: usize) -> bool {
        self.width > x && self.height > y
    }

    fn index(&self, x: usize, y: usize, dx: isize, dy: isize) -> (usize, usize) {
        ((x as isize + dx) as usize, (y as isize + dy) as usize)
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<Neighbor2> {
        let mut neighbors = Vec::new();
        for neighbor in DIR8 {
            let (nx, ny) = self.index(x, y, neighbor.dx, neighbor.dy);
            if !self.inbounds(nx, ny) {
                continue;
            }

            neighbors.push(neighbor);
        }
        neighbors
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if !self.inbounds(x, y) {
            return None;
        }
        Some(&self.cells[y][x])
    }

    fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if !self.inbounds(x, y) {
            return None;
        }
        Some(&mut self.cells[y][x])
    }
}
