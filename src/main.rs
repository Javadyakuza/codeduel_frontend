#![no_std]

use ink_lang as ink;

#[ink::contract]
mod code_duel {

    use ink_storage::{collections::HashMap as StorageHashMap, lazy::Lazy};

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum QuestionStatus {
        Open,
        ClosedSolved,
        ClosedNotSolved,
    }

    #[ink(storage)]
    pub struct CodeDuel {
        code_duel_fee: Lazy<u32>,
        questions: StorageHashMap<u32, Question>,
        dared_evils: StorageHashMap<AccountId, Vec<u32>>,
        rivals: StorageHashMap<AccountId, Vec<u32>>,
        rivals_balances: StorageHashMap<AccountId, u32>,
        questions_pool: StorageHashMap<u32, u32>,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        question_id: u32,
        #[ink(topic)]
        amount: u32,
    }

    #[ink(event)]
    pub struct OpenQuestion {
        #[ink(topic)]
        question_id: u32,
    }

    #[ink(event)]
    pub struct CloseQuestion {
        #[ink(topic)]
        question_id: u32,
        #[ink(topic)]
        status: QuestionStatus,
    }

    #[ink(event)]
    pub struct RewardDaredevil {
        #[ink(topic)]
        account_id: AccountId,
        #[ink(topic)]
        amount: u32,
    }

    #[ink(event)]
    pub struct RefundRival {
        #[ink(topic)]
        account_id: AccountId,
        #[ink(topic)]
        amount: u32,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Question {
        id: u32,
        name: Hash,
        status: QuestionStatus,
        reward: u32,
        rival_id: AccountId,
        daredevil_id: AccountId,
        current_prize_pool: u32,
        entrance_fee: u32,
        deadline: u64,
    }

    impl CodeDuel {
        #[ink(constructor)]
        pub fn new(code_duel_fee: u32) -> Self {
            Self {
                code_duel_fee: Lazy::new(code_duel_fee),
                questions: StorageHashMap::new(),
                dared_evils: StorageHashMap::new(),
                rivals: StorageHashMap::new(),
                rivals_balances: StorageHashMap::new(),
                questions_pool: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn deposit(&mut self, question_id: u32, amount: u32) {
            // Check amount and emit Deposit event
        }

        #[ink(message)]
        pub fn open_question(&mut self, question: Question) {
            // Store question, update mappings and emit OpenQuestion event
        }

        #[ink(message)]
        pub fn close_question(&mut self, status: QuestionStatus, question_id: u32) {
            // Close question, reward/refund accounts, emit events
        }

        #[ink(message)]
        fn reward_daredevil(&mut self, account_id: AccountId, amount: u32) {
            // Transfer SOL and emit event
        }

        #[ink(message)]
        fn refund_rival(&mut self, account_id: AccountId, amount: u32) {
            // Transfer SOL and emit event
        }

        // Getter methods
    }
}
