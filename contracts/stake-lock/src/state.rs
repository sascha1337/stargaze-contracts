use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Stake {
    /// Address of the staker
    pub owner: Addr,
    /// Validator to stake to
    pub validator: Addr,
    /// Time when lockup period ends
    pub end_time: Expiration,
    /// Amount of tokens to stake
    pub amount: Uint128,
    /// This is the minimum amount we will pull out to reinvest + claim
    pub min_withdrawal: Uint128,
}

pub const STAKE: Item<Stake> = Item::new("stake");
