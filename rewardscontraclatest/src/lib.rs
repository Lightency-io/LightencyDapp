use near_sdk::{ext_contract};
use serde::{Serialize, Deserialize};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Gas};
use near_sdk::collections::{Vector, UnorderedMap};

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize,Deserialize)]
pub struct Data {
    amount:u128,
    time:u64,
    reward:f64,
    next_reward_time:u64,
    unstaked_amount:u128,
    unstake_timestamp:u64
}
#[ext_contract(ext_ft)]
pub trait Rewardpool {
    #[payable]
    fn pay(&mut self, amount: u128, to: String);
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rewardercontract {
    redeemers:Vector<String>,
    staker_data:UnorderedMap<String,Data>,
}

impl Default for Rewardercontract {
    fn default() -> Self {
        panic!("Contract is not initialized yet")
    }
}

fn assert_self() {
    assert_eq!(
        env::current_account_id(),
        env::predecessor_account_id(),
        "Can only be called by owner"
    );
}

// Implement the Rewardercontract structure
#[near_bindgen]
impl Rewardercontract {

    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            redeemers: Vector::new(b"a"),
            staker_data: UnorderedMap::new(b"m"),
        }
    }

    // delete all stakers
    pub fn delete_all_stakers(&mut self) {
        assert_self();
        self.staker_data.clear();
    }

    pub fn redeem(&mut self,account:String){
        self.redeemers.push(&account);
    }

    pub fn add_staker(&mut self, account:String, amount:u128) {
        if self.staker_data.get(&account).is_none() {
            let data = Data {
                amount : amount,
                time: env::block_timestamp(),
                reward:0 as f64,
                next_reward_time:env::block_timestamp() + 120000000,
                unstaked_amount:0,
                unstake_timestamp:0
            };
            self.staker_data.insert(&account, &data);
        }else {
            let mut data = self.staker_data.get(&account).unwrap(); 
            data.amount+=amount;
            data.time = env::block_timestamp(); 
            self.staker_data.insert(&account, &data);
        }
    }

    pub fn check_staker(&self, account:String) -> bool {
        let mut existance = false;
        let stakers = self.staker_data.keys_as_vector();
        for i in stakers.to_vec() {
            if account == i {
                existance = true;
                break;
            }
        }
        existance
    }

    pub fn get_totalstaked(&self) -> f64 {
        let mut sum:f64= 0.0;
        for i in self.staker_data.values_as_vector().to_vec() {
                sum = sum + i.amount as f64+ i.reward ;
        }
        sum
    }

    pub fn get_data(&self, account:String) -> Data {
        self.staker_data.get(&account).unwrap()
    } 

    pub fn unstake(&mut self, account:String, amount:u128){
        if self.check_staker(account.clone()){
            if amount < self.get_data(account.clone()).amount {
                let mut data=self.get_data(account.clone());
                data.amount-=amount;
                data.unstaked_amount+=amount;
                data.unstake_timestamp=env::block_timestamp();
                self.staker_data.insert(&account.clone(), &data);
            }else if amount == self.get_data(account.clone()).amount {
                let mut data=self.get_data(account.clone());
                data.amount-=amount;
                data.unstaked_amount+=amount;
                data.unstake_timestamp=env::block_timestamp();
                self.staker_data.insert(&account.clone(), &data);
            }else{
                panic!("You don't have enough staked amount !!!");
            }
        }else {
            panic!("You are not one of the stakers");
        }
    }

    pub fn pay(&self ,amount:u128 , to:String){
        let account = "lightencyrewardpool.testnet".to_string().try_into().unwrap();
        ext_ft::ext(account)
            .with_static_gas(Gas(5 * 1000000000000))
            .pay(amount,to);
        // let promise = ext_rewardpool::pay(
        //     "lightencyrewarder.testnet".to_string().try_into().unwrap(),
        //     0,
        //     Gas(50000000000000)
        // );
    }

    pub fn withdraw(&mut self){
        for i in 0..self.redeemers.len(){
            match self.redeemers.get(i){
                Some(r) => if r==env::signer_account_id().to_string() {
                    //let data2= self.query_data_staker_callback(self.get_data_staker(env::signer_account_id().to_string()));
                    //let rewards = self.calculaterewards(data2,data2.time);
                    self.pay(1,env::signer_account_id().to_string());
                    self.redeemers.swap_remove(i);
                },
                None => panic!("you are not included in redeemers list"),
            }

        }

    }

    pub fn get_total_amount_per_wallet(&self, account:String) -> f64{
        self.get_data(account.clone()).amount as f64+ self.get_data(account.clone()).reward
    }

    pub fn calculaterewards(&self,account:String)-> f64{
        //Reward to stakers= Total staked (t) X APY(t) 
        //APY(t)=Staking pool supply/total staked(t) X Yield parameter.
        let staked_per_wallet = self.get_total_amount_per_wallet(account);
        let reward_pool = 100 as f64;
        let total_reward = (reward_pool / 1095 as f64) as f64;
        let apy=(total_reward / self.get_totalstaked() as f64) as f64;
        let reward = (apy * staked_per_wallet as f64) as f64;
        return reward;
    } 

    pub fn update_reward(&mut self,account:String){
        let mut new_data= self.get_data(account.clone());
        if env::block_timestamp() > new_data.next_reward_time {
            let add_reward= self.calculaterewards(account.clone());
            new_data.reward = new_data.reward + add_reward;
            new_data.next_reward_time = new_data.next_reward_time + 120000000;
            self.staker_data.insert(&account, &new_data);
        }else {
            panic!("You have not earned reward yet");
        }
    }
}