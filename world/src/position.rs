#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            yaw: 0.0,
            pitch: 0.0,
            on_ground: false,
        }
    }

    pub fn add(self, dx: f64, dy: f64, dz: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
            ..self
        }
    }

    pub fn with_look(self, yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch, ..self }
    }

    pub fn with_ground(self, on_ground: bool) -> Self {
        Self { on_ground, ..self }
    }

    pub fn to_center(self) -> Self {
        Self {
            x: self.x.floor() + 0.5,
            y: self.y.floor() + 0.5,
            z: self.z.floor() + 0.5,
            ..self
        }
    }

    pub fn block_x(self) -> i32 {
        self.x.floor() as i32
    }

    pub fn block_y(self) -> i32 {
        self.y.floor() as i32
    }

    pub fn block_z(self) -> i32 {
        self.z.floor() as i32
    }

    pub fn chunk_x(self) -> i32 {
        self.block_x().div_euclid(16)
    }

    pub fn chunk_z(self) -> i32 {
        self.block_z().div_euclid(16)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0.0, 64.0, 0.0)
    }
}