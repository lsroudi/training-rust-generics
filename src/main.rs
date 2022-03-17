use num::{CheckedAdd, CheckedSub, Zero};
use std::cmp::Eq;
use std::hash::Hash;

mod step1;
mod step2;
mod step3;
mod step4;
mod step5;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_step1 () {
    let mut balances = step1::BalanceModule::new();

    balances.set_balance(0,100);
    balances.set_balance(1,1000);

    assert!(balances.balance(0)==100);
    assert!(balances.balance(1)==1000);
    assert!(balances.balance(3)==0);

    assert!(balances.transfer(0, 1, 10).is_ok());

    assert!(balances.balance(0)==90);
    assert!(balances.balance(1)==1010);
}

#[test]
fn test_step2 () {
    let mut balances = step2::BalanceModule::new();

    balances.set_balance(0,100);
    balances.set_balance(1,1000);

    assert!(balances.balance(0)==100);
    assert!(balances.balance(1)==1000);
    assert!(balances.balance(3)==0);

    assert!(balances.transfer(0, 1, 10).is_ok());

    assert!(balances.balance(0)==90);
    assert!(balances.balance(1)==1010);
}


//#[test]
// // Account ID in step 2 and step 3 are defined differently!
// fn wont_work() {
// 	let user_1 = 1;
// 	let user_2 = 2;
// 	let mut balances = step2::BalanceModule::new();
// 	let mut voting = step3::VotingModule::new();

// 	balances.set_balance(user_1, 100);
// 	balances.set_balance(user_2, 200);

// 	voting.vote(user_1, 0, true);
// 	voting.vote(user_2, 0, false);
// }

#[test]
fn test_step_3() {
    type AccountId = u16;
    type VoteIndex = u64;
	let mut voting = step3::VotingModule::<AccountId,VoteIndex>::new();
	voting.vote(1, 0, true);
	voting.vote(2, 0, false);

	assert!(voting.get_vote(1, 0) == true);
	assert!(voting.get_vote(2, 0) == false);

	assert!(voting.get_vote(1, 1) == false);
	assert!(voting.get_vote(2, 1) == false);
}

#[test]
fn test_step_4() {
	type AccountId = u32;
	type Balance = u32;
	type VoteIndex = u8;

	let user_1: AccountId = 1;
	let user_2: AccountId = 2;

	let mut balances = step4::BalancesModule::<AccountId, Balance>::new();
	let mut voting = step4::VotingModule::<AccountId, VoteIndex>::new();

	balances.set_balance(user_1, 100);
	balances.set_balance(user_2, 200);

	voting.vote(user_1, 0, true);
	voting.vote(user_2, 0, false);
}


pub trait Trait {
    type AccountId : Eq + Hash;
	type Balance: Zero+CheckedAdd+CheckedSub+ Copy;
	type VoteIndex: Eq + Hash;
}
#[test]
fn test_step_5() {
    struct Runtime;
    impl Trait for Runtime {
        type AccountId = u32;
        type Balance = u32;
        type VoteIndex = u8;      
    }


	let user_1: <Runtime as Trait>::AccountId = 1;
	let user_2: <Runtime as Trait>::AccountId = 2;

	let mut balances = step5::BalancesModule::<Runtime>::new();
	let mut voting = step5::VotingModule::<Runtime>::new();

	balances.set_balance(user_1, 100);
	balances.set_balance(user_2, 200);

	voting.vote(user_1, 0, true);
	voting.vote(user_2, 0, false);
}