/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/

use std::convert::TryInto;

use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{
    env, ext_contract, log, near_bindgen, AccountId, Balance, Gas, PanicOnDefault, Promise,
    PromiseError, PromiseOrValue,
};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_ft)]
pub trait Rewarder {
    #[payable]
    fn add_staker(&mut self, account: String, amount: u128);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    contract_owner: AccountId, 
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/jpeg;base64,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";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: u128) -> Self {
        Self::new(
            owner_id,
            total_supply.into(),
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Lights".to_string(),
                symbol: "LTS".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 8,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(owner_id: AccountId, total_supply: u128, metadata: FungibleTokenMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            contract_owner:owner_id.clone(),
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id.clone());
        this.token.internal_deposit(&owner_id, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &total_supply.into(),
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: U128) {
        near_contract_standards::fungible_token::events::FtBurn {
            owner_id: &account_id,
            amount: &amount,
            memo: Some("Tokens are burned"),
        }
        .emit();
    }

    fn on_tokens_minted(&mut self, account_id: AccountId, amount: u128) {
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &account_id,
            amount: &amount.into(),
            memo: Some("Tokens are minted"),
        }
        .emit();
    }
    
    #[payable]
    pub fn mint_token(&mut self, account_id: AccountId, amount: u128) {
        assert_eq!(env::predecessor_account_id() , self.contract_owner,"Only the the contract owner can mint LTS tokens");
        self.storage_deposit(Some(account_id.clone()),Some(true));
        self.token.internal_deposit(&account_id.clone(), amount.into());
        self.on_tokens_minted(account_id, amount);
    }

    pub fn burn_token(&mut self, account_id: AccountId, amount: u128) {
        assert_eq!(env::predecessor_account_id() , self.contract_owner,"Only the the contract owner can burn LTS tokens");
        self.token.internal_withdraw(&account_id, amount.into());
        self.on_tokens_burned(account_id, amount.into());
    }

    pub fn change_contract_owner(&mut self, new_owner:AccountId) {
        assert!(env::signer_account_id() == self.contract_owner,"You don't have the permission to change the contract_owner");
        self.contract_owner = new_owner;
    }

    pub fn get_contract_owner(&self) -> AccountId {
        self.contract_owner.clone()
    }

    // stake function
    pub fn stake(&self, amount: u128) -> Promise {
        let account_reward = "rewarder_contract.testnet".to_string().try_into().unwrap();
        let p = ext_ft::ext(account_reward)
            .with_static_gas(Gas(5 * TGAS))
            .add_staker(env::signer_account_id().to_string(), amount);

        return p.then(
            // Create a promise to callback staking_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(3 * TGAS))
                .staking_callback(),
        );
    }
    #[private] // Public - but only callable by env::current_account_id()
    pub fn staking_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>) {
        // Check if the promise succeeded
        if call_result.is_err() {
            panic!("There was an error contacting the rewarder contract");
        }
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    //testing internal functions
    // start
    #[test]
    fn test_get_total_supply() {
        //testing the total supply

        //initiliazing the contract
        let signer: AccountId = env::signer_account_id();
        let contract = Contract::new_default_meta(signer, 100);

        //getting the balance of owner == total supply
        let balance = contract
            .token
            .internal_unwrap_balance_of(&env::signer_account_id());
        //assertion
        assert_eq!(balance, 100);
    }

    #[test]
    fn test_register() {
        //testing the registration of the account in the contract

        //initiliazing the contract
        let signer: AccountId = env::signer_account_id();
        let mut contract = Contract::new_default_meta(signer, 100);

        //setting up the account
        let issam: AccountId = "issameths.testnet".parse().unwrap();

        //registring the account
        contract.token.internal_register_account(&issam);

        //getting the balance of issam == 0
        let balance = contract.token.internal_unwrap_balance_of(&issam);

        //assertion
        assert_eq!(balance, 0);
    }

    #[test]
    fn test_deposit() {
        //testing the deposit method

        //initiliazing the contract with 100 total supply
        let signer: AccountId = env::signer_account_id();
        let mut contract = Contract::new_default_meta(signer, 100);

        //setting up the account Id
        let issam: AccountId = "issameths.testnet".parse().unwrap();

        //registring the account
        contract.token.internal_register_account(&issam);

        // depositing tokens into issam
        contract.token.internal_deposit(&issam, 20);

        //getting balance of issam
        let balance_issam = contract.token.internal_unwrap_balance_of(&issam);

        //assertion : check if issam does indeed own 20 lts now
        assert_eq!(balance_issam, 20);
    }

    #[test]
    fn test_withdraw() {
        //testing the withdraw method

        //initiliazing the contract with 100 total supply
        let signer: AccountId = env::signer_account_id();
        let mut contract = Contract::new_default_meta(signer, 100);

        //setting up the account Id
        let issam: AccountId = "issameths.testnet".parse().unwrap();

        //registring the account
        contract.token.internal_register_account(&issam);

        //depositting tokens into issam
        contract.token.internal_deposit(&issam, 30);

        //withdrawing(burning) tokens from issam
        contract.token.internal_withdraw(&issam, 10);

        //getting balance of issam
        let balance_issam = contract.token.internal_unwrap_balance_of(&issam);

        //assertion
        assert_eq!(balance_issam, 20);
    }
}
