use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone, Debug)]
pub struct Beneficiary {
    pub account_id: AccountId,
    pub total_amount: U128,
}

#[near_bindgen]
impl Contract {
    // Public function that returns the unordered map of beneficiaries
    pub fn get_beneficiaries(&self) -> Vec<Beneficiary> {
        self.beneficiaries.clone()
    }

    pub fn add_beneficiciary(&mut self) {
        let beneficiary_account_id = env::signer_account_id();

        let beneficiary_object = Beneficiary {
            account_id: beneficiary_account_id,
            total_amount: U128(0),
        };
        self.beneficiaries.push(beneficiary_object);
    }

    pub fn get_beneficiary_by_account_id(&self, account_id: AccountId) -> Option<Beneficiary> {
        for beneficiary in self.beneficiaries.iter() {
            if beneficiary.account_id == account_id {
                return Some(beneficiary.clone());
            }
        }
        None
    }

    #[private]
    pub fn remove_beneficiary(&mut self, account_id: AccountId) {
        self.beneficiaries
            .retain(|beneficiary| beneficiary.account_id != account_id);
    }
}
