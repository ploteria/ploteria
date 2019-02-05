//! Traits

/// Types that can be plotted
pub trait Data {
    /// Convert the type into a double precision float
    fn f64(self) -> f64;
}

/// Overloaded `plot` method
pub trait Plot<This> {
    /// The properties associated to the plot
    type Properties;

    /// Plots some `data` with some `configuration`
    fn plot<F>(self, this: This, function: F) -> Self
    where
        F: FnOnce(Self::Properties) -> Self::Properties;
}
