//! Railways and airports of the world planner: derived from the same terrain
//! layers as settlements and roads. Rules and sources: `docs/transport-infrastructure.md`.

pub mod airport;
pub mod geometry;
pub mod rail;
pub mod wind;

use terrain_core::TerrainData;

/// The terrain as metres-space fields: all transport geometry is planned in
/// metres from the world corner and converted to kilometres at the edge.
pub struct World<'a> {
    pub terrain: &'a TerrainData,
    pub n: usize,
    pub size_m: f32,
}

/// Open water below this height is the sea (the planner's convention).
pub const SEA_LEVEL_M: f32 = 58.0;

impl<'a> World<'a> {
    pub fn new(terrain: &'a TerrainData, world_size_km: f32) -> Self {
        Self {
            terrain,
            n: terrain.size,
            size_m: world_size_km * 1000.0,
        }
    }

    pub fn cell_m(&self) -> f32 {
        self.size_m / (self.n - 1) as f32
    }

    pub fn sample(&self, field: &[f32], x_m: f32, y_m: f32) -> f32 {
        let last = (self.n - 1) as f32;
        let gx = (x_m / self.size_m * last).clamp(0.0, last);
        let gy = (y_m / self.size_m * last).clamp(0.0, last);
        let (x0, y0) = (gx.floor() as usize, gy.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(self.n - 1), (y0 + 1).min(self.n - 1));
        let (fx, fy) = (gx - x0 as f32, gy - y0 as f32);
        let at = |x: usize, y: usize| field[y * self.n + x];
        let top = at(x0, y0) * (1.0 - fx) + at(x1, y0) * fx;
        let bottom = at(x0, y1) * (1.0 - fx) + at(x1, y1) * fx;
        top * (1.0 - fy) + bottom * fy
    }

    pub fn height(&self, x_m: f32, y_m: f32) -> f32 {
        self.sample(&self.terrain.height, x_m, y_m)
    }

    /// Ground slope (rise over run) from central differences over `span_m`.
    pub fn slope(&self, x_m: f32, y_m: f32, span_m: f32) -> f32 {
        let dx = (self.height(x_m + span_m, y_m) - self.height(x_m - span_m, y_m)) / (2.0 * span_m);
        let dy = (self.height(x_m, y_m + span_m) - self.height(x_m, y_m - span_m)) / (2.0 * span_m);
        dx.hypot(dy)
    }

    pub fn is_sea(&self, x_m: f32, y_m: f32) -> bool {
        self.height(x_m, y_m) < SEA_LEVEL_M
    }

    pub fn is_lake(&self, x_m: f32, y_m: f32) -> bool {
        self.sample(&self.terrain.lake, x_m, y_m) > 0.3
    }

    /// A river or open water the line must bridge.
    pub fn is_river(&self, x_m: f32, y_m: f32) -> bool {
        let last = (self.n - 1) as f32;
        let gx = (x_m / self.size_m * last).round().clamp(0.0, last) as usize;
        let gy = (y_m / self.size_m * last).round().clamp(0.0, last) as usize;
        let i = gy * self.n + gx;
        self.terrain.height[i] >= SEA_LEVEL_M
            && (self.terrain.river_order[i] >= 2 || self.terrain.water[i] > 0.35)
    }

    pub fn wetness(&self, x_m: f32, y_m: f32) -> f32 {
        let w = self.sample(&self.terrain.wetland, x_m, y_m);
        let f = self.sample(&self.terrain.floodplain, x_m, y_m);
        w.max(f * 0.8)
    }
}
