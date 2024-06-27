#![no_std]
#![no_main]

use core::default::Default;

use alloy_core::primitives::Address;
use contract_derive::contract;
use eth_riscv_runtime::types::Mapping;

#[derive(Default)]
pub struct ERC20 {
  balance: Mapping<Address, u64>,
}

#[contract]
impl ERC20 {
  pub fn balance_of(&self, owner: Address) -> u64 {
    self.balance.read(owner)
  }

  pub fn transfer(&self, from: Address, to: Address, value: u64) {
    let from_balance = self.balance.read(from);
    let to_balance = self.balance.read(to);

    if from == to || from_balance < value {
      revert();
    }

    self.balance.write(from, from_balance - value);
    self.balance.write(to, to_balance + value);
  }

  pub fn mint(&self, to: Address, value: u64) {
    let to_balance = self.balance.read(to);
    self.balance.write(to, to_balance + value);
  }
}
