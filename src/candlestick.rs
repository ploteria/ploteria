//! "Candlestick" plots

use std::iter::IntoIterator;
use std::{default::Default, fmt::Debug};
use std::borrow::Cow;
use itertools::izip;

use crate::data::Matrix;
use crate::traits::{Data, Plot as PlotTrait};
use crate::{scale_factor, Axes, Color, Figure, LineType, Plot, Script};

/// Properties common to candlestick plots
#[derive(Debug, Default)]
pub struct Properties {
    color: Option<Color>,
    label: Option<Cow<'static, str>>,
    line_type: LineType,
    line_width: Option<f64>,
}

impl Properties {
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

    pub fn line_type(mut self, line_type: LineType) -> Properties {
        self.line_type = line_type;

        self
    }

    pub fn line_width(mut self, line_width: f64) -> Properties {
        self.line_width = Some(line_width);

        self
    }
}

impl Script for Properties {
    fn script(&self) -> String {
        let mut script = String::from("with candlesticks ");
        let line_type: &'static str = self.line_type.into();
        script.push_str(&format!("lt {} ", line_type));

        if let Some(lw) = self.line_width {
            script.push_str(&format!("lw {} ", lw))
        }

        if let Some(color) = self.color {
            script.push_str(&format!("lc rgb '{}' ", Into::<&'static str>::into(color)));
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

/// A candlestick consists of a box and two whiskers that extend beyond the box
#[derive(Debug)]
pub struct Candlesticks<X, WM, BM, BH, WH>
where
    X: Debug,
    WM: Debug,
    BM: Debug,
    BH: Debug,
    WH: Debug,
{
    /// X coordinate of the candlestick
    pub x: X,
    /// Y coordinate of the end point of the bottom whisker
    pub whisker_min: WM,
    /// Y coordinate of the bottom of the box
    pub box_min: BM,
    /// Y coordinate of the top of the box
    pub box_high: BH,
    /// Y coordinate of the end point of the top whisker
    pub whisker_high: WH,
}

impl<X, WM, BM, BH, WH> PlotTrait<Candlesticks<X, WM, BM, BH, WH>> for Figure
where
    BH: IntoIterator + Debug,
    BH::Item: Data,
    BM: IntoIterator + Debug,
    BM::Item: Data,
    WH: IntoIterator + Debug,
    WH::Item: Data,
    WM: IntoIterator + Debug,
    WM::Item: Data,
    X: IntoIterator + Debug,
    X::Item: Data,
{
    type Properties = Properties;

    fn plot<F>(mut self, candlesticks: Candlesticks<X, WM, BM, BH, WH>, configure: F) -> Figure
    where
        F: FnOnce(Properties) -> Properties,
        X: Debug,
        WM: Debug,
        BM: Debug,
        BH: Debug,
        WH: Debug,
    {
        let (x_factor, y_factor) = scale_factor(&self.axes, Axes::BottomXLeftY);
        let Candlesticks {
            x,
            whisker_min,
            box_min,
            box_high,
            whisker_high,
        } = candlesticks;

        let data = Matrix::new(
            izip!(x, box_min, whisker_min, whisker_high, box_high),
            (x_factor, y_factor, y_factor, y_factor, y_factor),
        );
        self.plots
            .push(Plot::new(data, &configure(Default::default())));
        self
    }
}
