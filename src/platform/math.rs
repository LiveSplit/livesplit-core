pub mod f32 {
    cfg_if::cfg_if! {
        if #[cfg(all(feature = "std"))] {
            #[inline(always)]
            #[allow(clippy::missing_const_for_fn)] // Can't do this for the libm counterpart.
            pub fn abs(x: f32) -> f32 {
                x.abs()
            }
        } else {
            pub use libm::fabsf as abs;
        }
    }

    // FIXME: For our tests we need to ensure that the `powf` function behaves
    // consistently across all platforms. Ideally we would only use it during
    // testing, but cfg(test) apparently doesn't work for integration tests. We
    // ensure consistent results by increasing the precision for the powf by
    // using f64 internally.
    #[inline(always)]
    pub fn stable_powf(x: f32, y: f32) -> f32 {
        super::f64::powf(x as f64, y as f64) as f32
    }
}

mod f64 {
    cfg_if::cfg_if! {
        if #[cfg(all(feature = "std"))] {
            #[inline(always)]
            #[allow(clippy::missing_const_for_fn)] // Can't do this for the libm counterpart.
            pub fn powf(x: f64, y: f64) -> f64 {
                x.powf(y)
            }
        } else {
            pub use libm::pow as powf;
        }
    }
}
