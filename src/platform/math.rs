cfg_if::cfg_if! {
    if #[cfg(feature = "std")] {
        pub mod f64 {
            #[inline(always)]
            pub fn abs(x: f64) -> f64 {
                x.abs()
            }

            #[inline(always)]
            pub fn floor(x: f64) -> f64 {
                x.floor()
            }
        }
    } else {
        pub mod f64 {
            pub use libm::{fabs as abs, floor as floor};
        }
    }
}
