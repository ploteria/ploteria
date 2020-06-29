use crate::{Default, Display, Script};

#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum GridLayer {
    Default,
    Front,
    Back,
}

#[derive(Clone, Copy)]
pub struct GridStyle {
    line_width: Option<f64>,
}

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
    pub fn line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = Some(width);
        self
    }
}

impl<'a> Script for &'a GridStyle {
    fn script(&self) -> String {
        let mut script = String::new();
        if let Some(line_width) = self.line_width {
            script.push_str(&format!("lw {} ", line_width));
        }
        script
    }
}

impl GridOptions {
    pub fn layer(&mut self, layer: GridLayer) -> &mut Self {
        self.layer = Some(layer);
        self
    }

    pub fn configure_major<F: FnOnce(&mut GridStyle) -> &mut GridStyle>(
        &mut self,
        configure: F,
    ) -> &mut Self {
        configure(&mut self.major_style);
        self
    }

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
            major_style: GridStyle { line_width: None },
            minor_style: GridStyle { line_width: None },
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
