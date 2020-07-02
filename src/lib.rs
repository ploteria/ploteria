//! [Criterion]'s plotting library.
//!
//! [Criterion]: https://github.com/bheisler/criterion.rs
//!
//! **WARNING** This library is criterion's implementation detail and there no plans to stabilize
//! it. In other words, the API may break at any time without notice.
//!
//! # Examples
//!
//! - Simple "curves" (based on [`simple.dem`](http://gnuplot.sourceforge.net/demo/simple.html))
//!
//! ![Plot](curve.svg)
//!
//! ```
//! extern crate itertools_num;
//! extern crate ploteria as plot;
//!
//! # use std::fs;
//! # use std::path::Path;
//! use itertools_num::linspace;
//! use plot::prelude::*;
//!
//! # fn main() {
//! # if let Err(_) = plot::version() {
//! #     return;
//! # }
//! let ref xs = linspace::<f64>(-10., 10., 51).collect::<Vec<_>>();
//!
//! # fs::create_dir_all(Path::new("target/doc/ploteria")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .font("Helvetica")
//! #   .font_size(12.)
//! #   .output(Path::new("target/doc/ploteria/curve.svg"))
//! #   .figure_size(1280, 720)
//!     .configure_key(|k| {
//!         k.boxed(true)
//!          .position(Position::Inside(Vertical::Top, Horizontal::Left))
//!     })
//!     .plot(LinesPoints {
//!               x: xs,
//!               y: xs.iter().map(|x| x.sin()),
//!           },
//!           |lp| {
//!               lp.color(Color::DarkViolet)
//!                 .label("sin(x)")
//!                 .line_type(LineType::Dash)
//!                 .point_size(1.5)
//!                 .point_type(PointType::Circle)
//!           })
//!     .plot(Steps {
//!               x: xs,
//!               y: xs.iter().map(|x| x.atan()),
//!           },
//!           |s| {
//!               s.color(Color::Rgb(0, 158, 115))
//!                .label("atan(x)")
//!                .line_width(2.)
//!           })
//!     .plot(Impulses {
//!               x: xs,
//!               y: xs.iter().map(|x| x.atan().cos()),
//!           },
//!           |i| {
//!               i.color(Color::Rgb(86, 180, 233))
//!                .label("cos(atan(x))")
//!           })
//!     .draw()  // (rest of the chain has been omitted)
//! #   .ok()
//! #   .and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//! #   }));
//! # }
//! ```
//!
//! - error bars (based on
//! [Julia plotting tutorial](https://plot.ly/julia/error-bars/#Colored-and-Styled-Error-Bars))
//!
//! ![Plot](error_bar.svg)
//!
//! ```
//! extern crate itertools_num;
//! extern crate rand;
//! extern crate ploteria as plot;
//!
//! # use std::fs;
//! # use std::path::Path;
//! use std::f64::consts::PI;
//!
//! use itertools_num::linspace;
//! use rand::{Rng, XorShiftRng};
//! use plot::prelude::*;
//!
//! fn sinc(mut x: f64) -> f64 {
//!     if x == 0. {
//!         1.
//!     } else {
//!         x *= PI;
//!         x.sin() / x
//!     }
//! }
//!
//! # fn main() {
//! # if let Err(_) = plot::version() {
//! #     return;
//! # }
//! let ref xs_ = linspace::<f64>(-4., 4., 101).collect::<Vec<_>>();
//!
//! // Fake some data
//! let ref mut rng: XorShiftRng = rand::thread_rng().gen();
//! let xs = linspace::<f64>(-4., 4., 13).skip(1).take(11);
//! let ys = xs.map(|x| sinc(x) + 0.05 * rng.gen::<f64>() - 0.025).collect::<Vec<_>>();
//! let y_low = ys.iter().map(|&y| y - 0.025 - 0.075 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let y_high = ys.iter().map(|&y| y + 0.025 + 0.075 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let xs = linspace::<f64>(-4., 4., 13).skip(1).take(11);
//! let xs = xs.map(|x| x + 0.2 * rng.gen::<f64>() - 0.1);
//!
//! # fs::create_dir_all(Path::new("target/doc/ploteria")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .font("Helvetica")
//! #   .font_size(12.)
//! #   .output(Path::new("target/doc/ploteria/error_bar.svg"))
//! #   .figure_size(1280, 720)
//!     .configure_axis(Axis::BottomX, |a| {
//!         a.tick_labels(TicLabels {
//!             labels: &["-π", "0", "π"],
//!             positions: &[-PI, 0., PI],
//!         })
//!     })
//!     .configure_key(|k|
//!         k.position(Position::Outside(Vertical::Top, Horizontal::Right)))
//!     .plot(Lines {
//!               x: xs_,
//!               y: xs_.iter().cloned().map(sinc),
//!           },
//!           |l| {
//!               l.color(Color::Rgb(0, 158, 115))
//!                .label("sinc(x)")
//!                .line_width(2.)
//!           })
//!     .plot(YErrorBars {
//!               x: xs,
//!               y: &ys,
//!               y_low: &y_low,
//!               y_high: &y_high,
//!           },
//!           |eb| {
//!               eb.color(Color::DarkViolet)
//!                 .line_width(2.)
//!                 .point_type(PointType::FilledCircle)
//!                 .label("measured")
//!           })
//!     .draw()  // (rest of the chain has been omitted)
//! #   .ok()
//! #   .and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//! #   }));
//! # }
//! ```
//!
//! - Candlesticks (based on
//! [`candlesticks.dem`](http://gnuplot.sourceforge.net/demo/candlesticks.html))
//!
//! ![Plot](candlesticks.svg)
//!
//! ```
//! extern crate rand;
//! extern crate ploteria as plot;
//!
//! # use std::fs;
//! # use std::path::Path;
//! use plot::prelude::*;
//! use rand::Rng;
//!
//! # fn main() {
//! # if let Err(_) = plot::version() {
//! #     return;
//! # }
//! let xs = 1..11;
//!
//! // Fake some data
//! let mut rng = rand::thread_rng();
//! let bh = xs.clone().map(|_| 5f64 + 2.5 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let bm = xs.clone().map(|_| 2.5f64 + 2.5 * rng.gen::<f64>()).collect::<Vec<_>>();
//! let wh = bh.iter().map(|&y| y + (10. - y) * rng.gen::<f64>()).collect::<Vec<_>>();
//! let wm = bm.iter().map(|&y| y * rng.gen::<f64>()).collect::<Vec<_>>();
//! let m = bm.iter().zip(bh.iter()).map(|(&l, &h)| (h - l) * rng.gen::<f64>() + l)
//!     .collect::<Vec<_>>();
//!
//! # fs::create_dir_all(Path::new("target/doc/ploteria")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .font("Helvetica")
//! #   .font_size(12.)
//! #   .output(Path::new("target/doc/ploteria/candlesticks.svg"))
//! #   .figure_size(1280, 720)
//!     .box_width(0.2)
//!     .configure_axis(Axis::BottomX, |a| a.range(Range::Limits(0., 11.)))
//!     .plot(Candlesticks {
//!               x: xs.clone(),
//!               whisker_min: &wm,
//!               box_min: &bm,
//!               box_high: &bh,
//!               whisker_high: &wh,
//!           },
//!           |cs| {
//!               cs.color(Color::Rgb(86, 180, 233))
//!                 .label("Quartiles")
//!                 .line_width(2.)
//!           })
//!     // trick to plot the median
//!     .plot(Candlesticks {
//!               x: xs,
//!               whisker_min: &m,
//!               box_min: &m,
//!               box_high: &m,
//!               whisker_high: &m,
//!           },
//!           |cs| {
//!               cs.color(Color::Black)
//!                 .line_width(2.)
//!           })
//!     .draw()  // (rest of the chain has been omitted)
//! #   .ok()
//! #   .and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//! #   }));
//! # }
//! ```
//!
//! - Multiaxis (based on [`multiaxis.dem`](http://gnuplot.sourceforge.net/demo/multiaxis.html))
//!
//! ![Plot](multiaxis.svg)
//!
//! ```
//! extern crate itertools_num;
//! extern crate num_complex;
//! extern crate ploteria as plot;
//!
//! # use std::fs;
//! # use std::path::Path;
//! use std::f64::consts::PI;
//!
//! use itertools_num::linspace;
//! use num_complex::Complex;
//! use plot::prelude::*;
//!
//! fn tf(x: f64) -> Complex<f64> {
//!     Complex::new(0., x) / Complex::new(10., x) / Complex::new(1., x / 10_000.)
//! }
//!
//! # fn main() {
//! # if let Err(_) = plot::version() {
//! #     return;
//! # }
//! let (start, end): (f64, f64) = (1.1, 90_000.);
//! let ref xs = linspace(start.ln(), end.ln(), 101).map(|x| x.exp()).collect::<Vec<_>>();
//! let phase = xs.iter().map(|&x| tf(x).arg() * 180. / PI);
//! let magnitude = xs.iter().map(|&x| tf(x).norm());
//!
//! # fs::create_dir_all(Path::new("target/doc/ploteria")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .font("Helvetica")
//! #   .font_size(12.)
//! #   .output(Path::new("target/doc/ploteria/multiaxis.svg"))
//! #   .figure_size(1280, 720)
//!     .title("Frequency response")
//!     .configure_grid(|g| g
//!         .layer(GridLayer::Front)
//!         .configure_major(|g| g
//!             .line_width(2.0))
//!         .configure_minor(|g| g
//!             .line_width(0.05)
//!             .color(Color::Blue)
//!             .line_type(LineType::Solid)))
//!     .configure_axis(Axis::BottomX, |a| a
//!         .major_grid(true)
//!         .minor_grid(true)
//!         .label("Angular frequency (rad/s)")
//!         .range(Range::Limits(start, end))
//!         .scale(Scale::Logarithmic))
//!     .configure_axis(Axis::LeftY, |a| a
//!         .label("Gain")
//!         .scale(Scale::Logarithmic))
//!     .configure_axis(Axis::RightY, |a| a
//!         .label("Phase shift (°)")
//!         .major_grid(true))
//!     .configure_key(|k| k
//!         .position(Position::Inside(Vertical::Top, Horizontal::Center))
//!         .title(" "))
//!     .plot(Lines {
//!         x: xs,
//!         y: magnitude,
//!     }, |l| l
//!         .color(Color::DarkViolet)
//!         .label("Magnitude")
//!         .line_width(2.))
//!     .plot(Lines {
//!         x: xs,
//!         y: phase,
//!     }, |l| l
//!         .axes(Axes::BottomXRightY)
//!         .color(Color::Rgb(0, 158, 115))
//!         .label("Phase")
//!         .line_width(2.))
//!     .draw()  // (rest of the chain has been omitted)
//! #   .ok().and_then(|gnuplot| {
//! #       gnuplot.wait_with_output().ok().and_then(|p| {
//! #           String::from_utf8(p.stderr).ok()
//! #       })
//! #   }));
//! # }
//! ```
//! - Filled curves (based on
//! [`transparent.dem`](http://gnuplot.sourceforge.net/demo/transparent.html))
//!
//! ![Plot](filled_curve.svg)
//!
//! ```
//! extern crate itertools_num;
//! extern crate ploteria as plot;
//!
//! # use std::fs;
//! # use std::path::Path;
//! use std::f64::consts::PI;
//! use std::iter;
//!
//! use itertools_num::linspace;
//! use plot::prelude::*;
//!
//! # fn main() {
//! # if let Err(_) = plot::version() {
//! #     return;
//! # }
//! let (start, end) = (-5., 5.);
//! let ref xs = linspace(start, end, 101).collect::<Vec<_>>();
//! let zeros = iter::repeat(0);
//!
//! fn gaussian(x: f64, mu: f64, sigma: f64) -> f64 {
//!     1. / (((x - mu).powi(2) / 2. / sigma.powi(2)).exp() * sigma * (2. * PI).sqrt())
//! }
//!
//! # fs::create_dir_all(Path::new("target/doc/ploteria")).unwrap();
//! # assert_eq!(Some(String::new()),
//! Figure::new()
//! #   .font("Helvetica")
//! #   .font_size(12.)
//! #   .output(Path::new("target/doc/ploteria/filled_curve.svg"))
//! #   .figure_size(1280, 720)
//!     .title("Transparent filled curve")
//!     .configure_axis(Axis::BottomX, |a| a.range(Range::Limits(start, end)))
//!     .configure_axis(Axis::LeftY, |a| a.range(Range::Limits(0., 1.)))
//!     .configure_key(|k| {
//!         k.justification(Justification::Left)
//!          .order(Order::SampleText)
//!          .position(Position::Inside(Vertical::Top, Horizontal::Left))
//!          .title("Gaussian Distribution")
//!     })
//!     .plot(FilledCurve {
//!               x: xs,
//!               y1: xs.iter().map(|&x| gaussian(x, 0.5, 0.5)),
//!               y2: zeros.clone(),
//!           },
//!           |fc| {
//!               fc.color(Color::ForestGreen)
//!                 .label("μ = 0.5 σ = 0.5")
//!           })
//!     .plot(FilledCurve {
//!               x: xs,
//!               y1: xs.iter().map(|&x| gaussian(x, 2.0, 1.0)),
//!               y2: zeros.clone(),
//!           },
//!           |fc| {
//!               fc.color(Color::Gold)
//!                 .label("μ = 2.0 σ = 1.0")
//!                 .opacity(0.5)
//!           })
//!     .plot(FilledCurve {
//!               x: xs,
//!               y1: xs.iter().map(|&x| gaussian(x, -1.0, 2.0)),
//!               y2: zeros,
//!           },
//!           |fc| {
//!               fc.color(Color::Red)
//!                 .label("μ = -1.0 σ = 2.0")
//!                 .opacity(0.5)
//!           })
//!     .draw()
//!     .ok()
//!     .and_then(|gnuplot| {
//!         gnuplot.wait_with_output().ok().and_then(|p| String::from_utf8(p.stderr).ok())
//!     }));
//! # }
//! ```

#![deny(missing_docs)]
// This lint has lots of false positives ATM, see
// https://github.com/Manishearth/rust-clippy/issues/761
#![allow(clippy::new_without_default)]
// False positives with images
#![allow(clippy::doc_markdown)]
#![allow(clippy::many_single_char_names)]

use std::borrow::Cow;
use std::fmt;
use std::fs::File;
use std::io;
use std::num::ParseIntError;
use std::path::Path;
use std::process::{Child, Command};
use std::str;

use crate::data::Matrix;

mod data;
mod display;
mod map;

pub mod axis;
pub mod candlestick;
pub mod curve;
pub mod errorbar;
pub mod filledcurve;
pub mod grid;
pub mod key;
pub mod prelude;
pub mod traits;

use axis::{Axes, Axis, AxisProperties};
use grid::GridOptions;
use key::KeyProperties;

/// Plot container
#[derive(Clone)]
pub struct Figure {
    alpha: Option<f64>,
    axes: map::axis::Map<axis::AxisProperties>,
    box_width: Option<f64>,
    font: Option<Cow<'static, str>>,
    font_size: Option<f64>,
    key: Option<KeyProperties>,
    output: Cow<'static, Path>,
    plots: Vec<Plot>,
    size: Option<(usize, usize)>,
    terminal: Terminal,
    tics: map::axis::Map<String>,
    title: Option<Cow<'static, str>>,
    grid: Option<GridOptions>,
}

impl Figure {
    /// Creates an empty figure
    pub fn new() -> Figure {
        Figure {
            alpha: None,
            axes: map::axis::Map::new(),
            box_width: None,
            font: None,
            font_size: None,
            key: None,
            output: Cow::Borrowed(Path::new("output.plot")),
            plots: Vec::new(),
            size: None,
            terminal: Terminal::Svg,
            tics: map::axis::Map::new(),
            title: None,
            grid: None,
        }
    }

    /// Changes the box width of all the box related plots (bars, candlesticks, etc)
    ///
    /// **Note** The default value is 0
    ///
    /// # Panics
    ///
    /// Panics if `width` is a negative value
    pub fn box_width(&mut self, width: f64) -> &mut Figure {
        assert!(width >= 0.);

        self.box_width = Some(width);
        self
    }
    /// Changes the font
    pub fn font<S>(&mut self, font: S) -> &mut Figure
    where
        S: Into<Cow<'static, str>>,
    {
        self.font = Some(font.into());
        self
    }
    /// Changes the size of the font
    ///
    /// # Panics
    ///
    /// Panics if `size` is a non-positive value
    pub fn font_size(&mut self, size: f64) -> &mut Figure {
        assert!(size >= 0.);

        self.font_size = Some(size);
        self
    }
    /// Changes the output file
    ///
    /// **Note** The default output file is `output.plot`
    pub fn output<S>(&mut self, output: S) -> &mut Figure
    where
        S: Into<Cow<'static, Path>>,
    {
        self.output = output.into();
        self
    }
    /// Changes the figure size
    pub fn figure_size(&mut self, width: usize, height: usize) -> &mut Figure {
        self.size = Some((width, height));
        self
    }
    /// Changes the output terminal
    ///
    /// **Note** By default, the terminal is set to `Svg`
    pub fn terminal(&mut self, terminal: Terminal) -> &mut Figure {
        self.terminal = terminal;
        self
    }
    /// Sets the title
    pub fn title<S>(&mut self, title: S) -> &mut Figure
    where
        S: Into<Cow<'static, str>>,
    {
        self.title = Some(title.into());
        self
    }

    fn script(&self) -> Vec<u8> {
        let mut s = String::new();

        s.push_str("set encoding utf8\n");

        s.push_str(&format!("set output '{}'\n", self.output.display()));

        if let Some(width) = self.box_width {
            s.push_str(&format!("set boxwidth {}\n", width))
        }

        if let Some(ref title) = self.title {
            s.push_str(&format!("set title '{}'\n", title))
        }

        for axis in self.axes.iter() {
            s.push_str(&axis.script());
        }

        for (_, script) in self.tics.iter() {
            s.push_str(script);
        }

        if let Some(ref key) = self.key {
            s.push_str(&key.script())
        }

        if let Some(ref grid_options) = self.grid {
            s.push_str(&grid_options.script());
        }

        if let Some(alpha) = self.alpha {
            s.push_str(&format!("set style fill transparent solid {}\n", alpha))
        }

        s.push_str(&format!("set terminal {} dashed", self.terminal.display()));

        if let Some((width, height)) = self.size {
            s.push_str(&format!(" size {}, {}", width, height))
        }

        if let Some(ref name) = self.font {
            if let Some(size) = self.font_size {
                s.push_str(&format!(" font '{},{}'", name, size))
            } else {
                s.push_str(&format!(" font '{}'", name))
            }
        }

        // TODO This removes the crossbars from the ends of error bars, but should be configurable
        s.push_str("\nunset bars\n");

        let mut is_first_plot = true;
        for plot in &self.plots {
            let data = plot.data();

            if data.bytes().is_empty() {
                continue;
            }

            if is_first_plot {
                s.push_str("plot ");
                is_first_plot = false;
            } else {
                s.push_str(", ");
            }

            s.push_str(&format!(
                "'-' binary endian=little record={} format='%float64' using ",
                data.nrows()
            ));

            let mut is_first_col = true;
            for col in 0..data.ncols() {
                if is_first_col {
                    is_first_col = false;
                } else {
                    s.push(':');
                }
                s.push_str(&(col + 1).to_string());
            }
            s.push(' ');

            s.push_str(plot.script());
        }

        let mut buffer = s.into_bytes();
        let mut is_first = true;
        for plot in &self.plots {
            if is_first {
                is_first = false;
                buffer.push(b'\n');
            }
            buffer.extend_from_slice(plot.data().bytes());
        }

        buffer
    }

    /// Spawns a drawing child process
    ///
    /// NOTE: stderr, stdin, and stdout are piped
    pub fn draw(&mut self) -> io::Result<Child> {
        use std::process::Stdio;

        let mut gnuplot = Command::new("gnuplot")
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        self.dump(gnuplot.stdin.as_mut().unwrap())?;
        Ok(gnuplot)
    }

    /// Dumps the script required to produce the figure into `sink`
    pub fn dump<W>(&mut self, sink: &mut W) -> io::Result<&mut Figure>
    where
        W: io::Write,
    {
        sink.write_all(&self.script())?;
        Ok(self)
    }

    /// Saves the script required to produce the figure to `path`
    pub fn save(&self, path: &Path) -> io::Result<&Figure> {
        use std::io::Write;

        File::create(path)?.write_all(&self.script())?;
        Ok(self)
    }

    /// Configures an axis.
    pub fn configure_axis<F: FnOnce(&mut AxisProperties) -> &mut AxisProperties>(
        &mut self,
        axis: Axis,
        configure: F,
    ) -> &mut Figure {
        if self.axes.contains_key(axis) {
            configure(self.axes.get_mut(axis).unwrap());
        } else {
            let mut properties = Default::default();
            configure(&mut properties);
            self.axes.insert(axis, properties);
        }
        self
    }

    /// Configures the key (legend).
    pub fn configure_key<F: FnOnce(&mut KeyProperties) -> &mut KeyProperties>(
        &mut self,
        configure: F,
    ) -> &mut Figure {
        match self.key {
            Some(ref mut key) => {
                configure(key);
            }
            None => {
                let mut key = Default::default();
                configure(&mut key);
                self.key = Some(key);
            }
        }
        self
    }

    /// Configures the major and the minor grid
    pub fn configure_grid<F: FnOnce(&mut GridOptions) -> &mut GridOptions>(
        &mut self,
        configure: F,
    ) -> &mut Figure {
        match self.grid {
            Some(ref mut grid) => {
                configure(grid);
            }
            None => {
                let mut grid = Default::default();
                configure(&mut grid);
                self.grid = Some(grid);
            }
        }
        self
    }
}

impl Default for Figure {
    fn default() -> Self {
        Self::new()
    }
}

/// Color
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    DarkViolet,
    ForestGreen,
    Gold,
    Gray,
    Green,
    Magenta,
    Red,
    /// Custom RGB color
    Rgb(u8, u8, u8),
    White,
    Yellow,
}

/// Line type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum LineType {
    Dash,
    Dot,
    DotDash,
    DotDotDash,
    /// Line made of minimally sized dots
    SmallDot,
    Solid,
}

/// Point type
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum PointType {
    Circle,
    FilledCircle,
    FilledSquare,
    FilledTriangle,
    Plus,
    Square,
    Star,
    Triangle,
    X,
}

/// Output terminal
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum Terminal {
    Svg,
}

/// Not public version of `std::default::Default`, used to not leak default constructors into the
/// public API
trait Default {
    /// Creates `Properties` with default configuration
    fn default() -> Self;
}

/// Enums that can produce gnuplot code
trait Display<S> {
    /// Translates the enum in gnuplot code
    fn display(&self) -> S;
}

/// Curve variant of Default
trait CurveDefault<S> {
    /// Creates `curve::Properties` with default configuration
    fn default(_: S) -> Self;
}

/// Error bar variant of Default
trait ErrorBarDefault<S> {
    /// Creates `errorbar::Properties` with default configuration
    fn default(_: S) -> Self;
}

/// Structs that can produce gnuplot code
trait Script {
    /// Translates some configuration struct into gnuplot code
    fn script(&self) -> String;
}

#[derive(Clone)]
struct Plot {
    data: Matrix,
    script: String,
}

impl Plot {
    fn new<S>(data: Matrix, script: &S) -> Plot
    where
        S: Script,
    {
        Plot {
            data,
            script: script.script(),
        }
    }

    fn data(&self) -> &Matrix {
        &self.data
    }

    fn script(&self) -> &str {
        &self.script
    }
}

/// Possible errors when parsing gnuplot's version string
#[derive(Debug)]
pub enum VersionError {
    /// The `gnuplot` command couldn't be executed
    Exec(io::Error),
    /// The `gnuplot` command returned an error message
    Error(String),
    /// The `gnuplot` command returned invalid utf-8
    OutputError,
    /// The `gnuplot` command returned an unparseable string
    ParseError(String),
}
impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionError::Exec(err) => write!(f, "`gnuplot --version` failed: {}", err),
            VersionError::Error(msg) => {
                write!(f, "`gnuplot --version` failed with error message:\n{}", msg)
            }
            VersionError::OutputError => write!(f, "`gnuplot --version` returned invalid utf-8"),
            VersionError::ParseError(msg) => write!(
                f,
                "`gnuplot --version` returned an unparseable version string: {}",
                msg
            ),
        }
    }
}
impl ::std::error::Error for VersionError {
    fn description(&self) -> &str {
        match self {
            VersionError::Exec(_) => "Execution Error",
            VersionError::Error(_) => "Other Error",
            VersionError::OutputError => "Output Error",
            VersionError::ParseError(_) => "Parse Error",
        }
    }

    fn cause(&self) -> Option<&dyn (::std::error::Error)> {
        match self {
            VersionError::Exec(err) => Some(err),
            _ => None,
        }
    }
}

/// Structure representing a gnuplot version number.
pub struct Version {
    /// The major version number
    pub major: usize,
    /// The minor version number
    pub minor: usize,
    /// The patch level
    pub patch: String,
}

/// Returns `gnuplot` version
pub fn version() -> Result<Version, VersionError> {
    let command_output = Command::new("gnuplot")
        .arg("--version")
        .output()
        .map_err(VersionError::Exec)?;
    if !command_output.status.success() {
        let error =
            String::from_utf8(command_output.stderr).map_err(|_| VersionError::OutputError)?;
        return Err(VersionError::Error(error));
    }

    let output = String::from_utf8(command_output.stdout).map_err(|_| VersionError::OutputError)?;

    parse_version(&output).map_err(|_| VersionError::ParseError(output.clone()))
}

fn parse_version(version_str: &str) -> Result<Version, Option<ParseIntError>> {
    let mut words = version_str.split_whitespace().skip(1);
    let mut version = words.next().ok_or(None)?.split('.');
    let major = version.next().ok_or(None)?.parse()?;
    let minor = version.next().ok_or(None)?.parse()?;
    let patchlevel = words.nth(1).ok_or(None)?.to_owned();

    Ok(Version {
        major,
        minor,
        patch: patchlevel,
    })
}

fn scale_factor(map: &map::axis::Map<AxisProperties>, axes: Axes) -> (f64, f64) {
    use crate::Axes::*;
    use crate::Axis::*;

    match axes {
        BottomXLeftY => (
            map.get(BottomX).map_or(1., |props| props.scale_factor()),
            map.get(LeftY).map_or(1., |props| props.scale_factor()),
        ),
        BottomXRightY => (
            map.get(BottomX).map_or(1., |props| props.scale_factor()),
            map.get(RightY).map_or(1., |props| props.scale_factor()),
        ),
        TopXLeftY => (
            map.get(TopX).map_or(1., |props| props.scale_factor()),
            map.get(LeftY).map_or(1., |props| props.scale_factor()),
        ),
        TopXRightY => (
            map.get(TopX).map_or(1., |props| props.scale_factor()),
            map.get(RightY).map_or(1., |props| props.scale_factor()),
        ),
    }
}

// XXX :-1: to intra-crate privacy rules
/// Private
trait ScaleFactorTrait {
    /// Private
    fn scale_factor(&self) -> f64;
}

#[cfg(test)]
mod test {
    #[test]
    fn version() {
        if let Ok(version) = super::version() {
            assert!(version.major >= 4);
        } else {
            println!("Gnuplot not installed.");
        }
    }

    #[test]
    fn test_parse_version_on_valid_string() {
        let string = "gnuplot 5.0 patchlevel 7";
        let version = super::parse_version(&string).unwrap();
        assert_eq!(5, version.major);
        assert_eq!(0, version.minor);
        assert_eq!("7", &version.patch);
    }

    #[test]
    fn test_parse_gentoo_version() {
        let string = "gnuplot 5.2 patchlevel 5a (Gentoo revision r0)";
        let version = super::parse_version(&string).unwrap();
        assert_eq!(5, version.major);
        assert_eq!(2, version.minor);
        assert_eq!("5a", &version.patch);
    }

    #[test]
    fn test_parse_version_returns_error_on_invalid_strings() {
        let strings = [
            "",
            "foobar",
            "gnuplot 50 patchlevel 7",
            "gnuplot 5.0 patchlevel",
            "gnuplot foo.bar patchlevel 7",
        ];
        for string in &strings {
            assert!(super::parse_version(string).is_err());
        }
    }
}
