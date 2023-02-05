use beneficiary::Beneficiary;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

mod beneficiary;
// dev-1675621995087-84833802127813

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub beneficiaries: Vec<Beneficiary>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            beneficiaries: Vec::new(),
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
        }
    }
}
