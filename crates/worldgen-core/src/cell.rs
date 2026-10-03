//! Hierarchical addresses.
//!
//! A [`Cell`] is a square of the world at a level: level 0 cells are the largest,
//! and each level halves the side. Every cell has one parent and four children,
//! and its address is just three integers, so it can be hashed, compared and
//! sent anywhere.

/// Where level 0 sits in metres, and how big it is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    /// Position of the corner of cell `(0, 0)` at level 0.
    pub origin: [f64; 2],
    /// Side of a level-0 cell, metres.
    pub root_size_m: f64,
}

impl Frame {
    pub const fn new(origin: [f64; 2], root_size_m: f64) -> Self {
        Self {
            origin,
            root_size_m,
        }
    }
}

/// An axis-aligned rectangle in metres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

impl Rect {
    pub fn width(&self) -> f64 {
        self.max[0] - self.min[0]
    }
    pub fn height(&self) -> f64 {
        self.max[1] - self.min[1]
    }
    pub fn contains(&self, p: [f64; 2]) -> bool {
        p[0] >= self.min[0] && p[0] < self.max[0] && p[1] >= self.min[1] && p[1] < self.max[1]
    }
    /// The rectangle grown by `margin` on every side.
    pub fn grown(&self, margin: f64) -> Rect {
        Rect {
            min: [self.min[0] - margin, self.min[1] - margin],
            max: [self.max[0] + margin, self.max[1] + margin],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cell {
    pub level: u8,
    pub x: i64,
    pub y: i64,
}

impl Cell {
    pub const fn new(level: u8, x: i64, y: i64) -> Self {
        Self { level, x, y }
    }

    pub fn parent(self) -> Option<Cell> {
        (self.level > 0)
            .then(|| Cell::new(self.level - 1, self.x.div_euclid(2), self.y.div_euclid(2)))
    }

    /// The ancestor at `level` (this cell itself if it is already there).
    pub fn ancestor(self, level: u8) -> Cell {
        assert!(level <= self.level, "an ancestor is coarser than the cell");
        let shift = u32::from(self.level - level);
        Cell::new(
            level,
            self.x.div_euclid(1_i64 << shift),
            self.y.div_euclid(1_i64 << shift),
        )
    }

    pub fn children(self) -> [Cell; 4] {
        let (l, x, y) = (self.level + 1, self.x * 2, self.y * 2);
        [
            Cell::new(l, x, y),
            Cell::new(l, x + 1, y),
            Cell::new(l, x, y + 1),
            Cell::new(l, x + 1, y + 1),
        ]
    }

    pub fn neighbour(self, dx: i64, dy: i64) -> Cell {
        Cell::new(self.level, self.x + dx, self.y + dy)
    }

    /// Side length in metres.
    pub fn size_m(self, frame: &Frame) -> f64 {
        frame.root_size_m / (1_u64 << self.level) as f64
    }

    pub fn rect(self, frame: &Frame) -> Rect {
        let s = self.size_m(frame);
        let min = [
            frame.origin[0] + self.x as f64 * s,
            frame.origin[1] + self.y as f64 * s,
        ];
        Rect {
            min,
            max: [min[0] + s, min[1] + s],
        }
    }

    /// The cell at `level` containing a point in metres.
    pub fn containing(frame: &Frame, point: [f64; 2], level: u8) -> Cell {
        let s = frame.root_size_m / (1_u64 << level) as f64;
        Cell::new(
            level,
            ((point[0] - frame.origin[0]) / s).floor() as i64,
            ((point[1] - frame.origin[1]) / s).floor() as i64,
        )
    }
}
