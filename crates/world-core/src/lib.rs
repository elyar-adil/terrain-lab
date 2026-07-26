use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorldError {
    #[error("world grid must contain at least two cells per axis")]
    InvalidSize,
    #[error("world size must be positive")]
    InvalidWorldSize,
    #[error("layer length does not match the world grid")]
    InvalidLayerLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridPoint {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldGrid {
    pub size: usize,
    pub world_size_km: f32,
}

impl WorldGrid {
    pub fn new(size: usize, world_size_km: f32) -> Result<Self, WorldError> {
        if size < 2 {
            return Err(WorldError::InvalidSize);
        }
        if !world_size_km.is_finite() || world_size_km <= 0.0 {
            return Err(WorldError::InvalidWorldSize);
        }
        Ok(Self {
            size,
            world_size_km,
        })
    }

    pub fn len(self) -> usize {
        self.size * self.size
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    pub fn cell_metres(self) -> f32 {
        self.world_size_km * 1000.0 / (self.size - 1) as f32
    }

    pub fn index(self, point: GridPoint) -> Option<usize> {
        (point.x < self.size && point.y < self.size).then_some(point.y * self.size + point.x)
    }

    pub fn point(self, index: usize) -> Option<GridPoint> {
        (index < self.len()).then_some(GridPoint {
            x: index % self.size,
            y: index / self.size,
        })
    }

    pub fn distance_km(self, a: GridPoint, b: GridPoint) -> f32 {
        let dx = a.x as f32 - b.x as f32;
        let dy = a.y as f32 - b.y as f32;
        (dx * dx + dy * dy).sqrt() * self.cell_metres() / 1000.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalarLayer {
    pub name: String,
    pub grid: WorldGrid,
    pub values: Vec<f32>,
}

impl ScalarLayer {
    pub fn new(
        name: impl Into<String>,
        grid: WorldGrid,
        values: Vec<f32>,
    ) -> Result<Self, WorldError> {
        if values.len() != grid.len() {
            return Err(WorldError::InvalidLayerLength);
        }
        Ok(Self {
            name: name.into(),
            grid,
            values,
        })
    }

    pub fn value(&self, point: GridPoint) -> Option<f32> {
        self.grid.index(point).map(|index| self.values[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_coordinate_round_trip_is_stable() {
        let grid = WorldGrid::new(128, 80.0).unwrap();
        for index in [0, 127, 128, grid.len() - 1] {
            assert_eq!(grid.index(grid.point(index).unwrap()), Some(index));
        }
    }

    #[test]
    fn physical_distance_uses_world_scale() {
        let grid = WorldGrid::new(101, 100.0).unwrap();
        assert!(
            (grid.distance_km(GridPoint { x: 10, y: 10 }, GridPoint { x: 13, y: 14 }) - 5.0).abs()
                < 1.0e-5
        );
    }
}
