#![no_std]
use soroban_sdk::{contract, contractimpl, Address, BytesN};

#[contract]
pub struct ReserveGuard;

#[contractimpl]
#[allow(unused_variables)]
impl ReserveGuard {
    /// `min_reserve_bps` is the minimum reserve ratio in basis points of supply
    /// (10_000 = 1:1), taken from the jurisdiction's rule set.
    pub fn init(admin: Address, token: Address, custodian_public_key: BytesN<32>, min_reserve_bps: u32) {
        unimplemented!()
    }

    pub fn attest_reserves(amount: i128, timestamp: u64, signature: BytesN<64>) {
        unimplemented!()
    }

    /// Fails unless reserves × 10_000 >= min_reserve_bps × (supply + amount).
    pub fn mint(to: Address, amount: i128) {
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
        env.register(ReserveGuard, ());
    }
}
