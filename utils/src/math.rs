/// Converts a normal angle in degrees into the a byte angle format
pub fn angle_byte(degrees: f32) -> u8 {
    ((degrees * 256.0 / 360.0) as i32 as i8) as u8
}