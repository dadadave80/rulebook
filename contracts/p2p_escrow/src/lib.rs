#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN};

#[contract]
pub struct P2pEscrow;

#[contractimpl]
#[allow(unused_variables)]
impl P2pEscrow {
    pub fn init(admin: Address, token: Address, attester_public_key: BytesN<32>) {
        unimplemented!()
    }

    pub fn open_trade(seller: Address, buyer: Address, amount: i128, expiry: u64) -> u64 {
        unimplemented!()
    }

    /// Verifies the attester's ed25519 signature over the trade ID, buyer and expiry before releasing.
    pub fn release(trade_id: u64, attestation: Bytes, signature: BytesN<64>) {
        unimplemented!()
    }

    /// Allowed only after expiry.
    pub fn refund(trade_id: u64) {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn registers() {
        let env = Env::default();
        env.register(P2pEscrow, ());
    }
}
