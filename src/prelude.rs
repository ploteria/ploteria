//! A collection of the most used traits, structs and enums

pub use axis::{Axes, Axis, Range, Scale, TicLabels};
pub use candlestick::Candlesticks;
pub use curve::Curve::{Dots, Impulses, Lines, LinesPoints, Points, Steps};
pub use errorbar::ErrorBar::{XErrorBars, XErrorLines, YErrorBars, YErrorLines};
pub use filledcurve::FilledCurve;
pub use key::{Horizontal, Justification, Order, Position, Stacked, Vertical};
pub use traits::Plot;
pub use {Color, Figure, LineType, PointType, Terminal};
