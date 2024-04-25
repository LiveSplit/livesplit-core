cfg_if::cfg_if! {
    if #[cfg(all(feature = "std", not(test)))] {
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
        pub mod f32 {
            pub use libm::{fabsf as abs, powf};
        }
    }
}
