mod abs;
mod avg;
mod ceil;
pub mod command;
mod floor;
mod max;
mod median;
mod min;
mod mode;
mod product;
mod reducers;
mod round;
mod sqrt;
mod sum;
mod utils;

pub use abs::SubCommand as MathAbs;
pub use avg::SubCommand as MathAvg;
pub use ceil::SubCommand as MathCeil;
pub use command::MathCommand as Math;
pub use floor::SubCommand as MathFloor;
pub use max::SubCommand as MathMax;
pub use median::SubCommand as MathMedian;
pub use min::SubCommand as MathMin;
pub use mode::SubCommand as MathMode;
pub use product::SubCommand as MathProduct;
pub use round::SubCommand as MathRound;
pub use sqrt::SubCommand as MathSqrt;
pub use sum::SubCommand as MathSum;
