//! Gridline

use crate::{Axis, Script};

/// Gridline properties.
///
/// Modified through [`configure_major_grid`] and [`configure_minor_grid`].
///
/// [`configure_major_grid`]: struct.AxisProperties.html#method.configure_major_grid
/// [`configure_minor_grid`]: struct.AxisProperties.html#method.configure_minor_grid
#[derive(Clone, Copy, Debug)]
pub struct Gridline {
    is_minor: bool,
    hidden: bool,
}

// TODO Lots of configuration pending: linetype, linewidth, etc
impl Gridline {
    pub(crate) fn new(is_minor: bool) -> Gridline {
        Gridline {
            is_minor,
            hidden: true,
        }
    }

    /// Hides the gridlines
    ///
    /// **Note** Both `Major` and `Minor` gridlines are hidden by default
    pub fn hide(mut self) -> Gridline {
        self.hidden = true;

        self
    }

    /// Shows the gridlines
    pub fn show(mut self) -> Gridline {
        self.hidden = false;

        self
    }
}

impl<'a> Script for (Axis, &'a Gridline) {
    fn script(&self) -> String {
        let &(axis, properties) = self;
        let axis: &'static str = axis.into();
        let grid = if properties.is_minor { "m" } else { "" };

        if properties.hidden {
            String::new()
        } else {
            format!("set grid {}{}tics\n", grid, axis)
        }
    }
}
