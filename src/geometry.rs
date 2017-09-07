use traits::{Dimension, Epsg};

pub enum GeometryType<T: Dimension> {
    Point(Point<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    MultiPoint(MultiPoint<T>),
    MultiLineString(MultiLineString<T>),
    MultiPolygon(MultiPolygon<T>),
    GeometryCollection(GeometryCollection<T>),
}

pub struct Point<T: Dimension> { }
pub struct LineString<T: Dimension> { }
pub struct Polygon<T: Dimension> { }
pub struct MultiPoint<T: Dimension> { }
pub struct MultiLineString<T: Dimension> { }
pub struct MultiPolygon<T: Dimension> { }
pub struct GeometryCollection<T: Dimension> { }

pub enum GeographyType<T: Dimension, U: Epsg> {
    Point(Point<T>, U),
    LineString(Point<T>, U),
    Polygon(Point<T>, U),
    MultiPoint(Point<T>, U),
    MultiLineString(Point<T>, U),
    MultiPolygon(Point<T>, U),
    GeometryCollection(Point<T>, U),
}
