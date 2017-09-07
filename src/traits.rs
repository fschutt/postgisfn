pub trait _2d { }
pub trait _3d { }
pub trait Dimension { }

impl<T> Dimension for T where T: _2d { }
impl<T> Dimension for T where T: _3d { }

pub trait Epsg {
    fn to_lat_lon(u32, u32) -> (u32, u32);
    fn from_lat_lon(u32, u32) -> (u32, u32);
}

pub struct Epsg_3857 { }

impl Epsg for Epsg_3857 {
    fn to_lat_lon(a: u32, b: u32) -> (u32, u32) {
        (0.0, 0.0)
    }

    fn from_lat_lon(a: u32, b: u32) -> (u32, u32) {
        (0.0, 0.0)
    }
}
