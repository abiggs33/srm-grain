#[derive(Debug, Default, Clone, Copy)]
pub struct Neighbor2 {
    pub dx: isize,
    pub dy: isize,
    pub dist: f32,
}

impl Neighbor2 {
    pub fn build(dx: isize, dy: isize) -> Self {
        let (x, y) = (dx as f32, dy as f32);
        Self { dx, dy, dist: (x * x + y * y).sqrt() }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Neighbor3 {
    pub dx: isize,
    pub dy: isize,
    pub dz: isize,
    pub dist: f32,
}

impl Neighbor3 {
    pub fn build(dx: isize, dy: isize, dz: isize) -> Self {
        let (x, y, z) = (dx as f32, dy as f32, dz as f32);
        Self { dx, dy, dz, dist: (x * x + y * y + z * z).sqrt() }
    }
}

#[allow(dead_code)]
#[allow(clippy::approx_constant)]
#[allow(clippy::excessive_precision)]
pub mod relations {
    use crate::geometry::{Neighbor2, Neighbor3};

    pub const DIR4: [Neighbor2; 4] = von_neumann2();
    pub const DIR6: [Neighbor3; 6] = von_neumann3();
    pub const DIR8: [Neighbor2; 8] = moore2();
    pub const DIR26: [Neighbor3; 26] = moore3();

    const SQRT2: f32 = 1.41421356237;
    const SQRT3: f32 = 1.73205080757;

    const fn von_neumann2() -> [Neighbor2; 4] {
        let mut neighbors = [Neighbor2 { dx: 0, dy: 0, dist: 0.0 }; 4];

        let mut idx = 0;

        let mut dx_counter = -1;
        while dx_counter <= 1 {
            let dx = dx_counter;
            dx_counter += 1;
            let mut dy_counter = -1;
            while dy_counter <= 1 {
                let dy = dy_counter;
                dy_counter += 1;

                if dx == 0 && dy == 0 {
                    continue;
                }
                if dx * dx + dy * dy > 1 {
                    continue;
                }

                let dist = if dx == 0 || dy == 0 { 1.0 } else { SQRT2 };

                neighbors[idx] = Neighbor2 { dx, dy, dist };
                idx += 1;
            }
        }

        neighbors
    }

    const fn von_neumann3() -> [Neighbor3; 6] {
        let mut neighbors = [Neighbor3 { dx: 0, dy: 0, dz: 0, dist: 0.0 }; 6];

        let mut idx = 0;

        let mut dx_counter = -1;
        while dx_counter <= 1 {
            let dx = dx_counter;
            dx_counter += 1;
            let mut dy_counter = -1;
            while dy_counter <= 1 {
                let dy = dy_counter;
                dy_counter += 1;
                let mut dz_counter = -1;
                while dz_counter <= 1 {
                    let dz = dz_counter;
                    dz_counter += 1;

                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    if dx * dx + dy * dy + dz * dz > 1 {
                        continue;
                    }

                    let dist = if dx == 0 || dy == 0 || dz == 0 { 1.0 } else { SQRT2 };

                    neighbors[idx] = Neighbor3 { dx, dy, dz, dist };
                    idx += 1;
                }
            }
        }

        neighbors
    }

    const fn moore2() -> [Neighbor2; 8] {
        let mut neighbors = [Neighbor2 { dx: 0, dy: 0, dist: 0.0 }; 8];

        let mut idx = 0;

        let mut dx_counter = -1;
        while dx_counter <= 1 {
            let dx = dx_counter;
            dx_counter += 1;
            let mut dy_counter = -1;
            while dy_counter <= 1 {
                let dy = dy_counter;
                dy_counter += 1;

                if dx == 0 && dy == 0 {
                    continue;
                }

                let dist = if dx == 0 || dy == 0 { 1.0 } else { SQRT2 };

                neighbors[idx] = Neighbor2 { dx, dy, dist };
                idx += 1;
            }
        }

        neighbors
    }

    const fn moore3() -> [Neighbor3; 26] {
        let mut neighbors = [Neighbor3 { dx: 0, dy: 0, dz: 0, dist: 0.0 }; 26];

        let mut idx = 0;

        let mut dx_counter = -1;
        while dx_counter <= 1 {
            let dx = dx_counter;
            dx_counter += 1;
            let mut dy_counter = -1;
            while dy_counter <= 1 {
                let dy = dy_counter;
                dy_counter += 1;
                let mut dz_counter = -1;
                while dz_counter <= 1 {
                    let dz = dz_counter;
                    dz_counter += 1;

                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }

                    let dist = match dx * dx + dy * dy + dz * dz {
                        | 1 => 1.0,
                        | 2 => SQRT2,
                        | 3 => SQRT3,
                        | _ => 1.0,
                    };

                    neighbors[idx] = Neighbor3 { dx, dy, dz, dist };
                    idx += 1;
                }
            }
        }

        neighbors
    }
}
