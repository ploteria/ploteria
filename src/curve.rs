//! Simple "curve" like plots

use std::borrow::Cow;
use std::iter::IntoIterator;

use itertools::izip;

use crate::axis::Axes;
use crate::data::Matrix;
use crate::traits::{Data, Plot as PlotTrait};
use crate::{scale_factor, Color, Figure, LineType, Plot, PointType, Script};

/// Properties common to simple "curve" like plots
#[derive(Clone, Debug)]
pub struct Properties {
    axes: Option<Axes>,
    color: Option<Color>,
    label: Option<Cow<'static, str>>,
    line_type: LineType,
    line_width: Option<f64>,
    point_type: Option<PointType>,
    point_size: Option<f64>,
    style: Style,
}

impl Properties {
    pub fn from_style(style: Style) -> Properties {
        Properties {
            axes: None,
            color: None,
            label: None,
            line_type: LineType::Solid,
            line_width: None,
            point_size: None,
            point_type: None,
            style,
        }
    }

    /// Select the axes to plot against
    ///
    /// **Note** By default, the `BottomXLeftY` axes are used
    pub fn axes(mut self, axes: Axes) -> Properties {
        self.axes = Some(axes);
        self
    }

    /// Sets the line color
    pub fn color(mut self, color: Color) -> Properties {
        self.color = Some(color);
        self
    }

    /// Sets the legend label
    pub fn label<S>(mut self, label: S) -> Properties
    where
        S: Into<Cow<'static, str>>,
    {
        self.label = Some(label.into());
        self
    }

    /// Changes the line type
    ///
    /// **Note** By default `Solid` lines are used
    pub fn line_type(mut self, lt: LineType) -> Properties {
        self.line_type = lt;
        self
    }

    /// Changes the width of the line
    ///
    /// # Panics
    ///
    /// Panics if `width` is a non-positive value
    pub fn line_width(mut self, lw: f64) -> Properties {
        assert!(lw > 0.);

        self.line_width = Some(lw);
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
}

impl Script for Properties {
    fn script(&self) -> String {
        let mut script = if let Some(axes) = self.axes {
            let axes: &'static str = axes.into();
            format!("axes {} ", axes)
        } else {
            String::new()
        };

        script.push_str(&format!("with {} ", Into::<&'static str>::into(self.style)));
        script.push_str(&format!(
            "lt {} ",
            Into::<&'static str>::into(self.line_type)
        ));

        if let Some(lw) = self.line_width {
            script.push_str(&format!("lw {} ", lw))
        }

        if let Some(color) = self.color {
            script.push_str(&format!("lc rgb '{}' ", Into::<&'static str>::into(color)));
        }

        if let Some(pt) = self.point_type {
            script.push_str(&format!("pt {} ", Into::<&'static str>::into(pt)));
        }

        if let Some(ps) = self.point_size {
            script.push_str(&format!("ps {} ", ps));
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

/// Types of "curve" plots
pub enum Curve<X, Y> {
    /// A minimally sized dot on each data point
    Dots {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
    },
    /// A vertical "impulse" on each data point
    Impulses {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
    },
    /// Line that joins the data points
    Lines {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
    },
    /// Line with a point on each data point
    LinesPoints {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
    },
    /// A point on each data point
    Points {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
    },
    /// An step `_|` between each data point
    Steps {
        /// X coordinate of the data points
        x: X,
        /// Y coordinate of the data points
        y: Y,
    },
}

impl<X, Y> Curve<X, Y> {
    fn style(&self) -> Style {
        match *self {
            Curve::Dots { .. } => Style::Dots,
            Curve::Impulses { .. } => Style::Impulses,
            Curve::Lines { .. } => Style::Lines,
            Curve::LinesPoints { .. } => Style::LinesPoints,
            Curve::Points { .. } => Style::Points,
            Curve::Steps { .. } => Style::Steps,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Style {
    Dots,
    Impulses,
    Lines,
    LinesPoints,
    Points,
    Steps,
}

impl From<Style> for &'static str {
    fn from(style: Style) -> Self {
        match style {
            Style::Dots => "dots",
            Style::Impulses => "impulses",
            Style::Lines => "lines",
            Style::LinesPoints => "linespoints",
            Style::Points => "points",
            Style::Steps => "steps",
        }
    }
}

impl<X, Y> PlotTrait<Curve<X, Y>> for Figure
where
    X: IntoIterator,
    X::Item: Data,
    Y: IntoIterator,
    Y::Item: Data,
{
    type Properties = Properties;

    fn plot<F>(mut self, curve: Curve<X, Y>, configure: F) -> Figure
    where
        F: FnOnce(Properties) -> Properties,
    {
        let style = curve.style();
        let (x, y) = match curve {
            Curve::Dots { x, y }
            | Curve::Impulses { x, y }
            | Curve::Lines { x, y }
            | Curve::LinesPoints { x, y }
            | Curve::Points { x, y }
            | Curve::Steps { x, y } => (x, y),
        };

        let props = Properties::from_style(style);
        configure(props.clone());

        let (x_factor, y_factor) =
            scale_factor(&self.axes, props.axes.unwrap_or(Axes::BottomXLeftY));

        let data = Matrix::new(izip!(x, y), (x_factor, y_factor));
        self.plots.push(Plot::new(data, &props));
        self
    }
}
