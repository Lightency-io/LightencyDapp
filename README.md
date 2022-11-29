About Lightency: 

Lightency is a Web3 green energy startup built on the NEAR protocol. 

Lightency utilizes the pillars of blockchain technology and expertise in renewable energies to work around several axes pertaining to energy management and monetization. 

Lightency adds a sustainable and environmentally-conscious spice to Defi by creating a one-stop liquidity pool to finance green energy projects and allow for sustainable staking and vesting mechanisms.

Lightency’s Energy Fund accelerates the creation and implementation of industry-shifting green energy projects worldwide. Crypto and Energy VCs will get above-market returns and a seat at the Lightency DAO council by investing in the fund. 

The fund is governed by a DAO composed of the project’s team and the contributing investors. DAO Council Members can initiate proposals to transfer funds from one DAO to another or to fund green energy projects, and decisions go through the voting procedure. 

Furthermore, Lightency’s Powerchain redefines energy management through blockchain-optimized microgrids and opens a portal for self-sustained communities to produce and consume energy locally and efficiently. 

NB: 

The voting mechanism is: Democratic. 
Only council and community members are part of the Lightency DAO.
A community member must be an LTS staker
A council member can be Lightency team member or strategic investor, or partner. 
A watchdog is an algorithm that is created to execute initiated proposals when the voting period expires.

The contracts below are the deliverables of Milestone 2 of the NEAR Foundation Grant.

LTS smart contract (lts_token.near)  :

* Mint function: A function to mint Light tokens 

* Burn function: A function to burn Light tokens 

* Ft_balance_of: A function to receive Light token balance by account id 

* Transfer function: a function to send Light tokens to an identified wallet  

* Stake function: a cross-call function that adds a Light token staker to the list of stakers. 


Treasury DAO smart contract ( treasurydao.near )  : 

*Create_proposal function: A function to create proposals in the treasury DAO. Only council members can use this function. There are four types of proposals 

Fund energy pool 
Add a council member to the DAO  
Borrow stable coins from burrow  
Borrow stable coins from compound


 Every proposal has an expiration date. Following the expiration of the proposal, an off-chain cron will execute the task immediately and autonomously.

* Add_vote function: this function is designated for council and community DAO members

Members submit an irrevocable Yes or No vote.

* Add_council function: This function adds a council member to the DAO. After the voting period is ended, a watchdog is required to proceed with the function.

* Add_community function: This function adds a user to the DAO as a community member. 

* Get_specific_proposal function: This function extracts a specific proposal by its ID. 

* Get_proposals function: This function extracts the list of all proposals.

* Get_communities: This function extracts the list of all community members 

*Get_councils: This function extracts the list of all council members. 

*Check_council function: This function verifies if the user is a council member 

*Check_member function: This function verifies if the user is a community member

*fund function: This function is delivered by the Lightency watchdog. 
This function is executed after the agreement of the proposal of type "fund energy pool." This function will send Light tokens from the Treasury Pool to the Energy Pool.

Staking wallet contract ( staking_contract.near ) : 

*Unstake function: This function deducts the demanded unstake amount.

In case of no staked funds remaining, the interested member will be removed from the Stakers list. Hence, the member loses their status as a community member. 

* Withdraw function: This function withdraws the unstaked amount. The Light tokens will be transferred immediately to the staker.

Staking pool contract  ( lightencypool.near ): 

* Transfer_lts: This function is executed when the staker asks for their stake to be removed. The assets will be transferred to the staking wallet.

Rewarder smart contract ( rewarder_contract.near ) : 

* Add_staker: This function adds a staker to the list of stakers.

* Check_staker: This function verifies a staker’s authenticity.

*Get_total_staked: this function returns the total amount staked in the pool. 

* Unstake: This function adds a staker to the list of unstakers. 

* Withdraw_reward: This function allows the user to withdraw their staking reward. 

*Get_total_amount_per_wallet: This function daily returns the amount of reward of every registered staker.

* Caculate_reward: This function calculates the reward of every staker and returns the amount of daily yield. 

* Update_reward: This function is used by the Lightency watchdog. The cron will distribute the rewards to every staker daily. 

Vesting smart contract ( lightency_vesting.near ): 

* Get_all_vestors: This function returns the list of all vestors. 

* Get_vestor: This function returns a vestor through its ID. 

* Get_total_locked_amount: This function returns all the locked light tokens in the smart contract

*Get_total_unlocked_amount: this function returns all the unlocked like tokens in the smart contract

*Add_lockup: This function adds a vesting schedule to the vestor.

*Refresh: This function verifies if the vester can unlock their token. If they do, the Light tokens will be immediately transferred to them. 

Create an energy certificate in a form of an NFT  ( nft-lightency.near)

*nft_mint()  :  A function to mint an energy certificate with an NFT.
*nft_approve() : A function to grant escrow access to rec_lightency.near contract



Fractionalizing an energy certificate (NFT) into shares (FTs) (lightency_rec.near)

*fill_share_holders(): A function to assign to each accountId a number of shares from that REC (NFT)
*securitize(): A function to fractionalize the REC (NFT), be transferred and a shares contract will be created under the ID of [nft-lightency-near- <tokenId>.lightency_rec.near] according to what we did with fill_share_holders() : each accontId and how many shares he owns.

The Shares Contract (nft-lightency-near- <tokenId>.lightency_rec.near)

*ft_balance_of() : A function to view how many that accountId own shares
