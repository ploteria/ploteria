//! A collection of the most used traits, structs and enums

pub use crate::axis::{Axes, Axis, Range, Scale, TicLabels};
pub use crate::candlestick::Candlesticks;
pub use crate::curve::Curve::{Dots, Impulses, Lines, LinesPoints, Points, Steps};
pub use crate::errorbar::ErrorBar::{XErrorBars, XErrorLines, YErrorBars, YErrorLines};
pub use crate::filledcurve::FilledCurve;
pub use crate::key::{Horizontal, Justification, Order, Position, Stacked, Vertical};
pub use crate::traits::Plot;
pub use crate::{Color, Figure, LineType, PointType, Terminal};
