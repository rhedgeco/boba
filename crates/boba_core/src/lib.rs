mod pearl;
mod registry;
mod resources;
mod stage;

pub use pearl::*;
pub use registry::*;
pub use resources::*;
pub use stage::*;

pub mod stages;

/// Generic result for quick returning from stage updates
pub type BobaResult = anyhow::Result<()>;
