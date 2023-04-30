use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract,AccountId, Promise, PromiseError, log};
use near_sdk::{env, near_bindgen, Gas};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::{PromiseOrValue};
use near_sdk::json_types::U128;
use std::f64;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;

pub const TGAS: u64 = 1_000_000_000_000;
pub const MAX_DECIMALS:f64 = 1000000000000000000.0 ;

#[ext_contract(ext_ft)]
pub trait Lighttoken {
    fn mint_token(&mut self, account_id: AccountId, amount: u128);
    fn burn_token(&mut self, account_id: AccountId, amount: u128);
    fn ft_balance_of (&mut self, account_id:String) -> String;
}

#[ext_contract(ext_stable_coin)]
pub trait Stablecoin {
    fn ft_balance_of (&mut self, account_id:String) -> String;
    fn storage_deposit (&mut self, account_id: String,registration_only:bool);
    fn ft_transfer(&mut self,receiver_id:String,amount:String);
    fn ft_metadata(&mut self) -> FungibleTokenMetadata;
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BondingCurve {
    owner_id: AccountId,
    token_price: u128,
    reserve_balance: LookupMap<String,u128>,
    total_supply: u128,
    price_floor: u128,
    a: f64,
    b: f64,
    c: f64,
    coin_ref:UnorderedMap<String, Vec<String>>,
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
        price_floor: u128,
        a: f64,
        b:f64,
        c:f64,
    ) -> Self {
        let mut dict = LookupMap::new(b"n");
        dict.insert(&"USDT.e".to_string(), &0);
        dict.insert(&"USDC".to_string(), &0);
        dict.insert(&"USN".to_string(), &0);
        let mut coin_ref = UnorderedMap::new(b"m");
        let mut vec1=Vec::<String>::new();
        vec1.push("usdt.fakes.testnet".to_string());
        vec1.push(1000000.to_string());
        coin_ref.insert(&"USDT.e".to_string(), &vec1);
        let mut vec2=Vec::<String>::new();
        vec2.push("usdc.fakes.testnet".to_string());
        vec2.push(1000000.to_string());
        coin_ref.insert(&"USDC".to_string(), &vec2);
        let mut vec3=Vec::<String>::new();
        vec3.push("usdn.testnet".to_string());
        vec3.push((1000000000000000000 as u128).to_string());
        coin_ref.insert(&"USN".to_string(), &vec3);
        Self {
            owner_id: "newtreasury.testnet".to_string().try_into().unwrap(),
            token_price,
            reserve_balance:dict,
            total_supply: 0,
            price_floor,
            a,
            b,
            c,
            coin_ref: coin_ref,
        }
    }

    fn integral_curve(&self, x: u128) -> f64{
        //integral of y=a/1+exp(-(bx+c)) -> y= a/b * ln(exp(bx+c)+1)
        (self.a/self.b)*(((self.b*(x as f64/100000000.0)+self.c).exp())+1.0).ln() 
    }

    //The price (coins) to pay an amount of tokens
    pub fn price_to_mint(&self, num_tokens:u128, coin_name:String) -> f64{
        let total_supply = self.total_supply;
        let new_supply = total_supply + num_tokens;
        let integral_result = self.integral_curve(new_supply);
        assert!(self.total_reserve_balance()  <= integral_result,"price_to_mint, integral_result cannot be lower than reserve_balance");
        let integ_curve = self.integral_curve(new_supply);
        let tot_res = self.total_reserve_balance();
        let decimal = self.get_coin_decimals(coin_name);
        let res = (integ_curve - tot_res) * decimal;
        res
    }

    //The price (coins) to to receive in exchange for an amount of tokens
    pub fn reward_for_burn(&mut self, num_tokens: u128, coin_name:String) -> u128 {
        let total_supply = self.total_supply;
        assert!(num_tokens <= total_supply,"num tokens cannot be higher than supply");
        let new_supply = total_supply - num_tokens;
        let rewards = self.integral_curve(new_supply);
        assert!(rewards <= (self.total_reserve_balance()),"Amount of tokens to reward cannot be higher than the reserve pool balance");
        (((self.total_reserve_balance()) - rewards) * self.get_coin_decimals(coin_name) ) as u128
    }

    /****** BACKUP FUNCTIONS ******/

    #[private] // Public - but only callable by env::current_account_id()
    pub fn mint_token_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>,num_tokens:u128, price_for_tokens:u128,coin_name:String,token_in:AccountId) {
        // Check if the promise succeeded
        if call_result.is_err() {
            ext_stable_coin::ext(token_in)
                .with_attached_deposit(1)
                .with_static_gas(Gas(2 * TGAS))
                .ft_transfer(env::signer_account_id().to_string(),price_for_tokens.to_string());
        }
        else{
            self.total_supply += num_tokens ;
            let new_res_balance =  self.reserve_balance.get(&coin_name).unwrap() + price_for_tokens ;
            self.reserve_balance.insert(&coin_name, &new_res_balance);
            self.token_price = ((self.a/(1.0+(-self.b*(self.total_supply as f64 / 100000000.0)-self.c).exp()))* 1000000.0) as u128 ;
        }
    }

    //Function to sell tokens to the bonding curve
    pub fn sell_lts (&mut self,num_tokens:u128,coin_name:String) -> Promise {
        let reward_to_return = self.reward_for_burn(num_tokens,coin_name.clone());
        assert!(self.reserve_balance.get(&coin_name.clone()).unwrap() > reward_to_return,"The bonding curve dosn't have enough rewards");
        let contract_account = "light-token.testnet".to_string().try_into().unwrap();
        // Function to get LTS balance
        let promise=ext_ft::ext(contract_account)
            .with_static_gas(Gas(2 * TGAS))
            .ft_balance_of(env::signer_account_id().to_string());
        return promise.then( // Create a promise to callback withdraw_callback
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(28 * TGAS))
            .ft_balance_callback(num_tokens,reward_to_return,coin_name)
            )
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn ft_balance_callback(&mut self, #[callback_result] call_result: Result<String, PromiseError>,num_tokens:u128, reward_to_return:u128,coin_name:String ) -> Promise {
        // Check if the promise succeeded
        if call_result.is_err() {
        panic!("There was an error contacting the LTS token contract (Get Balance function)");
        }
        let balance: String = call_result.unwrap();
        log!(balance);
        assert!(balance >= num_tokens.clone().to_string(),"You don't have enough LTS balance");
        let lts_account = "light-token.testnet".to_string().try_into().unwrap();
        let coin_account:AccountId = self.get_coin_contract(coin_name.clone()).try_into().unwrap();
        // Function to burn LTS
        let promise=ext_ft::ext(lts_account)
            .with_static_gas(Gas(2 * TGAS))
            .burn_token(env::signer_account_id(),num_tokens);
        return promise.then(ext_stable_coin::ext(coin_account.clone())
        .with_attached_deposit(2350000000000000000000)
        .with_static_gas(Gas(3 * TGAS))
        .storage_deposit(env::signer_account_id().to_string(),true))
        .then(ext_stable_coin::ext(coin_account.clone())
        .with_attached_deposit(1)
        .with_static_gas(Gas(2 * TGAS))
        .ft_transfer(env::signer_account_id().to_string(),reward_to_return.to_string()))
        .then(Self::ext(env::current_account_id())
        .with_static_gas(Gas(3 * TGAS))
        .ft_transfer_callback(num_tokens,reward_to_return,coin_name.clone()))
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn ft_transfer_callback(&mut self,#[callback_result] call_result: Result<(), PromiseError>,num_tokens:u128, reward_to_return:u128, coin_name:String ) -> Promise{
        if call_result.is_err() {
        let lts_account = "light-token.testnet".to_string().try_into().unwrap();
        let promise=ext_ft::ext(lts_account)
            .with_attached_deposit(2350000000000000000000)
            .with_static_gas(Gas(3 * TGAS))
            .mint_token(env::signer_account_id(), num_tokens);
        promise.then(Self::ext(env::current_account_id())
        .with_static_gas(Gas(3 * TGAS))
        .panic_sell(num_tokens))
        }else {
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(3 * TGAS))
            .finish_sell(num_tokens,reward_to_return,coin_name.clone())
        }
    }


    #[private] // Public - but only callable by env::current_account_id()
    pub fn finish_sell(&mut self,num_tokens:u128, reward_to_return:u128, coin_name:String ) {
        self.total_supply -= num_tokens;
        let new_res_balance = self.reserve_balance.get(&coin_name).unwrap() - reward_to_return;
        self.reserve_balance.insert(&coin_name, &new_res_balance);
        self.token_price = ((self.a/(1.0+(-self.b*(self.total_supply as f64 / 100000000.0)-self.c).exp()))* 1000000.0) as u128 ;
    }

    #[private] // Public - but only callable by env::current_account_id()
    pub fn panic_sell(&self,#[callback_result] call_result: Result<(), PromiseError>,num_tokens:u128){
        if call_result.is_err() {
            let lts_account = "light-token.testnet".to_string().try_into().unwrap();
            ext_ft::ext(lts_account)
                .with_attached_deposit(2350000000000000000000)
                .with_static_gas(Gas(3 * TGAS))
                .mint_token(env::signer_account_id(), num_tokens);
        }
        panic!("Transfer failed and all transactions reverted due to insufficiency of the reserve balance. Try again later"); 
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

    pub fn get_res_balance(&self, coin_name:String) -> f64{
        self.reserve_balance.get(&coin_name).unwrap() as f64 / self.get_coin_decimals(coin_name)
    }

    //Get the total reserve balance of the bonding curve
    pub fn total_reserve_balance(&self) -> f64{
        let coins = self.get_coins();
        let mut total: f64 = 0.0;
        for i in coins {
            total += self.reserve_balance.get(&i).unwrap() as f64 / self.get_coin_decimals(i);
        }
        total
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

    //--------------COINREF_DICTIONARY-----------------

    pub fn add_new_coin(&mut self, coin_name:String, coin_contract:String, decimals:u128){
        assert!(env::signer_account_id() == self.owner_id,"You don't have the permission to add a new coin.");
        let mut vec=Vec::<String>::new();
        vec.push(coin_contract);
        vec.push(decimals.to_string());
        self.coin_ref.insert(&coin_name, &vec);
    }

    pub fn get_coin_contract(&self, coin_name:String) -> String{
        return self.coin_ref.get(&coin_name).unwrap()[0].clone()
    }

    pub fn get_all_coins_contracts(&self) -> Vec<String>{
        let mut vec = Vec::new();
        let coins = self.get_coins();
        for i in coins {
            vec.push(self.get_coin_contract(i));
        }
        vec
    }

    pub fn get_coin_decimals(&self, coin_name:String) -> f64{
        return self.coin_ref.get(&coin_name).unwrap()[1].clone().parse::<f64>().unwrap()
    }

    pub fn get_coins(&self) -> Vec<String>{
        return self.coin_ref.keys_as_vector().to_vec()
    }

    pub fn get_coin_name_from_contract(&self, contract_name:String) -> String {
        let coins = self.get_coins();
        let mut res = String::new();
        for i in coins {
            if self.get_coin_contract(i.clone()) == contract_name {
                res = i;
                break;
            }
        }
        res
    }

    pub fn delete_coin(&mut self, coin_name:String){
        assert!(env::signer_account_id() == self.owner_id,"You don't have the permission to delete a coin.");
        self.coin_ref.remove(&coin_name);
    }

    fn percentage(&self,amount:u128,coin_name:String) -> u128{
        let decimals = self.get_coin_decimals(coin_name);
        (((amount as f64) / decimals) * 100000.0) as u128   
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for BondingCurve {
    /// Callback on receiving tokens by this contract.
    /// `msg` format is either "" for deposit or `TokenReceiverMessage`.
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_in = env::predecessor_account_id();
        let coins = self.get_all_coins_contracts();
        let mut existance = false;
        for i in coins {
            if token_in.to_string() == i {
                existance = true;
            }
        }
        let coin_name = self.get_coin_name_from_contract(token_in.to_string());
        assert!(existance,"This token in not supported");
        let message = msg.get(0..7).unwrap().to_string();
        let num_tokens=msg.get(8..).unwrap().parse::<u128>().unwrap();
        assert!(num_tokens > 0,"Invalid number of LTS");
        let price = self.price_to_mint(num_tokens, coin_name.clone());
        assert_eq!(self.percentage(price as u128, coin_name.clone()),self.percentage(amount.0, coin_name.clone()),"Amount transferred doesn't cover the price for the tokens");
        if message == "Buy lts".to_string() {
            // Function to mint LTS
            let contract_account = "light-token.testnet".to_string().try_into().unwrap();

            ext_ft::ext(contract_account)
                .with_attached_deposit(2350000000000000000000)
                .with_static_gas(Gas(3 * TGAS))
                .mint_token(sender_id.clone(), num_tokens)
                .then(Self::ext(env::current_account_id())
                .with_static_gas(Gas(28 * TGAS))
                .mint_token_callback(num_tokens,amount.0,coin_name,token_in));

            PromiseOrValue::Value(U128(0))
        } else {
            panic!("Error calling transfer function");
        }
    }

}