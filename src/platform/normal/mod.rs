#![allow(missing_docs)]

pub use chrono::{DateTime, Duration, Local, Utc};
pub use indexmap;

cfg_if::cfg_if! {
    // We can't use std's Instant as it's insufficiently specified. It neither
    // guarantees "real time" nor does it guarantee measuring "uptime" (the time
    // the OS has been awake rather than suspended), meaning that you can't
    // actually rely on it in practice. In livesplit-core we definitely want
    // real time rather than uptime. Various operating systems are problematic:
    //
    // # Linux, BSD and other Unixes
    //
    // POSIX intends for `CLOCK_MONOTONIC` to be real time, but this wasn't
    // correctly implemented in Linux and due to backwards compatibility
    // concerns they were never able to fix it properly. Thus `CLOCK_MONOTONIC`
    // means uptime on Linux whereas on other Unixes it means real time (the BSD
    // family). They however introduced `CLOCK_BOOTTIME` in the Linux kernel
    // 2.6.39 which measures real time. So the solution is to use this on all
    // operating systems that are based on the Linux kernel and fall back to
    // `CLOCK_MONOTONIC` if the kernel is too old and the syscall fails.
    //
    // # macOS and iOS
    //
    // macOS and iOS actually do the right thing for `CLOCK_MONOTONIC` but Rust
    // actually doesn't use it on iOS and macOS, so we also need to use our
    // custom implementation for those too, but skip `CLOCK_BOOTTIME` as that is
    // Linux specific. `clock_gettime` itself however has only been available
    // since macOS 10.12 (Sierra) and iOS 10 which both got released in
    // September 2016. While there is `mach_continuous_time` which does the same
    // thing, it got introduced in the same update and is not recommended by the
    // documentation, so it doesn't help with this problem.
    //
    // # Windows
    //
    // On Windows std's Instant currently measures real time through
    // `QueryPerformanceCounter`, but we may need to use a custom implementation
    // for it as well in case this ever changes.
    //
    // # Fuchsia
    //
    // Fuchsia is based on the new Zircon kernel. It has two functions for
    // querying the time:
    //
    // zx_clock_get:
    // https://fuchsia.dev/fuchsia-src/reference/syscalls/clock_get
    // zx_clock_get_monotonic:
    // https://fuchsia.dev/fuchsia-src/reference/syscalls/clock_get_monotonic
    //
    // `zx_clock_get_monotonic` specifically calls out that it `does not adjust
    // during sleep` which seems to mean that it doesn't count the time the OS
    // is suspended. This is further evidenced by their libc implementation not
    // treating `CLOCK_BOOTTIME` differently and a bug ticket being linked
    // there:
    // https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/third_party/ulib/musl/src/time/clock_gettime.c;l=40;drc=35e7a15cb21e16f0705560e5812b7a045d42c8a5
    //
    // # WASI
    //
    // https://github.com/WebAssembly/WASI/blob/5ab83a68d4eb4f218a898ed03b963b7393caaedc/phases/snapshot/docs.md#variant-cases
    //
    // WASI seems to underspecify its `monotonic` a bit, but says that it `is
    // defined as a clock measuring real time`, making it sound like a compliant
    // implementation should measure the time the OS is suspended as well.
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
    // https://github.com/emscripten-core/emscripten/blob/0321203d3614a97e4042ffa0c19ab770b1f5aa6c/src/library.js#L1419-L1426
    if #[cfg(any(
        target_os = "linux",
        target_os = "l4re",
        target_os = "android",
        target_os = "macos",
        target_os = "ios",
    ))] {
        use std::{cmp, fmt, ops::Sub};

        #[derive(Copy, Clone)]
        pub struct Instant {
            t: libc::timespec,
        }

        impl PartialEq for Instant {
            fn eq(&self, other: &Instant) -> bool {
                self.t.tv_sec == other.t.tv_sec && self.t.tv_nsec == other.t.tv_nsec
            }
        }

        impl Eq for Instant {}

        impl PartialOrd for Instant {
            fn partial_cmp(&self, other: &Instant) -> Option<cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Instant {
            fn cmp(&self, other: &Instant) -> cmp::Ordering {
                let me = (self.t.tv_sec, self.t.tv_nsec);
                let other = (other.t.tv_sec, other.t.tv_nsec);
                me.cmp(&other)
            }
        }

        impl fmt::Debug for Instant {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Instant")
                    .field("sec", &self.t.tv_sec)
                    .field("nsec", &self.t.tv_nsec)
                    .finish()
            }
        }

        impl Instant {
            /// Accesses the current point in time.
            pub fn now() -> Self {
                let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };

                // `CLOCK_BOOTTIME` is only necessary on Linux.
                #[cfg(not(any(target_os = "macos", target_os = "ios")))]
                {
                    // If it fails it always fails. So technically we could
                    // cache that result, but the Linux kernels that Rust
                    // supports that don't have `CLOCK_BOOTTIME` (2.6.32 to
                    // 2.6.38) are all EOL since at least 2016, so we might as
                    // well always use `CLOCK_BOOTTIME` on Linux. The additional
                    // atomic isn't worth the slight performance penalty and
                    // once Rust bumps the minimal Linux version we may also
                    // just cfg out `CLOCK_MONOTONIC` here.
                    if unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut t) } == 0 {
                        return Self { t };
                    }
                }

                if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t) } != 0 {
                    panic!("clock_gettime doesn't work.");
                }

                Self { t }
            }
        }

        impl Sub for Instant {
            type Output = Duration;

            fn sub(self, rhs: Instant) -> Duration {
                const NSEC_PER_SEC: u64 = 1_000_000_000;

                let s = (self.t.tv_sec, self.t.tv_nsec);
                let r = (rhs.t.tv_sec, rhs.t.tv_nsec);
                let ((max_sec, max_nano), (min_sec, min_nano)) = if s >= r {
                    (s, r)
                } else {
                    (r, s)
                };
                let (secs, nsec) = if max_nano >= min_nano {
                    ((max_sec - min_sec) as u64, (max_nano - min_nano) as u32)
                } else {
                    (
                        (max_sec - min_sec - 1) as u64,
                        max_nano as u32 + (NSEC_PER_SEC as u32) - min_nano as u32,
                    )
                };
                Duration::from_std(std::time::Duration::new(secs, nsec)).unwrap()
            }
        }
    } else {
        pub use std::time::Instant;
    }
}

pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}
