/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};





// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

#[ext_contract(ext_stakingpool)]
pub trait stakingpool {
    fn get_amount(&mut self, amount:u128);
    fn get_timestaked(&mut self,totaltimestaked:u64);
    fn get_totalstaked(&mut self);
}



// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Rewardercontract {
    
}

// Define the default, which automatically initializes the contract
impl Default for Rewardercontract{
    fn default() -> Self{
        Self{message: DEFAULT_MESSAGE.to_string()}
    }
}

// Implement the Rewardercontract structure
#[near_bindgen]
impl Rewardercontract {
    let account = "stakepool.testnet";


    pub fn get_amount(&self,staker:String){
        let promise = ext_stakingpool::get_amount(
            staker,
            AccountId::from_str(account).unwrap(),
            0,
            Gas(50000000000000)
        )
        return promise;
    }

    pub fn get_timestaked(&self,staker:String)-> Promise{
        let promise =ext_stakingpool::get_timestaked(
            staker,
            AccountId::from_str(account).unwrap(),
            0,
            Gas(50000000000000)

        )
        return promise;
    }

    pub fn get_totalstaked(&self)-> Promise{
        let promise = ext_stakingpool::get_totalstaked(
            AccountId::from_str(account).unwrap(),
            0,
            Gas(50000000000000)
        )
        return promise;
    }

    pub fn calculaterewards(&self,stakers:Vector<String>)-> f64{
        let totalstaked=get_totalstaked();
        for staker in stakers{
            let amount = get_amount(staker);
            let timer = get_timestaked(staker);
            //Reward to stakers= Total staked (t) X APY(t) 
            //APY(t)=Staking pool supply/total staked(t) X Yield parameter.
            const apy=0.5;
        }
    }


    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            contract.get_greeting(),
            "Hello".to_string()
        );
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            contract.get_greeting(),
            "howdy".to_string()
        );
    }
}
