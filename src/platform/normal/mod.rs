#![allow(missing_docs)]

pub use chrono::{DateTime, Duration, Local, Utc};
pub use indexmap;

cfg_if::cfg_if! {
    // We can't use std's Instant as it's insufficiently specified. It neither
    // guarantees "real time" nor does it guarantee measuring "uptime" (the time
    // the OS has been awake rather than suspended), meaning that you can't
    // actually rely on it in practice. In livesplit-core we definitely want
    // real time rather than uptime. The problem is mostly Linux. POSIX intends
    // for `CLOCK_MONOTONIC` to be real time, but this wasn't correctly
    // implemented in Linux and due to backwards compatibility concerns they
    // were never able to fix it properly. Thus `CLOCK_MONOTONIC` means uptime
    // on Linux whereas on other Unixes it means real time. They however
    // introduced `CLOCK_BOOTTIME` in the Linux kernel 2.6.39 which measures
    // real time. So the solution is to use this on all operating systems that
    // are based on the Linux kernel and fall back to `CLOCK_MONOTONIC` if the
    // kernel is too old and the syscall fails.
    //
    // macOS and iOS actually do the right thing for `CLOCK_MONOTONIC` but Rust
    // actually doesn't use it on iOS and macOS, so we also need to use our
    // custom implementation for those too, but skip `CLOCK_BOOTTIME` as that is
    // Linux specific.
    //
    // On Windows Instant currently measures real time, but we may need to use a
    // custom implementation for it as well in case this ever changes.
    //
    // This list of "Linux like operating systems" has to match libc (and our
    // Cargo.toml):
    // https://github.com/rust-lang/libc/blob/5632705fe1a7858d82609178ba96b13f98f8c2e6/src/unix/mod.rs#L1451-L1454
    if #[cfg(any(
        target_os = "linux",
        target_os = "l4re",
        target_os = "android",
        target_os = "emscripten",
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
                    use std::sync::atomic::{AtomicBool, Ordering};

                    static BOOT_TIME_BROKEN: AtomicBool = AtomicBool::new(false);

                    if !BOOT_TIME_BROKEN.load(Ordering::Relaxed) {
                        if unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut t) } == 0 {
                            return Self { t };
                        } else {
                            BOOT_TIME_BROKEN.store(true, Ordering::Relaxed);
                        }
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
