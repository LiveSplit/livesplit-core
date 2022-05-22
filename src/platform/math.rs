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

        pub mod f32 {
            #[inline(always)]
            pub fn abs(x: f32) -> f32 {
                x.abs()
            }

            #[inline(always)]
            pub fn powf(x: f32, y: f32) -> f32 {
                x.powf(y)
            }
        }
    } else {
        pub mod f64 {
            pub use libm::{fabs as abs, floor};
        }

        pub mod f32 {
            pub use libm::{fabsf as abs, powf};
        }
    }
}
