#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub port: i32,
    #[prost(bytes="vec", tag="3")]
    pub node_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingMessage {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(message, optional, tag="2")]
    pub to: ::core::option::Option<Endpoint>,
    #[prost(int32, tag="3")]
    pub version: i32,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PongMessage {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(int32, tag="2")]
    pub echo: i32,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindNeighbours {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(bytes="vec", tag="2")]
    pub target_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Neighbours {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(message, repeated, tag="2")]
    pub neighbours: ::prost::alloc::vec::Vec<Endpoint>,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupMessage {
    #[prost(bool, tag="1")]
    pub flag: bool,
    #[prost(int32, tag="2")]
    pub priority: i32,
}
/// AccountId, (name, address) use name, (null, address) use address, (name, null) use name,
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountId {
    #[prost(bytes="vec", tag="1")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
/// vote message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    /// the super rep address
    #[prost(bytes="vec", tag="1")]
    pub vote_address: ::prost::alloc::vec::Vec<u8>,
    /// the vote num to this super rep.
    #[prost(int64, tag="2")]
    pub vote_count: i64,
}
/// Proposal
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(int64, tag="1")]
    pub proposal_id: i64,
    #[prost(bytes="vec", tag="2")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="int64, int64", tag="3")]
    pub parameters: ::std::collections::HashMap<i64, i64>,
    #[prost(int64, tag="4")]
    pub expiration_time: i64,
    #[prost(int64, tag="5")]
    pub create_time: i64,
    #[prost(bytes="vec", repeated, tag="6")]
    pub approvals: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration="proposal::State", tag="7")]
    pub state: i32,
}
/// Nested message and enum types in `Proposal`.
pub mod proposal {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Pending = 0,
        Disapproved = 1,
        Approved = 2,
        Canceled = 3,
    }
}
/// Exchange
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Exchange {
    #[prost(int64, tag="1")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="2")]
    pub creator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub create_time: i64,
    #[prost(bytes="vec", tag="6")]
    pub first_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="7")]
    pub first_token_balance: i64,
    #[prost(bytes="vec", tag="8")]
    pub second_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="9")]
    pub second_token_balance: i64,
}
/// market
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrder {
    #[prost(bytes="vec", tag="1")]
    pub order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub create_time: i64,
    #[prost(bytes="vec", tag="4")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub sell_token_quantity: i64,
    #[prost(bytes="vec", tag="6")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
    /// min to receive
    #[prost(int64, tag="7")]
    pub buy_token_quantity: i64,
    #[prost(int64, tag="9")]
    pub sell_token_quantity_remain: i64,
    /// When state != ACTIVE and sell_token_quantity_return !=0,
    ///it means that some sell tokens are returned to the account due to insufficient remaining amount
    #[prost(int64, tag="10")]
    pub sell_token_quantity_return: i64,
    #[prost(enumeration="market_order::State", tag="11")]
    pub state: i32,
    #[prost(bytes="vec", tag="12")]
    pub prev: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub next: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `MarketOrder`.
pub mod market_order {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Active = 0,
        Inactive = 1,
        Canceled = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderList {
    #[prost(message, repeated, tag="1")]
    pub orders: ::prost::alloc::vec::Vec<MarketOrder>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderPairList {
    #[prost(message, repeated, tag="1")]
    pub order_pair: ::prost::alloc::vec::Vec<MarketOrderPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderPair {
    #[prost(bytes="vec", tag="1")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketAccountOrder {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// order_id list
    #[prost(bytes="vec", repeated, tag="2")]
    pub orders: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// active count
    #[prost(int64, tag="3")]
    pub count: i64,
    #[prost(int64, tag="4")]
    pub total_count: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPrice {
    #[prost(int64, tag="1")]
    pub sell_token_quantity: i64,
    #[prost(int64, tag="2")]
    pub buy_token_quantity: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPriceList {
    #[prost(bytes="vec", tag="1")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub prices: ::prost::alloc::vec::Vec<MarketPrice>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderIdList {
    #[prost(bytes="vec", tag="1")]
    pub head: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub tail: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainParameters {
    #[prost(message, repeated, tag="1")]
    pub chain_parameter: ::prost::alloc::vec::Vec<chain_parameters::ChainParameter>,
}
/// Nested message and enum types in `ChainParameters`.
pub mod chain_parameters {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ChainParameter {
        #[prost(string, tag="1")]
        pub key: ::prost::alloc::string::String,
        #[prost(int64, tag="2")]
        pub value: i64,
    }
}
/// Account 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// account nick name
    #[prost(bytes="vec", tag="1")]
    pub account_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="AccountType", tag="2")]
    pub r#type: i32,
    /// the create address
    #[prost(bytes="vec", tag="3")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// the trx balance
    #[prost(int64, tag="4")]
    pub balance: i64,
    /// the votes
    #[prost(message, repeated, tag="5")]
    pub votes: ::prost::alloc::vec::Vec<Vote>,
    /// the other asset owned by this account
    #[prost(map="string, int64", tag="6")]
    pub asset: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// the other asset owned by this account，key is assetId
    #[prost(map="string, int64", tag="56")]
    pub asset_v2: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    /// the frozen balance for bandwidth
    #[prost(message, repeated, tag="7")]
    pub frozen: ::prost::alloc::vec::Vec<account::Frozen>,
    /// bandwidth, get from frozen
    #[prost(int64, tag="8")]
    pub net_usage: i64,
    ///Frozen balance provided by other accounts to this account
    #[prost(int64, tag="41")]
    pub acquired_delegated_frozen_balance_for_bandwidth: i64,
    ///Freeze and provide balances to other accounts
    #[prost(int64, tag="42")]
    pub delegated_frozen_balance_for_bandwidth: i64,
    #[prost(int64, tag="46")]
    pub old_tron_power: i64,
    #[prost(message, optional, tag="47")]
    pub tron_power: ::core::option::Option<account::Frozen>,
    #[prost(bool, tag="60")]
    pub asset_optimized: bool,
    /// this account create time
    #[prost(int64, tag="9")]
    pub create_time: i64,
    /// this last operation time, including transfer, voting and so on. //FIXME fix grammar
    #[prost(int64, tag="10")]
    pub latest_opration_time: i64,
    /// witness block producing allowance
    #[prost(int64, tag="11")]
    pub allowance: i64,
    /// last withdraw time
    #[prost(int64, tag="12")]
    pub latest_withdraw_time: i64,
    /// not used so far
    #[prost(bytes="vec", tag="13")]
    pub code: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="14")]
    pub is_witness: bool,
    #[prost(bool, tag="15")]
    pub is_committee: bool,
    /// frozen asset(for asset issuer)
    #[prost(message, repeated, tag="16")]
    pub frozen_supply: ::prost::alloc::vec::Vec<account::Frozen>,
    /// asset_issued_name
    #[prost(bytes="vec", tag="17")]
    pub asset_issued_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="57")]
    pub asset_issued_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="string, int64", tag="18")]
    pub latest_asset_operation_time: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="58")]
    pub latest_asset_operation_time_v2: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="19")]
    pub free_net_usage: i64,
    #[prost(map="string, int64", tag="20")]
    pub free_asset_net_usage: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="59")]
    pub free_asset_net_usage_v2: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="21")]
    pub latest_consume_time: i64,
    #[prost(int64, tag="22")]
    pub latest_consume_free_time: i64,
    /// the identity of this account, case insensitive
    #[prost(bytes="vec", tag="23")]
    pub account_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="26")]
    pub account_resource: ::core::option::Option<account::AccountResource>,
    #[prost(bytes="vec", tag="30")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="31")]
    pub owner_permission: ::core::option::Option<Permission>,
    #[prost(message, optional, tag="32")]
    pub witness_permission: ::core::option::Option<Permission>,
    #[prost(message, repeated, tag="33")]
    pub active_permission: ::prost::alloc::vec::Vec<Permission>,
}
/// Nested message and enum types in `Account`.
pub mod account {
    /// frozen balance 
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Frozen {
        /// the frozen trx balance
        #[prost(int64, tag="1")]
        pub frozen_balance: i64,
        /// the expire time
        #[prost(int64, tag="2")]
        pub expire_time: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccountResource {
        /// energy resource, get from frozen
        #[prost(int64, tag="1")]
        pub energy_usage: i64,
        /// the frozen balance for energy
        #[prost(message, optional, tag="2")]
        pub frozen_balance_for_energy: ::core::option::Option<Frozen>,
        #[prost(int64, tag="3")]
        pub latest_consume_time_for_energy: i64,
        ///Frozen balance provided by other accounts to this account
        #[prost(int64, tag="4")]
        pub acquired_delegated_frozen_balance_for_energy: i64,
        ///Frozen balances provided to other accounts
        #[prost(int64, tag="5")]
        pub delegated_frozen_balance_for_energy: i64,
        /// storage resource, get from market
        #[prost(int64, tag="6")]
        pub storage_limit: i64,
        #[prost(int64, tag="7")]
        pub storage_usage: i64,
        #[prost(int64, tag="8")]
        pub latest_exchange_storage_time: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub weight: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResource {
    #[prost(bytes="vec", tag="1")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub frozen_balance_for_bandwidth: i64,
    #[prost(int64, tag="4")]
    pub frozen_balance_for_energy: i64,
    #[prost(int64, tag="5")]
    pub expire_time_for_bandwidth: i64,
    #[prost(int64, tag="6")]
    pub expire_time_for_energy: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<AccountId>,
    #[prost(bytes="vec", tag="2")]
    pub permission_name: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    #[prost(enumeration="permission::PermissionType", tag="1")]
    pub r#type: i32,
    ///Owner id=0, Witness id=1, Active id start by 2
    #[prost(int32, tag="2")]
    pub id: i32,
    #[prost(string, tag="3")]
    pub permission_name: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub threshold: i64,
    #[prost(int32, tag="5")]
    pub parent_id: i32,
    ///1 bit 1 contract
    #[prost(bytes="vec", tag="6")]
    pub operations: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="7")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
}
/// Nested message and enum types in `Permission`.
pub mod permission {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PermissionType {
        Owner = 0,
        Witness = 1,
        Active = 2,
    }
}
/// Witness
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Witness {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub vote_count: i64,
    #[prost(bytes="vec", tag="3")]
    pub pub_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub url: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub total_produced: i64,
    #[prost(int64, tag="6")]
    pub total_missed: i64,
    #[prost(int64, tag="7")]
    pub latest_block_num: i64,
    #[prost(int64, tag="8")]
    pub latest_slot_num: i64,
    #[prost(bool, tag="9")]
    pub is_jobs: bool,
}
/// Vote Change
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Votes {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub old_votes: ::prost::alloc::vec::Vec<Vote>,
    #[prost(message, repeated, tag="3")]
    pub new_votes: ::prost::alloc::vec::Vec<Vote>,
}
// Transcation

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutput {
    #[prost(int64, tag="1")]
    pub value: i64,
    #[prost(bytes="vec", tag="2")]
    pub pub_key_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInput {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<tx_input::Raw>,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `TXInput`.
pub mod tx_input {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(bytes="vec", tag="1")]
        pub tx_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub vout: i64,
        #[prost(bytes="vec", tag="3")]
        pub pub_key: ::prost::alloc::vec::Vec<u8>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxOutputs {
    #[prost(message, repeated, tag="1")]
    pub outputs: ::prost::alloc::vec::Vec<TxOutput>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReceipt {
    #[prost(int64, tag="1")]
    pub energy_usage: i64,
    #[prost(int64, tag="2")]
    pub energy_fee: i64,
    #[prost(int64, tag="3")]
    pub origin_energy_usage: i64,
    #[prost(int64, tag="4")]
    pub energy_usage_total: i64,
    #[prost(int64, tag="5")]
    pub net_usage: i64,
    #[prost(int64, tag="6")]
    pub net_fee: i64,
    #[prost(enumeration="transaction::result::ContractResult", tag="7")]
    pub result: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketOrderDetail {
    #[prost(bytes="vec", tag="1")]
    pub maker_order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub taker_order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub fill_sell_quantity: i64,
    #[prost(int64, tag="4")]
    pub fill_buy_quantity: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<transaction::Raw>,
    /// only support size = 1,  repeated list here for muti-sig extension
    #[prost(bytes="vec", repeated, tag="2")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="5")]
    pub ret: ::prost::alloc::vec::Vec<transaction::Result>,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Contract {
        #[prost(enumeration="contract::ContractType", tag="1")]
        pub r#type: i32,
        #[prost(message, optional, tag="2")]
        pub parameter: ::core::option::Option<::prost_types::Any>,
        #[prost(bytes="vec", tag="3")]
        pub provider: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="4")]
        pub contract_name: ::prost::alloc::vec::Vec<u8>,
        #[prost(int32, tag="5")]
        pub permission_id: i32,
    }
    /// Nested message and enum types in `Contract`.
    pub mod contract {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ContractType {
            AccountCreateContract = 0,
            TransferContract = 1,
            TransferAssetContract = 2,
            VoteAssetContract = 3,
            VoteWitnessContract = 4,
            WitnessCreateContract = 5,
            AssetIssueContract = 6,
            WitnessUpdateContract = 8,
            ParticipateAssetIssueContract = 9,
            AccountUpdateContract = 10,
            FreezeBalanceContract = 11,
            UnfreezeBalanceContract = 12,
            WithdrawBalanceContract = 13,
            UnfreezeAssetContract = 14,
            UpdateAssetContract = 15,
            ProposalCreateContract = 16,
            ProposalApproveContract = 17,
            ProposalDeleteContract = 18,
            SetAccountIdContract = 19,
            CustomContract = 20,
            CreateSmartContract = 30,
            TriggerSmartContract = 31,
            GetContract = 32,
            UpdateSettingContract = 33,
            ExchangeCreateContract = 41,
            ExchangeInjectContract = 42,
            ExchangeWithdrawContract = 43,
            ExchangeTransactionContract = 44,
            UpdateEnergyLimitContract = 45,
            AccountPermissionUpdateContract = 46,
            ClearAbiContract = 48,
            UpdateBrokerageContract = 49,
            ShieldedTransferContract = 51,
            MarketSellAssetContract = 52,
            MarketCancelOrderContract = 53,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(int64, tag="1")]
        pub fee: i64,
        #[prost(enumeration="result::Code", tag="2")]
        pub ret: i32,
        #[prost(enumeration="result::ContractResult", tag="3")]
        pub contract_ret: i32,
        #[prost(string, tag="14")]
        pub asset_issue_id: ::prost::alloc::string::String,
        #[prost(int64, tag="15")]
        pub withdraw_amount: i64,
        #[prost(int64, tag="16")]
        pub unfreeze_amount: i64,
        #[prost(int64, tag="18")]
        pub exchange_received_amount: i64,
        #[prost(int64, tag="19")]
        pub exchange_inject_another_amount: i64,
        #[prost(int64, tag="20")]
        pub exchange_withdraw_another_amount: i64,
        #[prost(int64, tag="21")]
        pub exchange_id: i64,
        #[prost(int64, tag="22")]
        pub shielded_transaction_fee: i64,
        #[prost(bytes="vec", tag="25")]
        pub order_id: ::prost::alloc::vec::Vec<u8>,
        #[prost(message, repeated, tag="26")]
        pub order_details: ::prost::alloc::vec::Vec<super::MarketOrderDetail>,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Code {
            Sucess = 0,
            Failed = 1,
        }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ContractResult {
            Default = 0,
            Success = 1,
            Revert = 2,
            BadJumpDestination = 3,
            OutOfMemory = 4,
            PrecompiledContract = 5,
            StackTooSmall = 6,
            StackTooLarge = 7,
            IllegalOperation = 8,
            StackOverflow = 9,
            OutOfEnergy = 10,
            OutOfTime = 11,
            JvmStackOverFlow = 12,
            Unknown = 13,
            TransferFailed = 14,
            InvalidCode = 15,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(bytes="vec", tag="1")]
        pub ref_block_bytes: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="3")]
        pub ref_block_num: i64,
        #[prost(bytes="vec", tag="4")]
        pub ref_block_hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="8")]
        pub expiration: i64,
        #[prost(message, repeated, tag="9")]
        pub auths: ::prost::alloc::vec::Vec<super::Authority>,
        /// data not used
        #[prost(bytes="vec", tag="10")]
        pub data: ::prost::alloc::vec::Vec<u8>,
        ///only support size = 1,  repeated list here for extension
        #[prost(message, repeated, tag="11")]
        pub contract: ::prost::alloc::vec::Vec<Contract>,
        /// scripts not used
        #[prost(bytes="vec", tag="12")]
        pub scripts: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="14")]
        pub timestamp: i64,
        #[prost(int64, tag="18")]
        pub fee_limit: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub fee: i64,
    #[prost(int64, tag="3")]
    pub block_number: i64,
    #[prost(int64, tag="4")]
    pub block_time_stamp: i64,
    #[prost(bytes="vec", repeated, tag="5")]
    pub contract_result: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="6")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub receipt: ::core::option::Option<ResourceReceipt>,
    #[prost(message, repeated, tag="8")]
    pub log: ::prost::alloc::vec::Vec<transaction_info::Log>,
    #[prost(enumeration="transaction_info::Code", tag="9")]
    pub result: i32,
    #[prost(bytes="vec", tag="10")]
    pub res_message: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="14")]
    pub asset_issue_id: ::prost::alloc::string::String,
    #[prost(int64, tag="15")]
    pub withdraw_amount: i64,
    #[prost(int64, tag="16")]
    pub unfreeze_amount: i64,
    #[prost(message, repeated, tag="17")]
    pub internal_transactions: ::prost::alloc::vec::Vec<InternalTransaction>,
    #[prost(int64, tag="18")]
    pub exchange_received_amount: i64,
    #[prost(int64, tag="19")]
    pub exchange_inject_another_amount: i64,
    #[prost(int64, tag="20")]
    pub exchange_withdraw_another_amount: i64,
    #[prost(int64, tag="21")]
    pub exchange_id: i64,
    #[prost(int64, tag="22")]
    pub shielded_transaction_fee: i64,
    #[prost(bytes="vec", tag="25")]
    pub order_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="26")]
    pub order_details: ::prost::alloc::vec::Vec<MarketOrderDetail>,
    #[prost(int64, tag="27")]
    pub packing_fee: i64,
}
/// Nested message and enum types in `TransactionInfo`.
pub mod transaction_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Log {
        #[prost(bytes="vec", tag="1")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", repeated, tag="2")]
        pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes="vec", tag="3")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        Sucess = 0,
        Failed = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionRet {
    #[prost(int64, tag="1")]
    pub block_number: i64,
    #[prost(int64, tag="2")]
    pub block_time_stamp: i64,
    #[prost(message, repeated, tag="3")]
    pub transactioninfo: ::prost::alloc::vec::Vec<TransactionInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSign {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(bytes="vec", tag="2")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<block_header::Raw>,
    #[prost(bytes="vec", tag="2")]
    pub witness_signature: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `BlockHeader`.
pub mod block_header {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(int64, tag="1")]
        pub timestamp: i64,
        #[prost(bytes="vec", tag="2")]
        pub tx_trie_root: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="3")]
        pub parent_hash: ::prost::alloc::vec::Vec<u8>,
        ///bytes nonce = 5;
        ///bytes difficulty = 6;
        #[prost(int64, tag="7")]
        pub number: i64,
        #[prost(int64, tag="8")]
        pub witness_id: i64,
        #[prost(bytes="vec", tag="9")]
        pub witness_address: ::prost::alloc::vec::Vec<u8>,
        #[prost(int32, tag="10")]
        pub version: i32,
        #[prost(bytes="vec", tag="11")]
        pub account_state_root: ::prost::alloc::vec::Vec<u8>,
    }
}
/// block
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
    #[prost(message, optional, tag="2")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainInventory {
    #[prost(message, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<chain_inventory::BlockId>,
    #[prost(int64, tag="2")]
    pub remain_num: i64,
}
/// Nested message and enum types in `ChainInventory`.
pub mod chain_inventory {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockId {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
}
/// Inventory
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInventory {
    #[prost(message, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<block_inventory::BlockId>,
    #[prost(enumeration="block_inventory::Type", tag="2")]
    pub r#type: i32,
}
/// Nested message and enum types in `BlockInventory`.
pub mod block_inventory {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockId {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Sync = 0,
        Advtise = 1,
        Fetch = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    #[prost(enumeration="inventory::InventoryType", tag="1")]
    pub r#type: i32,
    #[prost(bytes="vec", repeated, tag="2")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `Inventory`.
pub mod inventory {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InventoryType {
        Trx = 0,
        Block = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Items {
    #[prost(enumeration="items::ItemType", tag="1")]
    pub r#type: i32,
    #[prost(message, repeated, tag="2")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
    #[prost(message, repeated, tag="3")]
    pub block_headers: ::prost::alloc::vec::Vec<BlockHeader>,
    #[prost(message, repeated, tag="4")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
/// Nested message and enum types in `Items`.
pub mod items {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ItemType {
        Err = 0,
        Trx = 1,
        Block = 2,
        Blockheader = 3,
    }
}
/// DynamicProperties
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicProperties {
    #[prost(int64, tag="1")]
    pub last_solidity_block_num: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectMessage {
    #[prost(enumeration="ReasonCode", tag="1")]
    pub reason: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloMessage {
    #[prost(message, optional, tag="1")]
    pub from: ::core::option::Option<Endpoint>,
    #[prost(int32, tag="2")]
    pub version: i32,
    #[prost(int64, tag="3")]
    pub timestamp: i64,
    #[prost(message, optional, tag="4")]
    pub genesis_block_id: ::core::option::Option<hello_message::BlockId>,
    #[prost(message, optional, tag="5")]
    pub solid_block_id: ::core::option::Option<hello_message::BlockId>,
    #[prost(message, optional, tag="6")]
    pub head_block_id: ::core::option::Option<hello_message::BlockId>,
    #[prost(bytes="vec", tag="7")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="9")]
    pub node_type: i32,
    #[prost(int64, tag="10")]
    pub lowest_block_num: i64,
}
/// Nested message and enum types in `HelloMessage`.
pub mod hello_message {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockId {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalTransaction {
    /// internalTransaction identity, the root InternalTransaction hash
    /// should equals to root transaction id.
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// the one send trx (TBD: or token) via function
    #[prost(bytes="vec", tag="2")]
    pub caller_address: ::prost::alloc::vec::Vec<u8>,
    /// the one recieve trx (TBD: or token) via function
    #[prost(bytes="vec", tag="3")]
    pub transfer_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub call_value_info: ::prost::alloc::vec::Vec<internal_transaction::CallValueInfo>,
    #[prost(bytes="vec", tag="5")]
    pub note: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="6")]
    pub rejected: bool,
    #[prost(string, tag="7")]
    pub extra: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InternalTransaction`.
pub mod internal_transaction {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallValueInfo {
        /// trx (TBD: or token) value
        #[prost(int64, tag="1")]
        pub call_value: i64,
        /// TBD: tokenName, trx should be empty
        #[prost(string, tag="2")]
        pub token_id: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResourceAccountIndex {
    #[prost(bytes="vec", tag="1")]
    pub account: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub from_accounts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub to_accounts: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    #[prost(int64, tag="1")]
    pub begin_sync_num: i64,
    #[prost(string, tag="2")]
    pub block: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub solidity_block: ::prost::alloc::string::String,
    ///connect information
    #[prost(int32, tag="4")]
    pub current_connect_count: i32,
    #[prost(int32, tag="5")]
    pub active_connect_count: i32,
    #[prost(int32, tag="6")]
    pub passive_connect_count: i32,
    #[prost(int64, tag="7")]
    pub total_flow: i64,
    #[prost(message, repeated, tag="8")]
    pub peer_info_list: ::prost::alloc::vec::Vec<node_info::PeerInfo>,
    #[prost(message, optional, tag="9")]
    pub config_node_info: ::core::option::Option<node_info::ConfigNodeInfo>,
    #[prost(message, optional, tag="10")]
    pub machine_info: ::core::option::Option<node_info::MachineInfo>,
    #[prost(map="string, string", tag="11")]
    pub cheat_witness_info_map: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `NodeInfo`.
pub mod node_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PeerInfo {
        #[prost(string, tag="1")]
        pub last_sync_block: ::prost::alloc::string::String,
        #[prost(int64, tag="2")]
        pub remain_num: i64,
        #[prost(int64, tag="3")]
        pub last_block_update_time: i64,
        #[prost(bool, tag="4")]
        pub sync_flag: bool,
        #[prost(int64, tag="5")]
        pub head_block_time_we_both_have: i64,
        #[prost(bool, tag="6")]
        pub need_sync_from_peer: bool,
        #[prost(bool, tag="7")]
        pub need_sync_from_us: bool,
        #[prost(string, tag="8")]
        pub host: ::prost::alloc::string::String,
        #[prost(int32, tag="9")]
        pub port: i32,
        #[prost(string, tag="10")]
        pub node_id: ::prost::alloc::string::String,
        #[prost(int64, tag="11")]
        pub connect_time: i64,
        #[prost(double, tag="12")]
        pub avg_latency: f64,
        #[prost(int32, tag="13")]
        pub sync_to_fetch_size: i32,
        #[prost(int64, tag="14")]
        pub sync_to_fetch_size_peek_num: i64,
        #[prost(int32, tag="15")]
        pub sync_block_requested_size: i32,
        #[prost(int64, tag="16")]
        pub un_fetch_syn_num: i64,
        #[prost(int32, tag="17")]
        pub block_in_porc_size: i32,
        #[prost(string, tag="18")]
        pub head_block_we_both_have: ::prost::alloc::string::String,
        #[prost(bool, tag="19")]
        pub is_active: bool,
        #[prost(int32, tag="20")]
        pub score: i32,
        #[prost(int32, tag="21")]
        pub node_count: i32,
        #[prost(int64, tag="22")]
        pub in_flow: i64,
        #[prost(int32, tag="23")]
        pub disconnect_times: i32,
        #[prost(string, tag="24")]
        pub local_disconnect_reason: ::prost::alloc::string::String,
        #[prost(string, tag="25")]
        pub remote_disconnect_reason: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConfigNodeInfo {
        #[prost(string, tag="1")]
        pub code_version: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub p2p_version: ::prost::alloc::string::String,
        #[prost(int32, tag="3")]
        pub listen_port: i32,
        #[prost(bool, tag="4")]
        pub discover_enable: bool,
        #[prost(int32, tag="5")]
        pub active_node_size: i32,
        #[prost(int32, tag="6")]
        pub passive_node_size: i32,
        #[prost(int32, tag="7")]
        pub send_node_size: i32,
        #[prost(int32, tag="8")]
        pub max_connect_count: i32,
        #[prost(int32, tag="9")]
        pub same_ip_max_connect_count: i32,
        #[prost(int32, tag="10")]
        pub backup_listen_port: i32,
        #[prost(int32, tag="11")]
        pub backup_member_size: i32,
        #[prost(int32, tag="12")]
        pub backup_priority: i32,
        #[prost(int32, tag="13")]
        pub db_version: i32,
        #[prost(int32, tag="14")]
        pub min_participation_rate: i32,
        #[prost(bool, tag="15")]
        pub support_constant: bool,
        #[prost(double, tag="16")]
        pub min_time_ratio: f64,
        #[prost(double, tag="17")]
        pub max_time_ratio: f64,
        #[prost(int64, tag="18")]
        pub allow_creation_of_contracts: i64,
        #[prost(int64, tag="19")]
        pub allow_adaptive_energy: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MachineInfo {
        #[prost(int32, tag="1")]
        pub thread_count: i32,
        #[prost(int32, tag="2")]
        pub dead_lock_thread_count: i32,
        #[prost(int32, tag="3")]
        pub cpu_count: i32,
        #[prost(int64, tag="4")]
        pub total_memory: i64,
        #[prost(int64, tag="5")]
        pub free_memory: i64,
        #[prost(double, tag="6")]
        pub cpu_rate: f64,
        #[prost(string, tag="7")]
        pub java_version: ::prost::alloc::string::String,
        #[prost(string, tag="8")]
        pub os_name: ::prost::alloc::string::String,
        #[prost(int64, tag="9")]
        pub jvm_total_memory: i64,
        #[prost(int64, tag="10")]
        pub jvm_free_memory: i64,
        #[prost(double, tag="11")]
        pub process_cpu_rate: f64,
        #[prost(message, repeated, tag="12")]
        pub memory_desc_info_list: ::prost::alloc::vec::Vec<machine_info::MemoryDescInfo>,
        #[prost(message, repeated, tag="13")]
        pub dead_lock_thread_info_list: ::prost::alloc::vec::Vec<machine_info::DeadLockThreadInfo>,
    }
    /// Nested message and enum types in `MachineInfo`.
    pub mod machine_info {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MemoryDescInfo {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
            #[prost(int64, tag="2")]
            pub init_size: i64,
            #[prost(int64, tag="3")]
            pub use_size: i64,
            #[prost(int64, tag="4")]
            pub max_size: i64,
            #[prost(double, tag="5")]
            pub use_rate: f64,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeadLockThreadInfo {
            #[prost(string, tag="1")]
            pub name: ::prost::alloc::string::String,
            #[prost(string, tag="2")]
            pub lock_name: ::prost::alloc::string::String,
            #[prost(string, tag="3")]
            pub lock_owner: ::prost::alloc::string::String,
            #[prost(string, tag="4")]
            pub state: ::prost::alloc::string::String,
            #[prost(int64, tag="5")]
            pub block_time: i64,
            #[prost(int64, tag="6")]
            pub wait_time: i64,
            #[prost(string, tag="7")]
            pub stack_trace: ::prost::alloc::string::String,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsInfo {
    #[prost(int64, tag="1")]
    pub interval: i64,
    #[prost(message, optional, tag="2")]
    pub node: ::core::option::Option<metrics_info::NodeInfo>,
    #[prost(message, optional, tag="3")]
    pub blockchain: ::core::option::Option<metrics_info::BlockChainInfo>,
    #[prost(message, optional, tag="4")]
    pub net: ::core::option::Option<metrics_info::NetInfo>,
}
/// Nested message and enum types in `MetricsInfo`.
pub mod metrics_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NodeInfo {
        #[prost(string, tag="1")]
        pub ip: ::prost::alloc::string::String,
        #[prost(int32, tag="2")]
        pub node_type: i32,
        #[prost(string, tag="3")]
        pub version: ::prost::alloc::string::String,
        #[prost(int32, tag="4")]
        pub backup_status: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockChainInfo {
        #[prost(int64, tag="1")]
        pub head_block_num: i64,
        #[prost(int64, tag="2")]
        pub head_block_timestamp: i64,
        #[prost(string, tag="3")]
        pub head_block_hash: ::prost::alloc::string::String,
        #[prost(int32, tag="4")]
        pub fork_count: i32,
        #[prost(int32, tag="5")]
        pub fail_fork_count: i32,
        #[prost(message, optional, tag="6")]
        pub block_process_time: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="7")]
        pub tps: ::core::option::Option<RateInfo>,
        #[prost(int32, tag="8")]
        pub transaction_cache_size: i32,
        #[prost(message, optional, tag="9")]
        pub missed_transaction: ::core::option::Option<RateInfo>,
        #[prost(message, repeated, tag="10")]
        pub witnesses: ::prost::alloc::vec::Vec<block_chain_info::Witness>,
        #[prost(int64, tag="11")]
        pub fail_process_block_num: i64,
        #[prost(string, tag="12")]
        pub fail_process_block_reason: ::prost::alloc::string::String,
        #[prost(message, repeated, tag="13")]
        pub dup_witness: ::prost::alloc::vec::Vec<block_chain_info::DupWitness>,
    }
    /// Nested message and enum types in `BlockChainInfo`.
    pub mod block_chain_info {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Witness {
            #[prost(string, tag="1")]
            pub address: ::prost::alloc::string::String,
            #[prost(int32, tag="2")]
            pub version: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DupWitness {
            #[prost(string, tag="1")]
            pub address: ::prost::alloc::string::String,
            #[prost(int64, tag="2")]
            pub block_num: i64,
            #[prost(int32, tag="3")]
            pub count: i32,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateInfo {
        #[prost(int64, tag="1")]
        pub count: i64,
        #[prost(double, tag="2")]
        pub mean_rate: f64,
        #[prost(double, tag="3")]
        pub one_minute_rate: f64,
        #[prost(double, tag="4")]
        pub five_minute_rate: f64,
        #[prost(double, tag="5")]
        pub fifteen_minute_rate: f64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetInfo {
        #[prost(int32, tag="1")]
        pub error_proto_count: i32,
        #[prost(message, optional, tag="2")]
        pub api: ::core::option::Option<net_info::ApiInfo>,
        #[prost(int32, tag="3")]
        pub connection_count: i32,
        #[prost(int32, tag="4")]
        pub valid_connection_count: i32,
        #[prost(message, optional, tag="5")]
        pub tcp_in_traffic: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="6")]
        pub tcp_out_traffic: ::core::option::Option<RateInfo>,
        #[prost(int32, tag="7")]
        pub disconnection_count: i32,
        #[prost(message, repeated, tag="8")]
        pub disconnection_detail: ::prost::alloc::vec::Vec<net_info::DisconnectionDetailInfo>,
        #[prost(message, optional, tag="9")]
        pub udp_in_traffic: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="10")]
        pub udp_out_traffic: ::core::option::Option<RateInfo>,
        #[prost(message, optional, tag="11")]
        pub latency: ::core::option::Option<net_info::LatencyInfo>,
    }
    /// Nested message and enum types in `NetInfo`.
    pub mod net_info {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ApiInfo {
            #[prost(message, optional, tag="1")]
            pub qps: ::core::option::Option<super::RateInfo>,
            #[prost(message, optional, tag="2")]
            pub fail_qps: ::core::option::Option<super::RateInfo>,
            #[prost(message, optional, tag="3")]
            pub out_traffic: ::core::option::Option<super::RateInfo>,
            #[prost(message, repeated, tag="4")]
            pub detail: ::prost::alloc::vec::Vec<api_info::ApiDetailInfo>,
        }
        /// Nested message and enum types in `ApiInfo`.
        pub mod api_info {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ApiDetailInfo {
                #[prost(string, tag="1")]
                pub name: ::prost::alloc::string::String,
                #[prost(message, optional, tag="2")]
                pub qps: ::core::option::Option<super::super::RateInfo>,
                #[prost(message, optional, tag="3")]
                pub fail_qps: ::core::option::Option<super::super::RateInfo>,
                #[prost(message, optional, tag="4")]
                pub out_traffic: ::core::option::Option<super::super::RateInfo>,
            }
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DisconnectionDetailInfo {
            #[prost(string, tag="1")]
            pub reason: ::prost::alloc::string::String,
            #[prost(int32, tag="2")]
            pub count: i32,
        }
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LatencyInfo {
            #[prost(int32, tag="1")]
            pub top99: i32,
            #[prost(int32, tag="2")]
            pub top95: i32,
            #[prost(int32, tag="3")]
            pub top75: i32,
            #[prost(int32, tag="4")]
            pub total_count: i32,
            #[prost(int32, tag="5")]
            pub delay1_s: i32,
            #[prost(int32, tag="6")]
            pub delay2_s: i32,
            #[prost(int32, tag="7")]
            pub delay3_s: i32,
            #[prost(message, repeated, tag="8")]
            pub detail: ::prost::alloc::vec::Vec<latency_info::LatencyDetailInfo>,
        }
        /// Nested message and enum types in `LatencyInfo`.
        pub mod latency_info {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct LatencyDetailInfo {
                #[prost(string, tag="1")]
                pub witness: ::prost::alloc::string::String,
                #[prost(int32, tag="2")]
                pub top99: i32,
                #[prost(int32, tag="3")]
                pub top95: i32,
                #[prost(int32, tag="4")]
                pub top75: i32,
                #[prost(int32, tag="5")]
                pub count: i32,
                #[prost(int32, tag="6")]
                pub delay1_s: i32,
                #[prost(int32, tag="7")]
                pub delay2_s: i32,
                #[prost(int32, tag="8")]
                pub delay3_s: i32,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbftMessage {
    #[prost(message, optional, tag="1")]
    pub raw_data: ::core::option::Option<pbft_message::Raw>,
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `PBFTMessage`.
pub mod pbft_message {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        #[prost(enumeration="MsgType", tag="1")]
        pub msg_type: i32,
        #[prost(enumeration="DataType", tag="2")]
        pub data_type: i32,
        #[prost(int64, tag="3")]
        pub view_n: i64,
        #[prost(int64, tag="4")]
        pub epoch: i64,
        #[prost(bytes="vec", tag="5")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgType {
        ViewChange = 0,
        Request = 1,
        Preprepare = 2,
        Prepare = 3,
        Commit = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataType {
        Block = 0,
        Srl = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PbftCommitResult {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub signature: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Srl {
    #[prost(bytes="vec", repeated, tag="1")]
    pub sr_address: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    Normal = 0,
    AssetIssue = 1,
    Contract = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReasonCode {
    Requested = 0,
    BadProtocol = 2,
    TooManyPeers = 4,
    DuplicatePeer = 5,
    IncompatibleProtocol = 6,
    NullIdentity = 7,
    PeerQuiting = 8,
    UnexpectedIdentity = 9,
    LocalIdentity = 10,
    PingTimeout = 11,
    UserReason = 16,
    Reset = 17,
    SyncFail = 18,
    FetchFail = 19,
    BadTx = 20,
    BadBlock = 21,
    Forked = 22,
    Unlinkable = 23,
    IncompatibleVersion = 24,
    IncompatibleChain = 25,
    TimeOut = 32,
    ConnectFail = 33,
    TooManyPeersWithSameIp = 34,
    LightNodeSyncFail = 35,
    Unknown = 255,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetIssueContract {
    #[prost(string, tag="41")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub abbr: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub total_supply: i64,
    #[prost(message, repeated, tag="5")]
    pub frozen_supply: ::prost::alloc::vec::Vec<asset_issue_contract::FrozenSupply>,
    #[prost(int32, tag="6")]
    pub trx_num: i32,
    #[prost(int32, tag="7")]
    pub precision: i32,
    #[prost(int32, tag="8")]
    pub num: i32,
    #[prost(int64, tag="9")]
    pub start_time: i64,
    #[prost(int64, tag="10")]
    pub end_time: i64,
    /// useless
    #[prost(int64, tag="11")]
    pub order: i64,
    #[prost(int32, tag="16")]
    pub vote_score: i32,
    #[prost(bytes="vec", tag="20")]
    pub description: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="21")]
    pub url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="22")]
    pub free_asset_net_limit: i64,
    #[prost(int64, tag="23")]
    pub public_free_asset_net_limit: i64,
    #[prost(int64, tag="24")]
    pub public_free_asset_net_usage: i64,
    #[prost(int64, tag="25")]
    pub public_latest_free_net_time: i64,
}
/// Nested message and enum types in `AssetIssueContract`.
pub mod asset_issue_contract {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FrozenSupply {
        #[prost(int64, tag="1")]
        pub frozen_amount: i64,
        #[prost(int64, tag="2")]
        pub frozen_days: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferAssetContract {
    /// this field is token name before the proposal ALLOW_SAME_TOKEN_NAME is active, otherwise it is token id and token is should be in string format.
    #[prost(bytes="vec", tag="1")]
    pub asset_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfreezeAssetContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAssetContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub description: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub url: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub new_limit: i64,
    #[prost(int64, tag="5")]
    pub new_public_limit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParticipateAssetIssueContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    /// this field is token name before the proposal ALLOW_SAME_TOKEN_NAME is active, otherwise it is token id and token is should be in string format.
    #[prost(bytes="vec", tag="3")]
    pub asset_name: ::prost::alloc::vec::Vec<u8>,
    /// the amount of drops
    #[prost(int64, tag="4")]
    pub amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub account_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="AccountType", tag="3")]
    pub r#type: i32,
}
/// Update account name. Account name is not unique now.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountUpdateContract {
    #[prost(bytes="vec", tag="1")]
    pub account_name: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
/// Set account id if the account has no id. Account id is unique and case insensitive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAccountIdContract {
    #[prost(bytes="vec", tag="1")]
    pub account_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPermissionUpdateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    ///Empty is invalidate
    #[prost(message, optional, tag="2")]
    pub owner: ::core::option::Option<Permission>,
    ///Can be empty
    #[prost(message, optional, tag="3")]
    pub witness: ::core::option::Option<Permission>,
    ///Empty is invalidate
    #[prost(message, repeated, tag="4")]
    pub actives: ::prost::alloc::vec::Vec<Permission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub url: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessUpdateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="12")]
    pub update_url: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteWitnessContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub votes: ::prost::alloc::vec::Vec<vote_witness_contract::Vote>,
    #[prost(bool, tag="3")]
    pub support: bool,
}
/// Nested message and enum types in `VoteWitnessContract`.
pub mod vote_witness_contract {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Vote {
        #[prost(bytes="vec", tag="1")]
        pub vote_address: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub vote_count: i64,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceCode {
    Bandwidth = 0,
    Energy = 1,
    TronPower = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeBalanceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub frozen_balance: i64,
    #[prost(int64, tag="3")]
    pub frozen_duration: i64,
    #[prost(enumeration="ResourceCode", tag="10")]
    pub resource: i32,
    #[prost(bytes="vec", tag="15")]
    pub receiver_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnfreezeBalanceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="ResourceCode", tag="10")]
    pub resource: i32,
    #[prost(bytes="vec", tag="15")]
    pub receiver_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawBalanceContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionBalanceTrace {
    #[prost(bytes="vec", tag="1")]
    pub transaction_identifier: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub operation: ::prost::alloc::vec::Vec<transaction_balance_trace::Operation>,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransactionBalanceTrace`.
pub mod transaction_balance_trace {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Operation {
        #[prost(int64, tag="1")]
        pub operation_identifier: i64,
        #[prost(bytes="vec", tag="2")]
        pub address: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="3")]
        pub amount: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockBalanceTrace {
    #[prost(message, optional, tag="1")]
    pub block_identifier: ::core::option::Option<block_balance_trace::BlockIdentifier>,
    #[prost(int64, tag="2")]
    pub timestamp: i64,
    ///  BlockIdentifier parent_block_identifier = 4;
    #[prost(message, repeated, tag="3")]
    pub transaction_balance_trace: ::prost::alloc::vec::Vec<TransactionBalanceTrace>,
}
/// Nested message and enum types in `BlockBalanceTrace`.
pub mod block_balance_trace {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BlockIdentifier {
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        #[prost(int64, tag="2")]
        pub number: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountTrace {
    #[prost(int64, tag="1")]
    pub balance: i64,
    #[prost(int64, tag="99")]
    pub placeholder: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountIdentifier {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBalanceRequest {
    #[prost(message, optional, tag="1")]
    pub account_identifier: ::core::option::Option<AccountIdentifier>,
    #[prost(message, optional, tag="2")]
    pub block_identifier: ::core::option::Option<block_balance_trace::BlockIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBalanceResponse {
    #[prost(int64, tag="1")]
    pub balance: i64,
    #[prost(message, optional, tag="2")]
    pub block_identifier: ::core::option::Option<block_balance_trace::BlockIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalApproveContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub proposal_id: i64,
    /// add or remove approval
    #[prost(bool, tag="3")]
    pub is_add_approval: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="int64, int64", tag="2")]
    pub parameters: ::std::collections::HashMap<i64, i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalDeleteContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub proposal_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyStorageBytesContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// storage bytes for buy
    #[prost(int64, tag="2")]
    pub bytes: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuyStorageContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// trx quantity for buy storage (sun)
    #[prost(int64, tag="2")]
    pub quant: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellStorageContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub storage_bytes: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBrokerageContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    /// 1 mean 1%
    #[prost(int32, tag="2")]
    pub brokerage: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeCreateContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub first_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub first_token_balance: i64,
    #[prost(bytes="vec", tag="4")]
    pub second_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub second_token_balance: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeInjectContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub quant: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeWithdrawContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub quant: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeTransactionContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub exchange_id: i64,
    #[prost(bytes="vec", tag="3")]
    pub token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub quant: i64,
    #[prost(int64, tag="5")]
    pub expected: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketSellAssetContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub sell_token_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub sell_token_quantity: i64,
    #[prost(bytes="vec", tag="4")]
    pub buy_token_id: ::prost::alloc::vec::Vec<u8>,
    /// min to receive
    #[prost(int64, tag="5")]
    pub buy_token_quantity: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketCancelOrderContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub order_id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartContract {
    #[prost(bytes="vec", tag="1")]
    pub origin_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub abi: ::core::option::Option<smart_contract::Abi>,
    #[prost(bytes="vec", tag="4")]
    pub bytecode: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub call_value: i64,
    #[prost(int64, tag="6")]
    pub consume_user_resource_percent: i64,
    #[prost(string, tag="7")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag="8")]
    pub origin_energy_limit: i64,
    #[prost(bytes="vec", tag="9")]
    pub code_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub trx_hash: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `SmartContract`.
pub mod smart_contract {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Abi {
        #[prost(message, repeated, tag="1")]
        pub entrys: ::prost::alloc::vec::Vec<abi::Entry>,
    }
    /// Nested message and enum types in `ABI`.
    pub mod abi {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Entry {
            #[prost(bool, tag="1")]
            pub anonymous: bool,
            #[prost(bool, tag="2")]
            pub constant: bool,
            #[prost(string, tag="3")]
            pub name: ::prost::alloc::string::String,
            #[prost(message, repeated, tag="4")]
            pub inputs: ::prost::alloc::vec::Vec<entry::Param>,
            #[prost(message, repeated, tag="5")]
            pub outputs: ::prost::alloc::vec::Vec<entry::Param>,
            #[prost(enumeration="entry::EntryType", tag="6")]
            pub r#type: i32,
            #[prost(bool, tag="7")]
            pub payable: bool,
            #[prost(enumeration="entry::StateMutabilityType", tag="8")]
            pub state_mutability: i32,
        }
        /// Nested message and enum types in `Entry`.
        pub mod entry {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Param {
                #[prost(bool, tag="1")]
                pub indexed: bool,
                #[prost(string, tag="2")]
                pub name: ::prost::alloc::string::String,
                /// SolidityType type = 3;
                #[prost(string, tag="3")]
                pub r#type: ::prost::alloc::string::String,
            }
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum EntryType {
                UnknownEntryType = 0,
                Constructor = 1,
                Function = 2,
                Event = 3,
                Fallback = 4,
                Receive = 5,
                Error = 6,
            }
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum StateMutabilityType {
                UnknownMutabilityType = 0,
                Pure = 1,
                View = 2,
                Nonpayable = 3,
                Payable = 4,
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSmartContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub new_contract: ::core::option::Option<SmartContract>,
    #[prost(int64, tag="3")]
    pub call_token_value: i64,
    #[prost(int64, tag="4")]
    pub token_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerSmartContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub call_value: i64,
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub call_token_value: i64,
    #[prost(int64, tag="6")]
    pub token_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearAbiContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub consume_user_resource_percent: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnergyLimitContract {
    #[prost(bytes="vec", tag="1")]
    pub owner_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub origin_energy_limit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartContractDataWrapper {
    #[prost(message, optional, tag="1")]
    pub smart_contract: ::core::option::Option<SmartContract>,
    #[prost(bytes="vec", tag="2")]
    pub runtimecode: ::prost::alloc::vec::Vec<u8>,
}
// for shielded transaction

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationPath {
    #[prost(bool, repeated, tag="1")]
    pub value: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePath {
    #[prost(message, repeated, tag="1")]
    pub authentication_paths: ::prost::alloc::vec::Vec<AuthenticationPath>,
    #[prost(bool, repeated, tag="2")]
    pub index: ::prost::alloc::vec::Vec<bool>,
    #[prost(bytes="vec", tag="3")]
    pub rt: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputPoint {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputPointInfo {
    #[prost(message, repeated, tag="1")]
    pub out_points: ::prost::alloc::vec::Vec<OutputPoint>,
    #[prost(int32, tag="2")]
    pub block_num: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PedersenHash {
    #[prost(bytes="vec", tag="1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementalMerkleTree {
    #[prost(message, optional, tag="1")]
    pub left: ::core::option::Option<PedersenHash>,
    #[prost(message, optional, tag="2")]
    pub right: ::core::option::Option<PedersenHash>,
    #[prost(message, repeated, tag="3")]
    pub parents: ::prost::alloc::vec::Vec<PedersenHash>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementalMerkleVoucher {
    #[prost(message, optional, tag="1")]
    pub tree: ::core::option::Option<IncrementalMerkleTree>,
    #[prost(message, repeated, tag="2")]
    pub filled: ::prost::alloc::vec::Vec<PedersenHash>,
    #[prost(message, optional, tag="3")]
    pub cursor: ::core::option::Option<IncrementalMerkleTree>,
    #[prost(int64, tag="4")]
    pub cursor_depth: i64,
    #[prost(bytes="vec", tag="5")]
    pub rt: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="10")]
    pub output_point: ::core::option::Option<OutputPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncrementalMerkleVoucherInfo {
    #[prost(message, repeated, tag="1")]
    pub vouchers: ::prost::alloc::vec::Vec<IncrementalMerkleVoucher>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub paths: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendDescription {
    #[prost(bytes="vec", tag="1")]
    pub value_commitment: ::prost::alloc::vec::Vec<u8>,
    /// merkle root
    #[prost(bytes="vec", tag="2")]
    pub anchor: ::prost::alloc::vec::Vec<u8>,
    /// used for check double spend
    #[prost(bytes="vec", tag="3")]
    pub nullifier: ::prost::alloc::vec::Vec<u8>,
    /// used for check spend authority signature
    #[prost(bytes="vec", tag="4")]
    pub rk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub zkproof: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub spend_authority_signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveDescription {
    #[prost(bytes="vec", tag="1")]
    pub value_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub note_commitment: ::prost::alloc::vec::Vec<u8>,
    /// for Encryption
    #[prost(bytes="vec", tag="3")]
    pub epk: ::prost::alloc::vec::Vec<u8>,
    /// Encryption for incoming, decrypt it with ivk
    #[prost(bytes="vec", tag="4")]
    pub c_enc: ::prost::alloc::vec::Vec<u8>,
    /// Encryption for audit, decrypt it with ovk
    #[prost(bytes="vec", tag="5")]
    pub c_out: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub zkproof: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedTransferContract {
    /// transparent address
    #[prost(bytes="vec", tag="1")]
    pub transparent_from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub from_amount: i64,
    #[prost(message, repeated, tag="3")]
    pub spend_description: ::prost::alloc::vec::Vec<SpendDescription>,
    #[prost(message, repeated, tag="4")]
    pub receive_description: ::prost::alloc::vec::Vec<ReceiveDescription>,
    #[prost(bytes="vec", tag="5")]
    pub binding_signature: ::prost::alloc::vec::Vec<u8>,
    /// transparent address
    #[prost(bytes="vec", tag="6")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    /// the amount to transparent to_address
    #[prost(int64, tag="7")]
    pub to_amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Return {
    #[prost(bool, tag="1")]
    pub result: bool,
    #[prost(enumeration="r#return::ResponseCode", tag="2")]
    pub code: i32,
    #[prost(bytes="vec", tag="3")]
    pub message: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Return`.
pub mod r#return {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseCode {
        Success = 0,
        /// error in signature
        Sigerror = 1,
        ContractValidateError = 2,
        ContractExeError = 3,
        BandwithError = 4,
        DupTransactionError = 5,
        TaposError = 6,
        TooBigTransactionError = 7,
        TransactionExpirationError = 8,
        ServerBusy = 9,
        NoConnection = 10,
        NotEnoughEffectiveConnection = 11,
        OtherError = 20,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockReference {
    #[prost(int64, tag="1")]
    pub block_num: i64,
    #[prost(bytes="vec", tag="2")]
    pub block_hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WitnessList {
    #[prost(message, repeated, tag="1")]
    pub witnesses: ::prost::alloc::vec::Vec<Witness>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalList {
    #[prost(message, repeated, tag="1")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExchangeList {
    #[prost(message, repeated, tag="1")]
    pub exchanges: ::prost::alloc::vec::Vec<Exchange>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetIssueList {
    #[prost(message, repeated, tag="1")]
    pub asset_issue: ::prost::alloc::vec::Vec<AssetIssueContract>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockList {
    #[prost(message, repeated, tag="1")]
    pub block: ::prost::alloc::vec::Vec<Block>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionList {
    #[prost(message, repeated, tag="1")]
    pub transaction: ::prost::alloc::vec::Vec<Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionIdList {
    #[prost(string, repeated, tag="1")]
    pub tx_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResourceMessage {
    #[prost(bytes="vec", tag="1")]
    pub from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatedResourceList {
    #[prost(message, repeated, tag="1")]
    pub delegated_resource: ::prost::alloc::vec::Vec<DelegatedResource>,
}
/// Gossip node list
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeList {
    #[prost(message, repeated, tag="1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
}
/// Gossip node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(message, optional, tag="1")]
    pub address: ::core::option::Option<Address>,
}
/// Gossip node address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(bytes="vec", tag="1")]
    pub host: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="2")]
    pub port: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyMessage {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumberMessage {
    #[prost(int64, tag="1")]
    pub num: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesMessage {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeMessage {
    #[prost(int64, tag="1")]
    pub begin_in_milliseconds: i64,
    #[prost(int64, tag="2")]
    pub end_in_milliseconds: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockLimit {
    #[prost(int64, tag="1")]
    pub start_num: i64,
    #[prost(int64, tag="2")]
    pub end_num: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionLimit {
    #[prost(bytes="vec", tag="1")]
    pub transaction_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="2")]
    pub limit_num: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPaginated {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<Account>,
    #[prost(int64, tag="2")]
    pub offset: i64,
    #[prost(int64, tag="3")]
    pub limit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimePaginatedMessage {
    #[prost(message, optional, tag="1")]
    pub time_message: ::core::option::Option<TimeMessage>,
    #[prost(int64, tag="2")]
    pub offset: i64,
    #[prost(int64, tag="3")]
    pub limit: i64,
}
///deprecated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountNetMessage {
    #[prost(int64, tag="1")]
    pub free_net_used: i64,
    #[prost(int64, tag="2")]
    pub free_net_limit: i64,
    #[prost(int64, tag="3")]
    pub net_used: i64,
    #[prost(int64, tag="4")]
    pub net_limit: i64,
    #[prost(map="string, int64", tag="5")]
    pub asset_net_used: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="6")]
    pub asset_net_limit: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="7")]
    pub total_net_limit: i64,
    #[prost(int64, tag="8")]
    pub total_net_weight: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountResourceMessage {
    #[prost(int64, tag="1")]
    pub free_net_used: i64,
    #[prost(int64, tag="2")]
    pub free_net_limit: i64,
    #[prost(int64, tag="3")]
    pub net_used: i64,
    #[prost(int64, tag="4")]
    pub net_limit: i64,
    #[prost(map="string, int64", tag="5")]
    pub asset_net_used: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(map="string, int64", tag="6")]
    pub asset_net_limit: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
    #[prost(int64, tag="7")]
    pub total_net_limit: i64,
    #[prost(int64, tag="8")]
    pub total_net_weight: i64,
    #[prost(int64, tag="9")]
    pub total_tron_power_weight: i64,
    #[prost(int64, tag="10")]
    pub tron_power_used: i64,
    #[prost(int64, tag="11")]
    pub tron_power_limit: i64,
    #[prost(int64, tag="13")]
    pub energy_used: i64,
    #[prost(int64, tag="14")]
    pub energy_limit: i64,
    #[prost(int64, tag="15")]
    pub total_energy_limit: i64,
    #[prost(int64, tag="16")]
    pub total_energy_weight: i64,
    #[prost(int64, tag="21")]
    pub storage_used: i64,
    #[prost(int64, tag="22")]
    pub storage_limit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginatedMessage {
    #[prost(int64, tag="1")]
    pub offset: i64,
    #[prost(int64, tag="2")]
    pub limit: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EasyTransferMessage {
    #[prost(bytes="vec", tag="1")]
    pub pass_phrase: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EasyTransferAssetMessage {
    #[prost(bytes="vec", tag="1")]
    pub pass_phrase: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EasyTransferByPrivateMessage {
    #[prost(bytes="vec", tag="1")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EasyTransferAssetByPrivateMessage {
    #[prost(bytes="vec", tag="1")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub asset_id: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub amount: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EasyTransferResponse {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(message, optional, tag="2")]
    pub result: ::core::option::Option<Return>,
    ///transaction id =  sha256(transaction.rowdata)
    #[prost(bytes="vec", tag="3")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressPrKeyPairMessage {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub private_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionExtention {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    ///transaction id =  sha256(transaction.rowdata)
    #[prost(bytes="vec", tag="2")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="3")]
    pub constant_result: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="4")]
    pub result: ::core::option::Option<Return>,
    #[prost(int64, tag="5")]
    pub energy_used: i64,
    #[prost(message, repeated, tag="6")]
    pub logs: ::prost::alloc::vec::Vec<transaction_info::Log>,
    #[prost(message, repeated, tag="7")]
    pub internal_transactions: ::prost::alloc::vec::Vec<InternalTransaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockExtention {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionExtention>,
    #[prost(message, optional, tag="2")]
    pub block_header: ::core::option::Option<BlockHeader>,
    #[prost(bytes="vec", tag="3")]
    pub blockid: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockListExtention {
    #[prost(message, repeated, tag="1")]
    pub block: ::prost::alloc::vec::Vec<BlockExtention>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionListExtention {
    #[prost(message, repeated, tag="1")]
    pub transaction: ::prost::alloc::vec::Vec<TransactionExtention>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockIncrementalMerkleTree {
    #[prost(int64, tag="1")]
    pub number: i64,
    #[prost(message, optional, tag="2")]
    pub merkle_tree: ::core::option::Option<IncrementalMerkleTree>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSignWeight {
    #[prost(message, optional, tag="1")]
    pub permission: ::core::option::Option<Permission>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub approved_list: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, tag="3")]
    pub current_weight: i64,
    #[prost(message, optional, tag="4")]
    pub result: ::core::option::Option<transaction_sign_weight::Result>,
    #[prost(message, optional, tag="5")]
    pub transaction: ::core::option::Option<TransactionExtention>,
}
/// Nested message and enum types in `TransactionSignWeight`.
pub mod transaction_sign_weight {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(enumeration="result::ResponseCode", tag="1")]
        pub code: i32,
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ResponseCode {
            EnoughPermission = 0,
            /// error in
            NotEnoughPermission = 1,
            SignatureFormatError = 2,
            ComputeAddressError = 3,
            ///The key is not in permission
            PermissionError = 4,
            OtherError = 20,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionApprovedList {
    #[prost(bytes="vec", repeated, tag="2")]
    pub approved_list: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, optional, tag="4")]
    pub result: ::core::option::Option<transaction_approved_list::Result>,
    #[prost(message, optional, tag="5")]
    pub transaction: ::core::option::Option<TransactionExtention>,
}
/// Nested message and enum types in `TransactionApprovedList`.
pub mod transaction_approved_list {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(enumeration="result::ResponseCode", tag="1")]
        pub code: i32,
        #[prost(string, tag="2")]
        pub message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ResponseCode {
            Success = 0,
            SignatureFormatError = 1,
            ComputeAddressError = 2,
            OtherError = 20,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IvkDecryptParameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IvkDecryptAndMarkParameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="5")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OvkDecryptParameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptNotes {
    #[prost(message, repeated, tag="1")]
    pub note_txs: ::prost::alloc::vec::Vec<decrypt_notes::NoteTx>,
}
/// Nested message and enum types in `DecryptNotes`.
pub mod decrypt_notes {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoteTx {
        #[prost(message, optional, tag="1")]
        pub note: ::core::option::Option<super::Note>,
        ///transaction id =  sha256(transaction.rowdata)
        #[prost(bytes="vec", tag="2")]
        pub txid: ::prost::alloc::vec::Vec<u8>,
        ///the index of note in receive
        #[prost(int32, tag="3")]
        pub index: i32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptNotesMarked {
    #[prost(message, repeated, tag="1")]
    pub note_txs: ::prost::alloc::vec::Vec<decrypt_notes_marked::NoteTx>,
}
/// Nested message and enum types in `DecryptNotesMarked`.
pub mod decrypt_notes_marked {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoteTx {
        #[prost(message, optional, tag="1")]
        pub note: ::core::option::Option<super::Note>,
        ///transaction id =  sha256(transaction.rowdata)
        #[prost(bytes="vec", tag="2")]
        pub txid: ::prost::alloc::vec::Vec<u8>,
        ///the index of note in receive
        #[prost(int32, tag="3")]
        pub index: i32,
        #[prost(bool, tag="4")]
        pub is_spend: bool,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Note {
    #[prost(int64, tag="1")]
    pub value: i64,
    #[prost(string, tag="2")]
    pub payment_address: ::prost::alloc::string::String,
    /// random 32
    #[prost(bytes="vec", tag="3")]
    pub rcm: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub memo: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendNote {
    #[prost(message, optional, tag="3")]
    pub note: ::core::option::Option<Note>,
    /// random number for spend authority signature
    #[prost(bytes="vec", tag="4")]
    pub alpha: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub voucher: ::core::option::Option<IncrementalMerkleVoucher>,
    /// path for cm from leaf to root in merkle tree
    #[prost(bytes="vec", tag="6")]
    pub path: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveNote {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateParameters {
    #[prost(bytes="vec", tag="1")]
    pub transparent_from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub from_amount: i64,
    #[prost(message, repeated, tag="6")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNote>,
    #[prost(message, repeated, tag="7")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="8")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="9")]
    pub to_amount: i64,
    /// timeout in seconds, it works only when it bigger than 0
    #[prost(int64, tag="10")]
    pub timeout: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateParametersWithoutAsk {
    #[prost(bytes="vec", tag="1")]
    pub transparent_from_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub from_amount: i64,
    #[prost(message, repeated, tag="6")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNote>,
    #[prost(message, repeated, tag="7")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="8")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="9")]
    pub to_amount: i64,
    /// timeout in seconds, it works only when it bigger than 0
    #[prost(int64, tag="10")]
    pub timeout: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendAuthSigParameters {
    #[prost(bytes="vec", tag="1")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub alpha: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfParameters {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
    #[prost(message, optional, tag="2")]
    pub voucher: ::core::option::Option<IncrementalMerkleVoucher>,
    #[prost(bytes="vec", tag="3")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedSpendingKeyMessage {
    #[prost(bytes="vec", tag="1")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewingKeyMessage {
    #[prost(bytes="vec", tag="1")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomingViewingKeyMessage {
    #[prost(bytes="vec", tag="1")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiversifierMessage {
    #[prost(bytes="vec", tag="1")]
    pub d: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomingViewingKeyDiversifierMessage {
    #[prost(message, optional, tag="1")]
    pub ivk: ::core::option::Option<IncomingViewingKeyMessage>,
    #[prost(message, optional, tag="2")]
    pub d: ::core::option::Option<DiversifierMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentAddressMessage {
    #[prost(message, optional, tag="1")]
    pub d: ::core::option::Option<DiversifierMessage>,
    #[prost(bytes="vec", tag="2")]
    pub pk_d: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub payment_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedAddressInfo {
    #[prost(bytes="vec", tag="1")]
    pub sk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub d: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub pk_d: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="10")]
    pub payment_address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoteParameters {
    #[prost(bytes="vec", tag="1")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub note: ::core::option::Option<Note>,
    #[prost(bytes="vec", tag="4")]
    pub txid: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="5")]
    pub index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendResult {
    #[prost(bool, tag="1")]
    pub result: bool,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoList {
    #[prost(message, repeated, tag="1")]
    pub transaction_info: ::prost::alloc::vec::Vec<TransactionInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendNoteTrc20 {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
    #[prost(bytes="vec", tag="2")]
    pub alpha: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub path: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="5")]
    pub pos: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateShieldedTrc20Parameters {
    #[prost(bytes="vec", tag="1")]
    pub ask: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub from_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNoteTrc20>,
    #[prost(message, repeated, tag="6")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="7")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub to_amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateShieldedTrc20ParametersWithoutAsk {
    #[prost(bytes="vec", tag="1")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub nsk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="4")]
    pub from_amount: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="5")]
    pub shielded_spends: ::prost::alloc::vec::Vec<SpendNoteTrc20>,
    #[prost(message, repeated, tag="6")]
    pub shielded_receives: ::prost::alloc::vec::Vec<ReceiveNote>,
    #[prost(bytes="vec", tag="7")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub to_amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedTrc20Parameters {
    #[prost(message, repeated, tag="1")]
    pub spend_description: ::prost::alloc::vec::Vec<SpendDescription>,
    #[prost(message, repeated, tag="2")]
    pub receive_description: ::prost::alloc::vec::Vec<ReceiveDescription>,
    #[prost(bytes="vec", tag="3")]
    pub binding_signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub message_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub trigger_contract_input: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub parameter_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IvkDecryptTrc20Parameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub ivk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="7")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OvkDecryptTrc20Parameters {
    #[prost(int64, tag="1")]
    pub start_block_index: i64,
    #[prost(int64, tag="2")]
    pub end_block_index: i64,
    #[prost(bytes="vec", tag="3")]
    pub ovk: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag="5")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptNotesTrc20 {
    #[prost(message, repeated, tag="1")]
    pub note_txs: ::prost::alloc::vec::Vec<decrypt_notes_trc20::NoteTx>,
}
/// Nested message and enum types in `DecryptNotesTRC20`.
pub mod decrypt_notes_trc20 {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoteTx {
        #[prost(message, optional, tag="1")]
        pub note: ::core::option::Option<super::Note>,
        #[prost(int64, tag="2")]
        pub position: i64,
        #[prost(bool, tag="3")]
        pub is_spent: bool,
        #[prost(bytes="vec", tag="4")]
        pub txid: ::prost::alloc::vec::Vec<u8>,
        ///the index of note in txid
        #[prost(int32, tag="5")]
        pub index: i32,
        #[prost(string, tag="6")]
        pub to_amount: ::prost::alloc::string::String,
        #[prost(bytes="vec", tag="7")]
        pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfTrc20Parameters {
    #[prost(message, optional, tag="1")]
    pub note: ::core::option::Option<Note>,
    #[prost(bytes="vec", tag="2")]
    pub ak: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub nk: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="4")]
    pub position: i64,
    #[prost(bytes="vec", tag="5")]
    pub shielded_trc20_contract_address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullifierResult {
    #[prost(bool, tag="1")]
    pub is_spent: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedTrc20TriggerContractParameters {
    #[prost(message, optional, tag="1")]
    pub shielded_trc20_parameters: ::core::option::Option<ShieldedTrc20Parameters>,
    #[prost(message, repeated, tag="2")]
    pub spend_authority_signature: ::prost::alloc::vec::Vec<BytesMessage>,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub transparent_to_address: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod wallet_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WalletClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WalletClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WalletClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WalletClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_account_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_account_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountBalanceRequest>,
        ) -> Result<tonic::Response<super::AccountBalanceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_balance_trace(
            &mut self,
            request: impl tonic::IntoRequest<super::block_balance_trace::BlockIdentifier>,
        ) -> Result<tonic::Response<super::BlockBalanceTrace>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockBalanceTrace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use CreateTransaction2 instead of this function.
        pub async fn create_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of CreateTransaction.
        pub async fn create_transaction2(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateTransaction2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn broadcast_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> Result<tonic::Response<super::Return>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/BroadcastTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use UpdateAccount2 instead of this function.
        pub async fn update_account(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUpdateContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_account_id(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAccountIdContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/SetAccountId",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of UpdateAccount.
        pub async fn update_account2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountUpdateContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAccount2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use VoteWitnessAccount2 instead of this function.
        pub async fn vote_witness_account(
            &mut self,
            request: impl tonic::IntoRequest<super::VoteWitnessContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/VoteWitnessAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///modify the consume_user_resource_percent
        pub async fn update_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateSetting",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///modify the energy_limit
        pub async fn update_energy_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnergyLimitContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateEnergyLimit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of VoteWitnessAccount.
        pub async fn vote_witness_account2(
            &mut self,
            request: impl tonic::IntoRequest<super::VoteWitnessContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/VoteWitnessAccount2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use CreateAssetIssue2 instead of this function.
        pub async fn create_asset_issue(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetIssueContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAssetIssue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of CreateAssetIssue.
        pub async fn create_asset_issue2(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetIssueContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAssetIssue2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use UpdateWitness2 instead of this function.
        pub async fn update_witness(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessUpdateContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateWitness",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of UpdateWitness.
        pub async fn update_witness2(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessUpdateContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateWitness2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use CreateAccount2 instead of this function.
        pub async fn create_account(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountCreateContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of CreateAccount.
        pub async fn create_account2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountCreateContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAccount2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use CreateWitness2 instead of this function.
        pub async fn create_witness(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessCreateContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateWitness",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of CreateWitness.
        pub async fn create_witness2(
            &mut self,
            request: impl tonic::IntoRequest<super::WitnessCreateContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateWitness2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use TransferAsset2 instead of this function.
        pub async fn transfer_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferAssetContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TransferAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of TransferAsset.
        pub async fn transfer_asset2(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferAssetContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TransferAsset2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use ParticipateAssetIssue2 instead of this function.
        pub async fn participate_asset_issue(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipateAssetIssueContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ParticipateAssetIssue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of ParticipateAssetIssue.
        pub async fn participate_asset_issue2(
            &mut self,
            request: impl tonic::IntoRequest<super::ParticipateAssetIssueContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ParticipateAssetIssue2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use FreezeBalance2 instead of this function.
        pub async fn freeze_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::FreezeBalanceContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/FreezeBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of FreezeBalance.
        pub async fn freeze_balance2(
            &mut self,
            request: impl tonic::IntoRequest<super::FreezeBalanceContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/FreezeBalance2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use UnfreezeBalance2 instead of this function.
        pub async fn unfreeze_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeBalanceContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of UnfreezeBalance.
        pub async fn unfreeze_balance2(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeBalanceContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeBalance2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use UnfreezeAsset2 instead of this function.
        pub async fn unfreeze_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeAssetContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of UnfreezeAsset.
        pub async fn unfreeze_asset2(
            &mut self,
            request: impl tonic::IntoRequest<super::UnfreezeAssetContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UnfreezeAsset2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use WithdrawBalance2 instead of this function.
        pub async fn withdraw_balance(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawBalanceContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/WithdrawBalance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of WithdrawBalance.
        pub async fn withdraw_balance2(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawBalanceContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/WithdrawBalance2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use UpdateAsset2 instead of this function.
        pub async fn update_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAssetContract>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of UpdateAsset.
        pub async fn update_asset2(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAssetContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateAsset2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn proposal_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposalCreateContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ProposalCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn proposal_approve(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposalApproveContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ProposalApprove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn proposal_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposalDeleteContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ProposalDelete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn buy_storage(
            &mut self,
            request: impl tonic::IntoRequest<super::BuyStorageContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/BuyStorage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn buy_storage_bytes(
            &mut self,
            request: impl tonic::IntoRequest<super::BuyStorageBytesContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/BuyStorageBytes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn sell_storage(
            &mut self,
            request: impl tonic::IntoRequest<super::SellStorageContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/SellStorage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exchange_create(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeCreateContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeCreate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exchange_inject(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeInjectContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeInject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exchange_withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeWithdrawContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeWithdraw",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn exchange_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::ExchangeTransactionContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ExchangeTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn market_sell_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketSellAssetContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/MarketSellAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn market_cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketCancelOrderContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/MarketCancelOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_order_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::MarketOrder>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketOrderById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_order_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::MarketOrderList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketOrderByAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_price_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> Result<tonic::Response<super::MarketPriceList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketPriceByPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_order_list_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> Result<tonic::Response<super::MarketOrderList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketOrderListByPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_pair_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::MarketOrderPairList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMarketPairList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::NodeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListNodes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueByAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_account_net(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> Result<tonic::Response<super::AccountNetMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountNet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_account_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> Result<tonic::Response<super::AccountResourceMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAccountResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::AssetIssueContract>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueByName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_list_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueListByName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::AssetIssueContract>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use GetNowBlock2 instead of this function.
        pub async fn get_now_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNowBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetNowBlock.
        pub async fn get_now_block2(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNowBlock2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use GetBlockByNum2 instead of this function.
        pub async fn get_block_by_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetBlockByNum.
        pub async fn get_block_by_num2(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByNum2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_count_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionCountByBlockNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use GetBlockByLimitNext2 instead of this function.
        pub async fn get_block_by_limit_next(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockLimit>,
        ) -> Result<tonic::Response<super::BlockList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLimitNext",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetBlockByLimitNext.
        pub async fn get_block_by_limit_next2(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockLimit>,
        ) -> Result<tonic::Response<super::BlockListExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLimitNext2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use GetBlockByLatestNum2 instead of this function.
        pub async fn get_block_by_latest_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::BlockList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLatestNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetBlockByLatestNum.
        pub async fn get_block_by_latest_num2(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::BlockListExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBlockByLatestNum2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn deploy_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSmartContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/DeployContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::SmartContract>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_contract_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::SmartContractDataWrapper>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetContractInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn trigger_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TriggerContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn trigger_constant_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TriggerConstantContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn clear_contract_abi(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearAbiContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ClearContractABI",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_witnesses(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::WitnessList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListWitnesses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegated_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegatedResourceMessage>,
        ) -> Result<tonic::Response<super::DelegatedResourceList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDelegatedResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegated_resource_account_index(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDelegatedResourceAccountIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_proposals(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::ProposalList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListProposals",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_paginated_proposal_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> Result<tonic::Response<super::ProposalList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPaginatedProposalList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_proposal_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::Proposal>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetProposalById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::ExchangeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ListExchanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_paginated_exchange_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> Result<tonic::Response<super::ExchangeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPaginatedExchangeList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_exchange_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::Exchange>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetExchangeById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_chain_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::ChainParameters>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetChainParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAssetIssueList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_paginated_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPaginatedAssetIssueList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn total_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/TotalTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_next_maintenance_time(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNextMaintenanceTime",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        ///Please use GetTransactionSign2 instead of this function.
        pub async fn get_transaction_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionSign>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionSign",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        ///Use this function instead of GetTransactionSign.
        pub async fn get_transaction_sign2(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionSign>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionSign2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        pub async fn create_address(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        pub async fn easy_transfer_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::EasyTransferAssetMessage>,
        ) -> Result<tonic::Response<super::EasyTransferResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/EasyTransferAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        pub async fn easy_transfer_asset_by_private(
            &mut self,
            request: impl tonic::IntoRequest<super::EasyTransferAssetByPrivateMessage>,
        ) -> Result<tonic::Response<super::EasyTransferResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/EasyTransferAssetByPrivate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        pub async fn easy_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::EasyTransferMessage>,
        ) -> Result<tonic::Response<super::EasyTransferResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/EasyTransfer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        pub async fn easy_transfer_by_private(
            &mut self,
            request: impl tonic::IntoRequest<super::EasyTransferByPrivateMessage>,
        ) -> Result<tonic::Response<super::EasyTransferResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/EasyTransferByPrivate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        pub async fn generate_address(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::AddressPrKeyPairMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GenerateAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_info_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::TransactionInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionInfoById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn account_permission_update(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPermissionUpdateContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/AccountPermissionUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::TransactionSign>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/protocol.Wallet/AddSign");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_sign_weight(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> Result<tonic::Response<super::TransactionSignWeight>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionSignWeight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_approved_list(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> Result<tonic::Response<super::TransactionApprovedList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionApprovedList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_node_info(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::NodeInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNodeInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_reward_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetRewardInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_brokerage_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBrokerageInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_brokerage(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBrokerageContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/UpdateBrokerage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// for shiededTransaction
        pub async fn create_shielded_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::PrivateParameters>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_merkle_tree_voucher_info(
            &mut self,
            request: impl tonic::IntoRequest<super::OutputPointInfo>,
        ) -> Result<
            tonic::Response<super::IncrementalMerkleVoucherInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetMerkleTreeVoucherInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptParameters>,
        ) -> Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanNoteByIvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_and_mark_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptAndMarkParameters>,
        ) -> Result<tonic::Response<super::DecryptNotesMarked>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanAndMarkNoteByIvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_note_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptParameters>,
        ) -> Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanNoteByOvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_spending_key(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetSpendingKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_expanded_spending_key(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::ExpandedSpendingKeyMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetExpandedSpendingKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_ak_from_ask(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetAkFromAsk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_nk_from_nsk(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNkFromNsk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_incoming_viewing_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ViewingKeyMessage>,
        ) -> Result<tonic::Response<super::IncomingViewingKeyMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetIncomingViewingKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_diversifier(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::DiversifierMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetDiversifier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_new_shielded_address(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::ShieldedAddressInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetNewShieldedAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_zen_payment_address(
            &mut self,
            request: impl tonic::IntoRequest<super::IncomingViewingKeyDiversifierMessage>,
        ) -> Result<tonic::Response<super::PaymentAddressMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetZenPaymentAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_rcm(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/protocol.Wallet/GetRcm");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn is_spend(
            &mut self,
            request: impl tonic::IntoRequest<super::NoteParameters>,
        ) -> Result<tonic::Response<super::SpendResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/protocol.Wallet/IsSpend");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_shielded_transaction_without_spend_auth_sig(
            &mut self,
            request: impl tonic::IntoRequest<super::PrivateParametersWithoutAsk>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedTransactionWithoutSpendAuthSig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_shield_transaction_hash(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetShieldTransactionHash",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_spend_auth_sig(
            &mut self,
            request: impl tonic::IntoRequest<super::SpendAuthSigParameters>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateSpendAuthSig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_shield_nullifier(
            &mut self,
            request: impl tonic::IntoRequest<super::NfParameters>,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldNullifier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///for shielded contract
        pub async fn create_shielded_contract_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::PrivateShieldedTrc20Parameters>,
        ) -> Result<tonic::Response<super::ShieldedTrc20Parameters>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedContractParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_shielded_contract_parameters_without_ask(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PrivateShieldedTrc20ParametersWithoutAsk,
            >,
        ) -> Result<tonic::Response<super::ShieldedTrc20Parameters>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateShieldedContractParametersWithoutAsk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptTrc20Parameters>,
        ) -> Result<tonic::Response<super::DecryptNotesTrc20>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanShieldedTRC20NotesByIvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptTrc20Parameters>,
        ) -> Result<tonic::Response<super::DecryptNotesTrc20>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/ScanShieldedTRC20NotesByOvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn is_shielded_trc20_contract_note_spent(
            &mut self,
            request: impl tonic::IntoRequest<super::NfTrc20Parameters>,
        ) -> Result<tonic::Response<super::NullifierResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/IsShieldedTRC20ContractNoteSpent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_trigger_input_for_shielded_trc20_contract(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ShieldedTrc20TriggerContractParameters,
            >,
        ) -> Result<tonic::Response<super::BytesMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTriggerInputForShieldedTRC20Contract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_common_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::Transaction>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/CreateCommonTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_info_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::TransactionInfoList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionInfoByBlockNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_burn_trx(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetBurnTrx",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_from_pending(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionFromPending",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_list_from_pending(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::TransactionIdList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetTransactionListFromPending",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_pending_size(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Wallet/GetPendingSize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod wallet_solidity_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WalletSolidityClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletSolidityClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WalletSolidityClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WalletSolidityClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WalletSolidityClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_account_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::Account>,
        ) -> Result<tonic::Response<super::Account>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAccountById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_witnesses(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::WitnessList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ListWitnesses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_paginated_asset_issue_list(
            &mut self,
            request: impl tonic::IntoRequest<super::PaginatedMessage>,
        ) -> Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetPaginatedAssetIssueList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::AssetIssueContract>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueByName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_list_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::AssetIssueList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueListByName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_asset_issue_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::AssetIssueContract>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetAssetIssueById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use GetNowBlock2 instead of this function.
        pub async fn get_now_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetNowBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetNowBlock.
        pub async fn get_now_block2(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetNowBlock2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use GetBlockByNum2 instead of this function.
        pub async fn get_block_by_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBlockByNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetBlockByNum.
        pub async fn get_block_by_num2(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::BlockExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBlockByNum2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_count_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionCountByBlockNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegated_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::DelegatedResourceMessage>,
        ) -> Result<tonic::Response<super::DelegatedResourceList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetDelegatedResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_delegated_resource_account_index(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<
            tonic::Response<super::DelegatedResourceAccountIndex>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetDelegatedResourceAccountIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_exchange_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::Exchange>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetExchangeById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::ExchangeList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ListExchanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::Transaction>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_info_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::TransactionInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionInfoById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Warning: do not invoke this interface provided by others.
        pub async fn generate_address(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::AddressPrKeyPairMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GenerateAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_merkle_tree_voucher_info(
            &mut self,
            request: impl tonic::IntoRequest<super::OutputPointInfo>,
        ) -> Result<
            tonic::Response<super::IncrementalMerkleVoucherInfo>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMerkleTreeVoucherInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptParameters>,
        ) -> Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanNoteByIvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_and_mark_note_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptAndMarkParameters>,
        ) -> Result<tonic::Response<super::DecryptNotesMarked>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanAndMarkNoteByIvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_note_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptParameters>,
        ) -> Result<tonic::Response<super::DecryptNotes>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanNoteByOvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn is_spend(
            &mut self,
            request: impl tonic::IntoRequest<super::NoteParameters>,
        ) -> Result<tonic::Response<super::SpendResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/IsSpend",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ivk(
            &mut self,
            request: impl tonic::IntoRequest<super::IvkDecryptTrc20Parameters>,
        ) -> Result<tonic::Response<super::DecryptNotesTrc20>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanShieldedTRC20NotesByIvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn scan_shielded_trc20_notes_by_ovk(
            &mut self,
            request: impl tonic::IntoRequest<super::OvkDecryptTrc20Parameters>,
        ) -> Result<tonic::Response<super::DecryptNotesTrc20>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/ScanShieldedTRC20NotesByOvk",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn is_shielded_trc20_contract_note_spent(
            &mut self,
            request: impl tonic::IntoRequest<super::NfTrc20Parameters>,
        ) -> Result<tonic::Response<super::NullifierResult>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/IsShieldedTRC20ContractNoteSpent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_reward_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetRewardInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_brokerage_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBrokerageInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn trigger_constant_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerSmartContract>,
        ) -> Result<tonic::Response<super::TransactionExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/TriggerConstantContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_transaction_info_by_block_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::TransactionInfoList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetTransactionInfoByBlockNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_order_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::MarketOrder>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketOrderById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_order_by_account(
            &mut self,
            request: impl tonic::IntoRequest<super::BytesMessage>,
        ) -> Result<tonic::Response<super::MarketOrderList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketOrderByAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_price_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> Result<tonic::Response<super::MarketPriceList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketPriceByPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_order_list_by_pair(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketOrderPair>,
        ) -> Result<tonic::Response<super::MarketOrderList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketOrderListByPair",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_market_pair_list(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::MarketOrderPairList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetMarketPairList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_burn_trx(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::NumberMessage>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletSolidity/GetBurnTrx",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod wallet_extension_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct WalletExtensionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletExtensionClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WalletExtensionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> WalletExtensionClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            WalletExtensionClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///Please use GetTransactionsFromThis2 instead of this function.
        pub async fn get_transactions_from_this(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> Result<tonic::Response<super::TransactionList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsFromThis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetTransactionsFromThis.
        pub async fn get_transactions_from_this2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> Result<tonic::Response<super::TransactionListExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsFromThis2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Please use GetTransactionsToThis2 instead of this function.
        pub async fn get_transactions_to_this(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> Result<tonic::Response<super::TransactionList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsToThis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Use this function instead of GetTransactionsToThis.
        pub async fn get_transactions_to_this2(
            &mut self,
            request: impl tonic::IntoRequest<super::AccountPaginated>,
        ) -> Result<tonic::Response<super::TransactionListExtention>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.WalletExtension/GetTransactionsToThis2",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod database_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// the api of tron's db
    #[derive(Debug, Clone)]
    pub struct DatabaseClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatabaseClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatabaseClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DatabaseClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DatabaseClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// for tapos
        pub async fn get_block_reference(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::BlockReference>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/getBlockReference",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_dynamic_properties(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::DynamicProperties>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/GetDynamicProperties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_now_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/GetNowBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_by_num(
            &mut self,
            request: impl tonic::IntoRequest<super::NumberMessage>,
        ) -> Result<tonic::Response<super::Block>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Database/GetBlockByNum",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod monitor_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MonitorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MonitorClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MonitorClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MonitorClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MonitorClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_stats_info(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyMessage>,
        ) -> Result<tonic::Response<super::MetricsInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/protocol.Monitor/GetStatsInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod network_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// the api of tron's network such as node list.
    #[derive(Debug, Clone)]
    pub struct NetworkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NetworkClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> NetworkClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> NetworkClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            NetworkClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
    }
}
