//! Coordinate axis

mod grid;
pub use self::grid::Gridline;

use std::borrow::Cow;
use std::iter::IntoIterator;

use crate::map::grid::Map;
use crate::traits::{Configure, Data};
use crate::{grid::Properties as GridProperties, Axis, Grid, Label, Range, Scale, ScaleFactor, Script, TicLabels};

/// Properties of the coordinate axes
#[derive(Clone, Debug)]
pub struct Properties {
    grids: Map<GridProperties>,
    hidden: bool,
    label: Option<Cow<'static, str>>,
    logarithmic: bool,
    range: Option<(f64, f64)>,
    scale_factor: f64,
    tics: Option<String>,
}

impl Default for Properties {
    fn default() -> Properties {
        Properties {
            grids: Map::new(),
            hidden: false,
            label: None,
            logarithmic: false,
            range: None,
            scale_factor: 1.,
            tics: None,
        }
    }
}

impl AxisProperties {
    /// Hides the axis
    ///
    /// **Note** The `TopX` and `RightY` axes are hidden by default
    pub fn hide(&mut self) -> &mut AxisProperties {
        self.hidden = true;
        self
    }

    /// Makes the axis visible
    ///
    /// **Note** The `BottomX` and `LeftY` axes are visible by default
    pub fn show(&mut self) -> &mut AxisProperties {
        self.hidden = false;
        self
    }
    /// Attaches a label to the axis
    pub fn label<S>(&mut self, label: S) -> &mut AxisProperties
    where
        S: Into<Cow<'static, str>>,
    {
        self.label = Some(label.into());
        self
    }

    /// Changes the range of the axis that will be shown
    ///
    /// **Note** All axes are auto-scaled by default
    pub fn range(&mut self, range: Range) -> &mut AxisProperties {
        self.hidden = false;

        match range {
            Range::Auto => self.range = None,
            Range::Limits(low, high) => self.range = Some((low, high)),
        }

        self
    }

    /// Sets the scale of the axis
    ///
    /// **Note** All axes use a linear scale by default
    pub fn scale(&mut self, scale: Scale) -> &mut AxisProperties {
        self.hidden = false;

        self.logarithmic = scale == Scale::Logarithmic;

        self
    }

    /// Changes the *scale factor* of the axis.
    ///
    /// All the data plotted against this axis will have its corresponding coordinate
    /// scaled with this factor before being plotted.
    ///
    /// **Note** The default scale factor is `1`.
    pub fn scale_factor(&mut self, factor: f64) -> &mut AxisProperties {
        self.scale_factor = factor;

        self
    }

    /// Attaches labels to the tics of an axis
    pub fn tick_labels<P, L>(&mut self, tics: TicLabels<P, L>) -> &mut AxisProperties
    where
        L: IntoIterator,
        L::Item: AsRef<str>,
        P: IntoIterator,
        P::Item: Data,
    {
        let TicLabels { positions, labels } = tics;

        let pairs = positions
            .into_iter()
            .zip(labels.into_iter())
            .map(|(pos, label)| format!("'{}' {}", label.as_ref(), pos.f64()))
            .collect::<Vec<_>>();

        if pairs.is_empty() {
            self.tics = None
        } else {
            self.tics = Some(pairs.join(", "));
        }

        self
    }

    /// Configure the major grid. These grid lines are places on the major tic marks.
    pub fn configure_major_grid<F: FnOnce(&mut Gridline) -> &mut Gridline>(
        &mut self,
        configure: F,
    ) -> &mut AxisProperties {
        configure(&mut self.major_grid);
        self
    }

    /// Configure the minor grid. These grid lines are places on the minor tic marks.
    pub fn configure_minor_grid<F: FnOnce(&mut Gridline) -> &mut Gridline>(
        &mut self,
        configure: F,
    ) -> &mut AxisProperties {
        configure(&mut self.minor_grid);
        self
    }
}

impl<'a> Script for (Axis, &'a AxisProperties) {
    fn script(&self) -> String {
        let &(axis, properties) = self;
        let axis_ = axis.display();

        let mut script = if properties.hidden {
            return format!("unset {}tics\n", axis_);
        } else {
            format!("set {}tics nomirror ", axis_)
        };

        if let Some(ref tics) = properties.tics {
            script.push_str(&format!("({})", tics))
        }

        script.push('\n');

        if let Some(ref label) = properties.label {
            script.push_str(&format!("set {}label '{}'\n", axis_, label))
        }

        if let Some((low, high)) = properties.range {
            script.push_str(&format!("set {}range [{}:{}]\n", axis_, low, high))
        }

        if properties.logarithmic {
            script.push_str(&format!("set logscale {}\n", axis_));
        }

        script.push_str(&(axis, &properties.major_grid).script());
        script.push_str(&(axis, &properties.minor_grid).script());

        script
    }
}

pub(crate) trait ScaleFactorTrait {
    fn scale_factor(&self) -> f64;
}

impl ScaleFactorTrait for Properties {
    fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}
