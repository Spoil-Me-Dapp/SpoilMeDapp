use crate::beneficiary::Beneficiary;
use crate::Contract;
use crate::ContractExt;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, Promise};

pub const STORAGE_COST: u128 = 1_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone, Debug)]
pub struct Donation {
    pub account_id: AccountId,
    pub total_amount: U128,
    pub beneficiary: Beneficiary,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn donate(&mut self, beneficiary: String) -> U128 {
        let beneficiary_account_id: AccountId = beneficiary.parse().unwrap();

        let beneficiary_object = self.get_beneficiary_by_account_id(beneficiary_account_id.clone());

        // Check if the beneficiary is present in the vector of beneficiaries
        if beneficiary_object.is_none() {
            log!("Error: Beneficiary not found in the list of beneficiaries");
            return U128(0);
        }

        let donor: AccountId = env::predecessor_account_id();
        let donation_amount: Balance = env::attached_deposit();

        let to_transfer: Balance = if donation_amount == 0 {
            // This is the user's first donation, lets register it, which increases storage
            assert!(
                donation_amount > STORAGE_COST,
                "Attach at least {} yoctoNEAR",
                STORAGE_COST
            );

            // Subtract the storage cost to the amount to transfer
            donation_amount - STORAGE_COST
        } else {
            donation_amount
        };

        let donation = Donation {
            account_id: donor.clone(),
            total_amount: U128(to_transfer),
            beneficiary: beneficiary_object.unwrap(),
        };

        self.donations.push(donation);

        log!(
            "Thank you {} for donating {}!",
            donor.clone(),
            donation_amount,
        );

        // Send the money to the beneficiary
        Promise::new(beneficiary_account_id.clone()).transfer(to_transfer);

        // Return the total amount donated so far
        U128(donation_amount)
    }
}
