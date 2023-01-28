#![no_std]

mod strobe;

use bevis::{Hasher, Sampler, Sponge, SpongeTranscript};

use strobe::Strobe128;

use rand_core::{impls, CryptoRng, RngCore};

pub type Transcript = SpongeTranscript<Strobe128>;

impl Hasher for Strobe128 {
    fn write(&mut self, buf: &[u8]) {
        self.bevis_absorb(buf)
    }
}

impl RngCore for Strobe128 {
    fn next_u32(&mut self) -> u32 {
        impls::next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_fill(self)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.bevis_squeeze(dest);
        Ok(())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.bevis_squeeze(dest)
    }
}

impl CryptoRng for Strobe128 {}

impl Sponge for Strobe128 {
    fn new(protocol_label: &str) -> Self {
        Strobe128::new(protocol_label.as_bytes())
    }
}

impl Sampler for Strobe128 {}
