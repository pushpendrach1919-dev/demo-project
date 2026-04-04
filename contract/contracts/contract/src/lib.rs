#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Vec, Address
};

#[contract]
pub struct VotingContract;

#[contracttype]
#[derive(Clone)]
pub struct Poll {
    pub question: Symbol,
    pub options: Vec<Symbol>,
    pub deadline: u64,
}

#[contracttype]
pub enum DataKey {
    Poll(u64),
    Vote(u64, Address),        // (poll_id, voter)
    VoteCount(u64, Symbol),   // (poll_id, option)
    PollCount,
}

#[contractimpl]
impl VotingContract {

    // 🗳️ Create a new poll (permissionless)
    pub fn create_poll(
        env: Env,
        question: Symbol,
        options: Vec<Symbol>,
        deadline: u64,
    ) -> u64 {

        // Get current poll count
        let mut poll_count: u64 = env
            .storage()
            .instance()
            .get(&DataKey::PollCount)
            .unwrap_or(0);

        poll_count += 1;

        let poll = Poll {
            question,
            options,
            deadline,
        };

        // Store poll
        env.storage()
            .instance()
            .set(&DataKey::Poll(poll_count), &poll);

        // Update poll count
        env.storage()
            .instance()
            .set(&DataKey::PollCount, &poll_count);

        poll_count
    }

    // 🗳️ Vote on a poll (permissionless)
    pub fn vote(
        env: Env,
        poll_id: u64,
        voter: Address,
        option: Symbol,
    ) {

        // Require authentication
        voter.require_auth();

        // Fetch poll
        let poll: Poll = env
            .storage()
            .instance()
            .get(&DataKey::Poll(poll_id))
            .expect("Poll does not exist");

        // Check deadline
        let current_time = env.ledger().timestamp();
        if current_time > poll.deadline {
            panic!("Voting has ended");
        }

        // Prevent double voting
        let vote_key = DataKey::Vote(poll_id, voter.clone());
        if env.storage().instance().has(&vote_key) {
            panic!("Already voted");
        }

        // Validate option exists
        let mut valid = false;
        for opt in poll.options.iter() {
            if opt == option {
                valid = true;
                break;
            }
        }

        if !valid {
            panic!("Invalid option");
        }

        // Store vote
        env.storage()
            .instance()
            .set(&vote_key, &option);

        // Increment vote count
        let count_key = DataKey::VoteCount(poll_id, option.clone());

        let mut count: u32 = env
            .storage()
            .instance()
            .get(&count_key)
            .unwrap_or(0);

        count += 1;

        env.storage()
            .instance()
            .set(&count_key, &count);
    }

    // 📊 Get vote count for an option
    pub fn get_vote_count(
        env: Env,
        poll_id: u64,
        option: Symbol,
    ) -> u32 {

        env.storage()
            .instance()
            .get(&DataKey::VoteCount(poll_id, option))
            .unwrap_or(0)
    }

    // 📄 Get poll details
    pub fn get_poll(env: Env, poll_id: u64) -> Poll {
        env.storage()
            .instance()
            .get(&DataKey::Poll(poll_id))
            .expect("Poll not found")
    }

    // 🔢 Get total number of polls
    pub fn get_poll_count(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::PollCount)
            .unwrap_or(0)
    }
}