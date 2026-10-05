use core::hash::Hasher;

/// A fast non-cryptographic hasher used for internal hashing.
pub(crate) struct FxHasher(u64);

impl FxHasher {
    const SEED: u64 = 0x517cc1b727220a95;

    #[inline]
    pub const fn new() -> Self {
        Self(Self::SEED)
    }
}

impl Default for FxHasher {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(Self::SEED);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        let [a, b]: [u64; 2] = bytemuck::cast(i);
        self.write_u64(a);
        self.write_u64(b);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write(bytemuck::bytes_of(&i))
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let (_, chunks, rem) = bytemuck::pod_align_to::<_, [u8; 8]>(bytes);
        for chunk in chunks {
            self.write_u64(bytemuck::cast(*chunk));
        }
        let (_, chunks, rem) = bytemuck::pod_align_to::<_, [u8; 4]>(rem);
        for chunk in chunks {
            self.write_u32(bytemuck::cast(*chunk));
        }
        for byte in rem {
            self.write_u8(*byte);
        }
    }
}
