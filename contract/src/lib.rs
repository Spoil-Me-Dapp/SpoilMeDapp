use beneficiary::Beneficiary;
use donation::Donation;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

mod beneficiary;
mod donation;
// dev-1675652781320-72401773996174

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub beneficiaries: Vec<Beneficiary>,
    pub donations: Vec<Donation>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            beneficiaries: Vec::new(),
            donations: Vec::new(),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private] // Public - but only callable by env::current_account_id()
    pub fn init() -> Self {
        Self {
            beneficiaries: Vec::new(),
            donations: Vec::new(),
        }
    }
}
