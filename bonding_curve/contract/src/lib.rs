use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract,AccountId, Promise, PromiseError};
use near_sdk::{env, near_bindgen, Gas};
use std::f64;

pub const TGAS: u64 = 1_000_000_000_000;

#[ext_contract(ext_ft)]
pub trait Lighttoken {
    fn mint_token(&mut self, account_id: AccountId, amount: u128);
    fn burn_token(&mut self, account_id: AccountId, amount: u128);
    fn storage_deposit (&mut self, account_id: String);
    fn ft_balance_of (&mut self, account_id:String) -> String;
}

#[ext_contract(ext_stable_coin)]
pub trait Stablecoin {
    fn ft_transfer(&mut self,receiver_id:String,amount:String);
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BondingCurve {
    owner_id: AccountId,
    token_price: u128,
    reserve_balance: u128,
    total_supply: u128,
    price_floor: u128,
    a: f64,
    b: f64,
    c: f64,
}

impl Default for BondingCurve {
    fn default() -> Self {
        panic!("Bonding curve contract must be initialized before use")
    }
}

#[near_bindgen]
impl BondingCurve {
    #[init]
    pub fn new(
        token_price: u128,
        reserve_balance: u128,
        price_floor: u128,
        a: f64,
        b:f64,
        c:f64,
    ) -> Self {
        Self {
            owner_id: "treasurydao.testnet".to_string().try_into().unwrap(),
            token_price,
            reserve_balance,
            total_supply: 0,
            price_floor,
            a,
            b,
            c
        }
    }

    //Function to buy tokens from the bonding curve
    #[payable]
    pub fn buy(&mut self, num_tokens:u128) {
        let price_for_tokens = self.price_to_mint(num_tokens);
        self.total_supply += num_tokens ;
        self.reserve_balance += price_for_tokens ;
        env::log_str(&self.total_supply.to_string());
        self.token_price = ((self.a/(1.0+(-self.b*(self.total_supply as f64 / 100000000.0)-self.c).exp()))* 1000000.0) as u128 ;
        env::log_str(&self.total_supply.to_string());
        // cross call from lts contract
    }

    //Burn tokens in exchange for coins
    pub fn sell(&mut self, num_tokens: u128) {
        let reward_to_return = self.reward_for_burn(num_tokens);
        self.total_supply -= num_tokens;
        self.reserve_balance -= reward_to_return;
        self.token_price = ((self.a/(1.0+(-self.b*(self.total_supply as f64 / 100000000.0)-self.c).exp()))* 1000000.0) as u128 ;
    }

    fn integral_curve(&self, x: u128) -> f64{
        //integral of y=a/1+exp(-(bx+c)) -> y= a/b * ln(exp(bx+c)+1)
        (self.a/self.b)*(((self.b*(x as f64/100000000.0)+self.c).exp())+1.0).ln() 
    }

    //The price (coins) to pay an amount of tokens
    pub fn price_to_mint(&self, num_tokens:u128) -> u128{
        let total_supply = self.total_supply;
        let new_supply = total_supply + num_tokens;
        let integral_result = self.integral_curve(new_supply);
        assert!(self.reserve_balance as f64 / 1000000.0 <= integral_result,"price_to_mint, integral_result cannot be lower than reserve_balance");
        ((self.integral_curve(new_supply) - self.reserve_balance as f64 / 1000000.0) * 1000000.0 ) as u128
    }

    //The price (coins) to to receive in exchange for an amount of tokens
    pub fn reward_for_burn(&mut self, num_tokens: u128) -> u128 {
        let total_supply = self.total_supply;
        assert!(num_tokens <= total_supply,"num tokens cannot be higher than supply");
        let new_supply = total_supply - num_tokens;
        let rewards = self.integral_curve(new_supply);
        assert!(rewards <= (self.reserve_balance as f64 / 1000000.0),"Amount of tokens to reward cannot be higher than the reserve pool balance");
        (((self.reserve_balance as f64 / 1000000.0) - rewards) * 1000000.0 ) as u128
    }

    /****** BACKUP FUNCTIONS ******/

    //Function to buy tokens from the bonding curve
    pub fn buy_lts (&mut self, num_tokens:u128) -> Promise{
        let contract_account = "light-token.testnet".to_string().try_into().unwrap();

        // Function to add the buyer in the storage of the LTS token
        let promise=ext_ft::ext(contract_account)
            .with_attached_deposit(1)
            .with_static_gas(Gas(3 * TGAS))
            .storage_deposit(env::signer_account_id().to_string());

            return promise.then( // Create a promise to callback withdraw_callback
                Self::ext(env::current_account_id())
                .with_static_gas(Gas(3 * TGAS))
                .add_storage_callback(num_tokens)
                )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn add_storage_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,num_tokens:u128 ) -> Promise{
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("There was an error contacting the token contract (Storage deposite function)");
        }

        let price_for_tokens = self.price_to_mint(num_tokens);
        let contract_account = "light-token.testnet".to_string().try_into().unwrap();

        // Function to mint LTS
        let promise=ext_ft::ext(contract_account)
            .with_static_gas(Gas(3 * TGAS))
            .mint_token(env::signer_account_id(), num_tokens);

        return promise.then( // Create a promise to callback withdraw_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(3 * TGAS))
            .mint_token_callback(num_tokens, price_for_tokens)
            )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn mint_token_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,num_tokens:u128, price_for_tokens:u128 ) {
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("There was an error contacting the token contract (Mint Token function)");
        }

        self.total_supply += num_tokens ;
        self.reserve_balance += price_for_tokens ;
        self.token_price = ((self.a/(1.0+(-self.b*(self.total_supply as f64 / 100000000.0)-self.c).exp()))* 1000000.0) as u128 ;
    }

    //Function to sell tokens to the bonding curve
    pub fn sell_lts (&mut self,num_tokens:u128) -> Promise {
        let reward_to_return = self.reward_for_burn(num_tokens);
        assert!(self.reserve_balance > reward_to_return,"The bonding curve dosn't have enough rewards");
        let contract_account = "light-token.testnet".to_string().try_into().unwrap();
        // Function to get LTS balance
        let promise=ext_ft::ext(contract_account)
            .with_static_gas(Gas(2 * TGAS))
            .ft_balance_of(env::signer_account_id().to_string());
        return promise.then( // Create a promise to callback withdraw_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(3 * TGAS))
            .ft_balance_callback(num_tokens,reward_to_return)
            )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn ft_balance_callback(&mut self, #[callback_result] call_result: Result<String, PromiseError>,num_tokens:u128, reward_to_return:u128 ) -> Promise {
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("There was an error contacting the token contract (Get Balance function)");
        }
        let balance: String = call_result.unwrap();
        assert!(balance >= num_tokens.clone().to_string(),"You don't have enough LTS balance");
        let contract_account = "light-token.testnet".to_string().try_into().unwrap();
        // Function to burn LTS
        let promise=ext_ft::ext(contract_account)
            .with_static_gas(Gas(3 * TGAS))
            .burn_token(env::signer_account_id(),num_tokens);
        return promise.then( // Create a promise to callback withdraw_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(3 * TGAS))
            .burn_token_callback(num_tokens,reward_to_return)
            )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn burn_token_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,num_tokens:u128,reward_to_return:u128) -> Promise {
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("There was an error contacting the token contract (Burn Token function)");
        }
        let contract_account:AccountId = "nusdt.ft-fin.testnet".to_string().try_into().unwrap();
        let promise=ext_stable_coin::ext(contract_account)
            .with_static_gas(Gas(3 * TGAS))
            .ft_transfer(env::signer_account_id().to_string(),reward_to_return.to_string());
        return promise.then( // Create a promise to callback withdraw_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(3 * TGAS))
            .ft_transfer_callback(num_tokens,reward_to_return)
            )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn ft_transfer_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,num_tokens:u128, reward_to_return:u128 ) {
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("There was an error contacting the USDT contract (ft_transfer function)");
        }

        self.total_supply -= num_tokens;
        self.reserve_balance -= reward_to_return;
        self.token_price = ((self.a/(1.0+(-self.b*(self.total_supply as f64 / 100000000.0)-self.c).exp()))* 1000000.0) as u128 ;
    }

    //------------CONTRACT MANAGEMENT FUNCTIONS------------------ 
    pub fn change_owner_id(&mut self, new_owner:AccountId) {
        assert!(env::signer_account_id() == self.owner_id,"You have the permission to change the owner_id");
        self.owner_id = new_owner;
    }

    pub fn change_a(&mut self, new_a:f64){
        assert!(env::signer_account_id() == self.owner_id,"You have the permission to change this coefficient");
        self.a = new_a;
    }

    pub fn change_b(&mut self, new_b:f64){
        assert!(env::signer_account_id() == self.owner_id,"You have the permission to change this coefficient");
        self.b = new_b;
    }

    pub fn change_c(&mut self, new_c:f64){
        assert!(env::signer_account_id() == self.owner_id,"You have the permission to change this coefficient");
        self.c = new_c;
    }

    //---------------GETTERS---------------------
    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn get_token_price(&self) -> f64 {
        self.token_price as f64 / 1000000.0
    }

    pub fn get_reserve_balance(&self) -> u128 {
        self.reserve_balance
    }

    pub fn get_total_supply(&self) -> u128 {
        self.total_supply
    }

    pub fn get_price_floor(&self) -> u128 {
        self.price_floor
    }
   
    pub fn get_coefficients(&self) -> (f64, f64, f64) {
        (self.a, self.b, self.c)
    }
}