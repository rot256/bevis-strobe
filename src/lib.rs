#![no_std]

mod strobe;

use bevis::{Hasher, Sponge, SpongeTranscript};

use strobe::Strobe128;

use rand_core::{impls, CryptoRng, RngCore};

pub type StrobeTranscript = SpongeTranscript<Strobe>;

#[derive(Debug)]
pub struct Strobe {
    strobe: Strobe128,
}

impl Strobe {
    pub fn new(protocol_label: &'static str) -> StrobeTranscript {
        SpongeTranscript::new(
            Strobe {
                strobe: Strobe128::new(protocol_label.as_bytes()),
            }
        )
    }

    fn read(&mut self, buf: &mut [u8]) {
        self.strobe.bevis_squeeze(buf)
    }
}

impl Hasher for Strobe {
    fn write(&mut self, buf: &[u8]) {
        self.strobe.bevis_absorb(buf)
    }
}

impl RngCore for Strobe {
    fn next_u32(&mut self) -> u32 {
        impls::next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_fill(self)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.read(dest);
        Ok(())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.read(dest)
    }
}

impl CryptoRng for Strobe {}

impl Sponge for Strobe {}
