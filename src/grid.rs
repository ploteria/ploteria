//! Grid along major and minor ticks

use crate::{Color, Default, Display, LineType, Script};

/// The sorting layer of the gridlines
#[derive(Clone, Copy)]
pub enum GridLayer {
    /// Default sorting layer (Back)
    Default,
    /// The gridlines are rendered in front of the plot
    Front,
    /// The gridlines are rendered behind the plot
    Back,
}

/// The appearence of the major or the minor gridlines
#[derive(Clone, Copy)]
pub struct GridStyle {
    line_width: Option<f64>,
    color: Option<Color>,
    line_type: Option<LineType>,
}

/// The common options of the grid of the plot
#[derive(Clone, Copy)]
pub struct GridOptions {
    layer: Option<GridLayer>,
    major_style: GridStyle,
    minor_style: GridStyle,
}

impl Display<&'static str> for GridLayer {
    fn display(&self) -> &'static str {
        match self {
            GridLayer::Default => "layerdefault",
            GridLayer::Front => "front",
            GridLayer::Back => "back",
        }
    }
}

impl GridStyle {
    /// Sets the line width of the grid
    pub fn line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = Some(width);
        self
    }

    /// Sets the color of the grid
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }

    /// Sets the line type of the grid
    pub fn line_type(&mut self, line_type: LineType) -> &mut Self {
        self.line_type = Some(line_type);
        self
    }
}

impl<'a> Script for &'a GridStyle {
    fn script(&self) -> String {
        let mut script = String::new();
        if let Some(line_width) = self.line_width {
            script.push_str(&format!("lw {} ", line_width));
        }
        if let Some(color) = self.color {
            script.push_str(&format!("lc rgb '{}' ", color.display()));
        }
        if let Some(line_type) = self.line_type {
            script.push_str(&format!("lt {} ", line_type.display()))
        }
        script
    }
}

impl GridOptions {
    /// Sets the sorting layer of both the major and minor gridlines
    pub fn layer(&mut self, layer: GridLayer) -> &mut Self {
        self.layer = Some(layer);
        self
    }

    /// Configure the major gridlines' style
    pub fn configure_major<F: FnOnce(&mut GridStyle) -> &mut GridStyle>(
        &mut self,
        configure: F,
    ) -> &mut Self {
        configure(&mut self.major_style);
        self
    }

    /// Configure the minor gridlines' style
    pub fn configure_minor<F: FnOnce(&mut GridStyle) -> &mut GridStyle>(
        &mut self,
        configure: F,
    ) -> &mut Self {
        configure(&mut self.minor_style);
        self
    }
}

impl Default for GridOptions {
    fn default() -> Self {
        GridOptions {
            layer: None,
            major_style: GridStyle {
                line_width: None,
                color: None,
                line_type: None,
            },
            minor_style: GridStyle {
                line_width: None,
                color: None,
                line_type: None,
            },
        }
    }
}

impl<'a> Script for &'a GridOptions {
    fn script(&self) -> String {
        let mut script = String::from("set grid ");
        if let Some(layer) = self.layer {
            script.push_str(&format!("{} ", layer.display()));
        }
        script.push_str(&format!("{},", (&self.major_style).script()));
        script.push_str(&format!("{}\n", (&self.minor_style).script()));
        script
    }
}
