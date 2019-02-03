//! A collection of the most used traits, structs and enums

pub use candlestick::Candlesticks;
pub use curve::Curve::{Dots, Impulses, Lines, LinesPoints, Points, Steps};
pub use errorbar::ErrorBar::{XErrorBars, XErrorLines, YErrorBars, YErrorLines};
pub use filledcurve::FilledCurve;
pub use key::{Horizontal, Justification, Order, Position, Stacked, Vertical};
pub use traits::{Configure, Plot};
pub use {
    Axes, Axis, Color, Figure, Grid, Key, LineType, PointType, Range, Scale, Terminal,
    TicLabels,
};
