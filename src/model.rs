pub mod bgs;
pub mod body;
pub mod station;
pub mod system;
pub mod system_populated;

mod dec;

pub use system::{SystemWithCoordinates, SystemWithoutCoordinates};
pub use system_populated::SystemPopulated;
