#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol, Vec, vec, log};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Club(Symbol),    // Stores the Club metadata
    Balance(Symbol), // Stores the current total saved
}

#[derive(Clone)]
#[contracttype]
pub struct Club {
    pub creator: Address,
    pub goal: i128,
    pub token: Address,
    pub description: Symbol,
    pub completed: bool,
}

#[contract]
pub struct MicroSavingContract;

#[contractimpl]
impl MicroSavingContract {

    /// Create a new savings club. 
    /// Permissionless: Anyone can call this to start a pool.
    pub fn create_club(
        env: Env, 
        club_id: Symbol, 
        creator: Address, 
        goal: i128, 
        token_addr: Address,
        desc: Symbol
    ) {
        let key = DataKey::Club(club_id.clone());
        
        if env.storage().instance().has(&key) {
            panic!("Club ID already exists");
        }

        let new_club = Club {
            creator,
            goal,
            token: token_addr,
            description: desc,
            completed: false,
        };

        env.storage().instance().set(&key, &new_club);
        env.storage().instance().set(&DataKey::Balance(club_id.clone()), &0i128);
        
        // Extend TTL (Time To Live) so the data doesn't expire quickly
        // This is crucial for permissionless dapps on Stellar
        env.storage().instance().extend_ttl(1000, 5000);
        
        log!(&env, "Club Created", club_id);
    }

    pub fn deposit(env: Env, user: Address, amount: i128, club_id: Symbol) {
        user.require_auth();

        let club_key = DataKey::Club(club_id.clone());
        let club: Club = env.storage().instance().get(&club_key).expect("Club not found");
        
        if club.completed {
            panic!("Club goal already reached");
        }

        let balance_key = DataKey::Balance(club_id.clone());
        let current_balance: i128 = env.storage().instance().get(&balance_key).unwrap_or(0);

        // Standard Soroban Token Client
        let client = token::Client::new(&env, &club.token);
        client.transfer(&user, &env.current_contract_address(), &amount);

        env.storage().instance().set(&balance_key, &(current_balance + amount));
        env.storage().instance().extend_ttl(1000, 5000);
    }

    pub fn withdraw(env: Env, club_id: Symbol) {
        let club_key = DataKey::Club(club_id.clone());
        let mut club: Club = env.storage().instance().get(&club_key).expect("Club not found");
        
        let balance_key = DataKey::Balance(club_id.clone());
        let current_balance: i128 = env.storage().instance().get(&balance_key).unwrap_or(0);

        if current_balance < club.goal {
            panic!("Goal not yet reached");
        }

        if club.completed {
            panic!("Funds already withdrawn");
        }

        club.completed = true;
        env.storage().instance().set(&club_key, &club);

        let client = token::Client::new(&env, &club.token);
        client.transfer(&env.current_contract_address(), &club.creator, &current_balance);
    }

    /// Fixed return type: Soroban prefers Vec or custom Structs over Rust tuples for public functions
    pub fn get_stats(env: Env, club_id: Symbol) -> Vec<i128> {
        let club_key = DataKey::Club(club_id.clone());
        let club: Club = env.storage().instance().get(&club_key).expect("Club not found");
        let balance: i128 = env.storage().instance().get(&DataKey::Balance(club_id)).unwrap_or(0);
        
        vec![&env, balance, club.goal]
    }
}