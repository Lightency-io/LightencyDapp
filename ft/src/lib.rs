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
use near_sdk::{env,Gas, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue,ext_contract,Promise,PromiseError};

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_ft)]
pub trait Rewarder {
    #[payable]
    fn add_staker(&mut self, account: String, amount: u128);
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/png;base64,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";

fn assert_self() {
    assert_eq!(
        env::current_account_id(),
        env::signer_account_id(),
        "Can only be called by the token contract"
    );
}

fn assert() {
    if env::signer_account_id().to_string() != env::current_account_id().to_string() {
        if env::signer_account_id().to_string() != "lightency_contract.testnet".to_string() {
            panic!("You're not authorised to execute this function");
        }
    }
}

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
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&owner_id);
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

    pub fn on_tokens_burned(&mut self, account_id: AccountId, amount: U128) {
        near_contract_standards::fungible_token::events::FtBurn {
            owner_id: &account_id,
            amount: &amount,
            memo: Some("Tokens are burned"),
        }
        .emit();
    }

    pub fn on_tokens_minted(&mut self, account_id: AccountId, amount: u128) {
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &account_id,
            amount: &amount.into(),
            memo: Some("Tokens are minted"),
        }
        .emit();
    }

    pub fn mint_token(&mut self, account_id: AccountId, amount: u128) {
            self.token.internal_deposit(&account_id, amount.into());
            self.on_tokens_minted(account_id, amount);
    }

    pub fn burn_token(&mut self, account_id: AccountId, amount: u128) {
        self.token.internal_withdraw(&account_id, amount.into());
    }

    // stake function 
    pub fn stake(&self, amount: u128) -> Promise {
        let account_reward = "rewarder_contract.testnet".to_string().try_into().unwrap();
        let p = ext_ft::ext(account_reward)
        .with_static_gas(Gas(5 * TGAS))
        .add_staker(env::signer_account_id().to_string(), amount);

        return p.then( // Create a promise to callback staking_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(3 * TGAS))
            .staking_callback()
        )
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
