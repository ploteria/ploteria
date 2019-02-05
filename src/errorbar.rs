//! Error bar plots

use std::fmt::Debug;
use std::iter::IntoIterator;

use crate::data::Matrix;
use crate::traits::{Data, Plot as PlotTrait};
use crate::{scale_factor, Axes, Color, Figure, LineType, Plot, PointType, Script};

use itertools::izip;

/// Properties common to error bar plots
#[derive(Clone, Debug)]
pub struct Properties {
    color: Option<Color>,
    label: Option<&'static str>,
    line_type: LineType,
    linewidth: Option<f64>,
    point_size: Option<f64>,
    point_type: Option<PointType>,
    style: Style,
}

impl Properties {
    /// Changes the color of the error bars
    pub fn color(mut self, color: Color) -> Properties {
        self.color = Some(color);
        self
    }

    /// Sets the legend label
    pub fn label(mut self, label: &'static str) -> Properties {
        self.label = Some(label);
        self
    }

    /// Change the line type
    ///
    /// **Note** By default `Solid` lines are used
    pub fn line_type(mut self, lt: LineType) -> Properties {
        self.line_type = lt;
        self
    }

    /// Changes the linewidth
    ///
    /// # Panics
    ///
    /// Panics if `lw` is a non-positive value
    pub fn line_width(mut self, lw: f64) -> Properties {
        assert!(lw > 0.);

        self.linewidth = Some(lw);
        self
    }

    /// Changes the size of the points
    ///
    /// # Panics
    ///
    /// Panics if `size` is a non-positive value
    pub fn point_size(mut self, ps: f64) -> Properties {
        assert!(ps > 0.);

        self.point_size = Some(ps);
        self
    }

    /// Changes the point type
    pub fn point_type(mut self, pt: PointType) -> Properties {
        self.point_type = Some(pt);
        self
    }

    pub fn from_style(style: Style) -> Properties {
        Properties {
            color: None,
            label: None,
            line_type: LineType::Solid,
            linewidth: None,
            point_type: None,
            point_size: None,
            style,
        }
    }
}

impl Script for Properties {
    fn script(&self) -> String {
        let mut script = format!("with {} ", Into::<&'static str>::into(self.style));

        script.push_str(&format!(
            "lt {} ",
            Into::<&'static str>::into(self.line_type)
        ));

        if let Some(lw) = self.linewidth {
            script.push_str(&format!("lw {} ", lw));
        }

        if let Some(color) = self.color {
            script.push_str(&format!("lc rgb '{}' ", Into::<&'static str>::into(color)));
        }

        if let Some(pt) = self.point_type {
            script.push_str(&format!("pt {} ", Into::<&'static str>::into(pt)));
        }

        if let Some(ps) = self.point_size {
            script.push_str(&format!("ps {} ", ps))
        }

        if let Some(ref label) = self.label {
            script.push_str("title '");
            script.push_str(label);
            script.push('\'')
        } else {
            script.push_str("notitle")
        }

        script
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Style {
    XErrorBars,
    XErrorLines,
    YErrorBars,
    YErrorLines,
}

impl From<Style> for &'static str {
    fn from(style: Style) -> Self {
        match style {
            Style::XErrorBars => "xerrorbars",
            Style::XErrorLines => "xerrorlines",
            Style::YErrorBars => "yerrorbars",
            Style::YErrorLines => "yerrorlines",
        }
    }
}

/// Asymmetric error bar plots
#[derive(Debug)]
pub enum ErrorBar<X, Y, L, H>
where
    X: Debug,
    Y: Debug,
    L: Debug,
    H: Debug,
{
    /// Horizontal error bars
    XErrorBars {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// X coordinate of the left end of the error bar
        x_low: L,
        /// Y coordinate of the right end of the error bar
        x_high: H,
    },
    /// Horizontal error bars, where each point is joined by a line
    XErrorLines {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// X coordinate of the left end of the error bar
        x_low: L,
        /// Y coordinate of the right end of the error bar
        x_high: H,
    },
    /// Vertical error bars
    YErrorBars {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// Y coordinate of the bottom of the error bar
        y_low: L,
        /// Y coordinate of the top of the error bar
        y_high: H,
    },
    /// Vertical error bars, where each point is joined by a line
    YErrorLines {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
        /// Y coordinate of the bottom of the error bar
        y_low: L,
        /// Y coordinate of the top of the error bar
        y_high: H,
    },
}

impl<X, Y, L, H> ErrorBar<X, Y, L, H>
where
    X: Debug,
    Y: Debug,
    L: Debug,
    H: Debug,
{
    fn style(&self) -> Style {
        match *self {
            ErrorBar::XErrorBars { .. } => Style::XErrorBars,
            ErrorBar::XErrorLines { .. } => Style::XErrorLines,
            ErrorBar::YErrorBars { .. } => Style::YErrorBars,
            ErrorBar::YErrorLines { .. } => Style::YErrorLines,
        }
    }
}

impl<X, Y, L, H> PlotTrait<ErrorBar<X, Y, L, H>> for Figure
where
    H: IntoIterator + Debug,
    H::Item: Data,
    L: IntoIterator + Debug,
    L::Item: Data,
    X: IntoIterator + Debug,
    X::Item: Data,
    Y: IntoIterator + Debug,
    Y::Item: Data,
{
    type Properties = Properties;

    fn plot<F>(mut self, e: ErrorBar<X, Y, L, H>, configure: F) -> Figure
    where
        F: FnOnce(Properties) -> Properties,
    {
        let (x_factor, y_factor) = scale_factor(&self.axes, Axes::BottomXLeftY);

        let style = e.style();
        let (x, y, length, height, e_factor) = match e {
            ErrorBar::XErrorBars {
                x,
                y,
                x_low,
                x_high,
            }
            | ErrorBar::XErrorLines {
                x,
                y,
                x_low,
                x_high,
            } => (x, y, x_low, x_high, x_factor),
            ErrorBar::YErrorBars {
                x,
                y,
                y_low,
                y_high,
            }
            | ErrorBar::YErrorLines {
                x,
                y,
                y_low,
                y_high,
            } => (x, y, y_low, y_high, y_factor),
        };
        let data = Matrix::new(
            izip!(x, y, length, height),
            (x_factor, y_factor, e_factor, e_factor),
        );
        self.plots
            .push(Plot::new(data, &configure(Properties::from_style(style))));
        self
    }
}

// TODO XY error bar
// pub struct XyErrorBar<X, Y, XL, XH, YL, YH> {
// x: X,
// y: Y,
// x_low: XL,
// x_high: XH,
// y_low: YL,
// y_high: YH,
// }

// TODO Symmetric error bars
// pub enum SymmetricErrorBar {
// XSymmetricErrorBar { x: X, y: Y, x_delta: D },
// XSymmetricErrorLines { x: X, y: Y, x_delta: D },
// YSymmetricErrorBar { x: X, y: Y, y_delta: D },
// YSymmetricErrorLines { x: X, y: Y, y_delta: D },
// }
