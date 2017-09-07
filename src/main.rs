pub mod schema;
pub mod traits;
pub mod geometry;
pub mod index;
pub mod raster;
pub mod spheroid;

use schema::Schema;
use traits::{Dimension};
use geometry::GeometryType;

pub struct Database {
    schemas: Vec<Schema>,
}

pub struct Table { }

fn add_geometry_column<T: Dimension>
(schema_name: &str, table_name: &str, column_name: &str, srid: u32, _type: GeometryType<T>)
-> Result<(), Error>
{
    Ok(())
}

pub enum Error {
    /// Function is not yet implemented
    Unimplemented,
}

fn main() {
    println!("Hello, world!");
}
