use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone, Debug)]
pub struct Beneficiary {
    pub account_id: AccountId,
    pub first_name: String,
    pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
    // Public function that returns the unordered map of beneficiaries
    pub fn get_beneficiaries(&self) -> Vec<Beneficiary> {
        self.beneficiaries.clone()
    }
}
