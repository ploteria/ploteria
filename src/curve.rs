//! Simple "curve" like plots

use std::borrow::Cow;
use std::iter::IntoIterator;

use data::Matrix;
use traits::{self, Data};
use {Axes, Color, CurveDefault, Display, Figure, LineType, Plot, PointType, Script};

/// Properties common to simple "curve" like plots
pub struct Properties {
    axes: Option<Axes>,
    color: Option<Color>,
    label: Option<Cow<'static, str>>,
    line_type: LineType,
    linewidth: Option<f64>,
    point_type: Option<PointType>,
    point_size: Option<f64>,
    style: Style,
}

impl Properties {
    /// Select the axes to plot against
    ///
    /// **Note** By default, the `BottomXLeftY` axes are used
    pub fn axes(&mut self, axes: Axes) -> &mut Properties {
        self.axes = Some(axes);
        self
    }

    /// Sets the line color
    pub fn color(&mut self, color: Color) -> &mut Properties {
        self.color = Some(color);
        self
    }

    /// Sets the legend label
    pub fn label<S>(&mut self, label: S) -> &mut Properties
    where
        S: Into<Cow<'static, str>>,
    {
        self.label = Some(label.into());
        self
    }

    /// Changes the line type
    ///
    /// **Note** By default `Solid` lines are used
    pub fn line_type(&mut self, lt: LineType) -> &mut Properties {
        self.line_type = lt;
        self
    }

    /// Changes the width of the line
    ///
    /// # Panics
    ///
    /// Panics if `width` is a non-positive value
    pub fn line_width(&mut self, lw: f64) -> &mut Properties {
        assert!(lw > 0.);

        self.linewidth = Some(lw);
        self
    }

    /// Changes the size of the points
    ///
    /// # Panics
    ///
    /// Panics if `size` is a non-positive value
    pub fn point_size(&mut self, ps: f64) -> &mut Properties {
        assert!(ps > 0.);

        self.point_size = Some(ps);
        self
    }

    /// Changes the point type
    pub fn point_type(&mut self, pt: PointType) -> &mut Properties {
        self.point_type = Some(pt);
        self
    }
}

impl CurveDefault<Style> for Properties {
    fn default(style: Style) -> Properties {
        Properties {
            axes: None,
            color: None,
            label: None,
            line_type: LineType::Solid,
            linewidth: None,
            point_size: None,
            point_type: None,
            style,
        }
    }
}

impl Script for Properties {
    fn script(&self) -> String {
        let mut script = if let Some(axes) = self.axes {
            format!("axes {} ", axes.display())
        } else {
            String::new()
        };

        script.push_str(&format!("with {} ", self.style.display()));
        script.push_str(&format!("lt {} ", self.line_type.display()));

        if let Some(lw) = self.linewidth {
            script.push_str(&format!("lw {} ", lw))
        }

        if let Some(color) = self.color {
            script.push_str(&format!("lc rgb '{}' ", color.display()))
        }

        if let Some(pt) = self.point_type {
            script.push_str(&format!("pt {} ", pt.display()))
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

#[derive(Clone, Copy)]
enum Style {
    Dots,
    Impulses,
    Lines,
    LinesPoints,
    Points,
    Steps,
}

impl Display<&'static str> for Style {
    fn display(&self) -> &'static str {
        match *self {
            Style::Dots => "dots",
            Style::Impulses => "impulses",
            Style::Lines => "lines",
            Style::LinesPoints => "linespoints",
            Style::Points => "points",
            Style::Steps => "steps",
        }
    }
}

impl<X, Y> traits::Plot<Curve<X, Y>> for Figure
where
    X: IntoIterator,
    X::Item: Data,
    Y: IntoIterator,
    Y::Item: Data,
{
    type Properties = Properties;

    fn plot<F>(&mut self, curve: Curve<X, Y>, configure: F) -> &mut Figure
    where
        F: FnOnce(&mut Properties) -> &mut Properties,
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

        let mut props = CurveDefault::default(style);
        configure(&mut props);

        let (x_factor, y_factor) =
            ::scale_factor(&self.axes, props.axes.unwrap_or(::Axes::BottomXLeftY));

        let data = Matrix::new(izip!(x, y), (x_factor, y_factor));
        self.plots.push(Plot::new(data, &props));
        self
    }
}
