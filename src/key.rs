//! Key (or legend)

use crate::Script;

/// KeyProperties of the key.
///
/// Modified through [`configure_key`].
///
/// [`configure_key`]: ../struct.Figure.html#method.configure_key
#[derive(Clone, Debug, Default)]
pub struct KeyProperties {
    boxed: bool,
    hidden: bool,
    justification: Option<Justification>,
    order: Option<Order>,
    position: Option<Position>,
    stacked: Option<Stacked>,
    title: Option<&'static str>,
}

impl KeyProperties {
    /// Hides the key
    pub fn hide(mut self) -> KeyProperties {
        self.hidden = true;

        self
    }

    /// Shows the key
    ///
    /// **Note** The key is shown by default
    pub fn show(mut self) -> KeyProperties {
        self.hidden = false;

        self
    }

    /// Should the key be surrounded by a box or not?
    ///
    /// **Note** The key is not boxed by default
    pub fn boxed(mut self, boxed: Boxed) -> KeyProperties {
        self.boxed = boxed.into();

        self
    }

    /// Changes the justification of the text of each entry
    ///
    /// **Note** The text is `RightJustified` by default
    pub fn justification(mut self, justification: Justification) -> KeyProperties {
        self.justification = Some(justification);

        self
    }

    /// How to order each entry
    ///
    /// **Note** The default order is `TextSample`
    pub fn order(mut self, order: Order) -> KeyProperties {
        self.order = Some(order);

        self
    }

    /// Selects where to place the key
    ///
    /// **Note** By default, the key is placed `Inside(Vertical::Top, Horizontal::Right)`
    pub fn position(mut self, position: Position) -> KeyProperties {
        self.position = Some(position);

        self
    }

    /// Changes how the entries of the key are stacked
    pub fn stacked(mut self, stacked: Stacked) -> KeyProperties {
        self.stacked = Some(stacked);

        self
    }

    /// Set the title
    pub fn title(mut self, title: &'static str) -> KeyProperties {
        self.title = Some(title);

        self
    }
}

impl Script for KeyProperties {
    fn script(&self) -> String {
        let mut script = if self.hidden {
            return String::from("set key off\n");
        } else {
            String::from("set key on ")
        };

        match self.position {
            None => {}
            Some(Position::Inside(v, h)) => script.push_str(&format!(
                "inside {} {} ",
                Into::<&'static str>::into(v),
                Into::<&'static str>::into(h)
            )),
            Some(Position::Outside(v, h)) => script.push_str(&format!(
                "outside {} {} ",
                Into::<&'static str>::into(v),
                Into::<&'static str>::into(h)
            )),
        }

        if let Some(stacked) = self.stacked {
            script.push_str(Into::<&'static str>::into(stacked));
            script.push(' ');
        }

        if let Some(justification) = self.justification {
            script.push_str(Into::<&'static str>::into(justification));
            script.push(' ');
        }

        if let Some(order) = self.order {
            script.push_str(Into::<&'static str>::into(order));
            script.push(' ');
        }

        if let Some(ref title) = self.title {
            script.push_str(&format!("title '{}' ", title))
        }

        if self.boxed {
            script.push_str("box ")
        }

        script.push('\n');
        script
    }
}

/// Whether the key is surrounded by a box or not
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum Boxed {
    No,
    Yes,
}

impl Into<bool> for Boxed {
    fn into(self) -> bool {
        match self {
            Boxed::Yes => true,
            Boxed::No => false,
        }
    }
}

/// Horizontal position of the key
#[derive(Clone, Copy, Debug)]
pub enum Horizontal {
    /// Center of the figure
    Center,
    /// Left border of the figure
    Left,
    /// Right border of the figure
    Right,
}

impl From<Horizontal> for &'static str {
    fn from(horizontal: Horizontal) -> Self {
        match horizontal {
            Horizontal::Center => "center",
            Horizontal::Left => "left",
            Horizontal::Right => "right",
        }
    }
}

/// Text justification of the key
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum Justification {
    Left,
    Right,
}

impl From<Justification> for &'static str {
    fn from(justification: Justification) -> Self {
        match justification {
            Justification::Left => "Left",
            Justification::Right => "Right",
        }
    }
}

/// Order of the elements of the key
#[derive(Clone, Copy, Debug)]
pub enum Order {
    /// Sample first, then text
    SampleText,
    /// Text first, then sample
    TextSample,
}

impl From<Order> for &'static str {
    fn from(order: Order) -> Self {
        match order {
            Order::TextSample => "noreverse",
            Order::SampleText => "reverse",
        }
    }
}

/// Position of the key
// TODO XY position
#[derive(Clone, Copy, Debug)]
pub enum Position {
    /// Inside the area surrounded by the four (BottomX, TopX, LeftY and RightY) axes
    Inside(Vertical, Horizontal),
    /// Outside of that area
    Outside(Vertical, Horizontal),
}

/// How the entries of the key are stacked
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum Stacked {
    Horizontally,
    Vertically,
}

impl From<Stacked> for &'static str {
    fn from(stacked: Stacked) -> Self {
        match stacked {
            Stacked::Horizontally => "horizontal",
            Stacked::Vertically => "vertical",
        }
    }
}

/// Vertical position of the key
#[derive(Clone, Copy, Debug)]
pub enum Vertical {
    /// Bottom border of the figure
    Bottom,
    /// Center of the figure
    Center,
    /// Top border of the figure
    Top,
}

impl From<Vertical> for &'static str {
    fn from(vertical: Vertical) -> Self {
        match vertical {
            Vertical::Bottom => "bottom",
            Vertical::Center => "center",
            Vertical::Top => "top",
        }
    }
}
