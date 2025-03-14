#[allow(unused)]
pub use std::sync::RwLock;
use time::UtcOffset;
pub use time::{Duration, OffsetDateTime as DateTime};

cfg_if::cfg_if! {
    // We can't use std's Instant as it's insufficiently specified. It neither
    // guarantees "real time" nor does it guarantee measuring "uptime" (the time
    // the OS has been awake rather than suspended), meaning that you can't
    // actually rely on it in practice. In livesplit-core we definitely want
    // real time rather than uptime. The problem in std is being tracked here:
    // https://github.com/rust-lang/rust/issues/87906
    //
    // Various operating systems are problematic:
    //
    // # Linux, BSD and other Unixes
    //
    // POSIX intends for `CLOCK_MONOTONIC` to be real time, but this wasn't
    // correctly implemented in Linux and due to backwards compatibility
    // concerns they were never able to fix it properly. Thus `CLOCK_MONOTONIC`
    // means uptime on Linux whereas on other Unixes it means real time (the BSD
    // family). As a solution the Linux kernel provides `CLOCK_BOOTTIME` to
    // measure the real time.
    //
    // # macOS and iOS
    //
    // For macOS and iOS there is `mach_continuous_time` which does the right
    // thing, but is not recommended by the documentation. The documentation
    // recommends `clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW)` instead.
    //
    // # Windows
    //
    // On Windows std's Instant currently measures real time through
    // `QueryPerformanceCounter`, but we may need to use a custom implementation
    // for it as well in case this ever changes.
    //
    // # Fuchsia
    //
    // Fuchsia is based on the new Zircon kernel. It has `zx_clock_get_boot` to
    // query the real time  and `zx_clock_get_monotonic` to query the uptime.
    //
    // https://fuchsia.dev/reference/syscalls/clock_get_boot
    //
    // They are supposed to be available through `CLOCK_BOOTTIME` and
    // `CLOCK_MONOTONIC` respectively, just like on Linux.
    //
    // # WASI
    //
    // https://github.com/WebAssembly/WASI/blob/5ab83a68d4eb4f218a898ed03b963b7393caaedc/phases/snapshot/docs.md#variant-cases
    //
    // WASI seems to under specify its `monotonic` a bit, but says that it `is
    // defined as a clock measuring real time`, making it sound like a compliant
    // implementation should measure the time the OS is suspended as well.
    //
    // Open issue: https://github.com/WebAssembly/wasi-clocks/issues/47
    //
    // # Web
    //
    // In the web we use `performance.now()` which they want to specify as being
    // required to keep counting during any sort of suspends (including both the
    // tab and the OS). The browsers currently do however not implement this
    // correctly at all, due to all of the same issues that we are facing here.
    //
    // Spec Issue: https://github.com/w3c/hr-time/issues/115
    //
    // Chromium Bug Ticket:
    // https://bugs.chromium.org/p/chromium/issues/detail?id=1206450
    //
    // Chrome Implementation (the various `time_*.cpp`):
    // https://github.com/chromium/chromium/tree/d7da0240cae77824d1eda25745c4022757499131/base/time
    //
    // Firefox Bug Ticket: https://bugzilla.mozilla.org/show_bug.cgi?id=1709767
    //
    // Firefox Implementation (the various `TimeStamp_*.cpp`):
    // https://github.com/mozilla/gecko-dev/blob/08c493902519265d570250c8e7ce575c8cd6f5b5/mozglue/misc
    //
    // WebKit Bug Ticket: https://bugs.webkit.org/show_bug.cgi?id=225610
    //
    // WebKit Implementation:
    // https://github.com/WebKit/WebKit/blob/79daff42b19103a15340e4005ac90facf1fc46c9/Source/WTF/wtf/CurrentTime.cpp#L268
    //
    //
    // # Final Notes
    //
    // We therefore need a custom implementation for all "Linux like operating
    // systems" as well as macOS and iOS. The following list has to match libc
    // (and our Cargo.toml):
    // https://github.com/rust-lang/libc/blob/5632705fe1a7858d82609178ba96b13f98f8c2e6/src/unix/mod.rs#L1451-L1454
    //
    // We however remove emscripten from this list as it's not actually based on
    // the Linux kernel and instead has its own implementation in JavaScript
    // where it actually errors out on `CLOCK_BOOTTIME`:
    // https://github.com/emscripten-core/emscripten/blob/9bdb310b89472a0f4d64f36e4a79273d8dc7fa98/system/lib/libc/emscripten_time.c#L50-L57
    //
    // And we add Fuchsia to this list as it's based on the Zircon kernel which
    // uses `CLOCK_BOOTTIME` in the same way as Linux.
    if #[cfg(any(
        target_os = "linux",
        target_os = "l4re",
        target_os = "android",
        target_os = "fuchsia",
    ))] {
        use core::{mem::MaybeUninit, ops::Sub};

        #[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
        #[repr(transparent)]
        pub struct Instant(Duration);

        impl Instant {
            /// Accesses the current point in time.
            pub fn now() -> Self {
                // SAFETY: This is safe because we pass a valid pointer to a
                // `timespec` to `clock_gettime` and check the return value.
                unsafe {
                    let mut t = MaybeUninit::uninit();

                    if libc::clock_gettime(libc::CLOCK_BOOTTIME, t.as_mut_ptr()) != 0 {
                        panic!("clock_gettime doesn't work.");
                    }

                    let t = t.assume_init_ref();
                    Self(Duration::new(t.tv_sec as _, t.tv_nsec as _))
                }
            }
        }

        impl Sub for Instant {
            type Output = Duration;

            #[inline]
            fn sub(self, rhs: Instant) -> Duration {
                self.0 - rhs.0
            }
        }
    } else if #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "watchos",
    ))] {
        use core::ops::Sub;

        unsafe extern "C" {
            fn clock_gettime_nsec_np(clock_id: libc::clockid_t) -> u64;
        }

        #[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
        #[repr(transparent)]
        pub struct Instant(u64);

        impl Instant {
            /// Accesses the current point in time.
            #[inline]
            pub fn now() -> Self {
                // SAFETY: This is always safe.
                unsafe {
                    Self(clock_gettime_nsec_np(libc::CLOCK_MONOTONIC_RAW))
                }
            }
        }

        impl Sub for Instant {
            type Output = Duration;

            #[inline]
            fn sub(self, rhs: Instant) -> Duration {
                Duration::nanoseconds(self.0 as i64 - rhs.0 as i64)
            }
        }
    } else {
        use core::ops::Sub;

        #[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug)]
        #[repr(transparent)]
        pub struct Instant(std::time::Instant);

        impl Instant {
            /// Accesses the current point in time.
            #[inline]
            pub fn now() -> Self {
                Self(std::time::Instant::now())
            }
        }

        impl Sub for Instant {
            type Output = Duration;

            #[inline]
            fn sub(self, rhs: Instant) -> Duration {
                time::ext::InstantExt::signed_duration_since(&self.0, rhs.0)
            }
        }
    }
}

#[inline]
pub fn utc_now() -> DateTime {
    DateTime::now_utc()
}

pub fn to_local(date_time: DateTime) -> DateTime {
    date_time.to_offset(UtcOffset::local_offset_at(date_time).unwrap_or(UtcOffset::UTC))
}
