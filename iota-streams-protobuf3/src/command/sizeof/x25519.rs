use anyhow::Result;

use super::Context;
use crate::{
    command::X25519,
    types::NBytes,
};
use iota_streams_core_edsig::key_exchange::x25519;

impl<'a, F> X25519<&'a x25519::StaticSecret, &'a x25519::PublicKey> for Context<F>
{
    fn x25519(&mut self, sk: &x25519::StaticSecret, pk: &x25519::PublicKey) -> Result<&mut Self> {
        // only shared secret is absorbed externally
        self.size += 0;
        Ok(self)
    }
}

impl<'a, F> X25519<&'a x25519::EphemeralSecret, &'a x25519::PublicKey> for Context<F>
{
    fn x25519(&mut self, sk: &x25519::EphemeralSecret, pk: &x25519::PublicKey) -> Result<&mut Self> {
        // shared secret is absorbed externally
        self.size += 0;
        Ok(self)
    }
}

impl<'a, F> X25519<&'a x25519::PublicKey, &'a NBytes> for Context<F>
{
    fn x25519(&mut self, pk: &x25519::PublicKey, key: &NBytes) -> Result<&mut Self> {
        self.size += 32 + key.0.len();
        Ok(self)
    }
}