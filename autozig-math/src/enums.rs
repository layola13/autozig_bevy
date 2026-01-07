/// Compass quadrant for 4-direction compass
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompassQuadrant {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

/// Euler rotation order for 3D rotations
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EulerRot {
    ZYX = 0,
    ZXY = 1,
    YXZ = 2,
    YZX = 3,
    XYZ = 4,
    XZY = 5,
}

/// Error for invalid direction vectors
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InvalidDirectionError {
    Zero = 0,
    NaN = 1,
    Infinite = 2,
}
