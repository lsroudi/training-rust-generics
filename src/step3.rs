#![allow(dead_code)]

use std::{collections::HashMap, hash::Hash};

type AccountId = u64;
type VoteIndex = u64;

pub struct VotingModule<AccountId,VoteIndex> {
	votes: HashMap<(AccountId, VoteIndex), bool>,
}

impl <AccountId:Eq+Hash,VoteIndex:Eq+Hash>VotingModule<AccountId,VoteIndex> {
	pub fn new() -> Self {
		Self {
			votes: HashMap::new()
		}
	}

	pub fn vote(&mut self, who: AccountId, index: VoteIndex, vote: bool) {
		self.votes.insert((who, index), vote);
	}

	pub fn get_vote(&self, who: AccountId, index: VoteIndex) -> bool {
		*self.votes.get(&(who, index)).unwrap_or(&false)
	}
}