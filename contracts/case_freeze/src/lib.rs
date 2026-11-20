#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Authority {
    CourtOrder,
    RegulatoryDirective,
    NfiuInstruction,
}

#[contract]
pub struct CaseFreeze;

#[contractimpl]
#[allow(unused_variables)]
impl CaseFreeze {
    pub fn init(admin: Address, token: Address) {
        unimplemented!()
    }

    pub fn open_case(officer: Address, case_id: u64, authority: Authority, reference_hash: BytesN<32>) {
        unimplemented!()
    }

    pub fn close_case(officer: Address, case_id: u64) {
        unimplemented!()
    }

    /// Fails unless the case is open.
    pub fn freeze(officer: Address, case_id: u64, account: Address) {
        unimplemented!()
    }

    /// Fails unless the case is open.
    pub fn unfreeze(officer: Address, case_id: u64, account: Address) {
        unimplemented!()
    }

    /// Fails unless the case is open.
    pub fn clawback(officer: Address, case_id: u64, account: Address, amount: i128) {
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
        env.register(CaseFreeze, ());
    }
}
