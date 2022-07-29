mod tron_grpc;
pub mod crypto;
mod private;
mod error;
mod signature;

use base58::FromBase58;
use ethers::abi::{Abi, Address, Detokenize, Tokenize};
use ethers::types::U256;
use hex::FromHex;
use once_cell::sync::Lazy;
use tonic::transport::Channel;
use tracing_subscriber::FmtSubscriber;
use crate::tron_grpc::{Account, AccountBalanceRequest, AccountIdentifier, EmptyMessage};
use crate::tron_grpc::wallet_client::WalletClient;
use prost::Message;
use tonic::IntoRequest;
use crate::tron_grpc::transaction::contract::ContractType;

const PRIVATE_KEY_1:&str="4830EC8DBE1E91557F6AE6A2EFED706821EEA5AD39FEC431B8C6860F486EF572";
const PUBLIC_KEY_1:&str="04779D7BD7B5A316DE28C30FB5B12E215718AA423F42B6C1CDA8EF61E845F2978A58529458D6D4B31FB2328DB62CBB341FC0815172F79C3423C42B7D5A5F1888E2";
const ADDR_1:&str="TYJWHBcoRe3cDp79M2R6WiY37EiHn9w7oe";
const ADDR_HEX_1:&str="41f4f917ee28322a788a8e712fa6bbcf4f8e149f34";

const PRIVATE_KEY_2:&str="9D4ED983D5A0B97CC428DB653B83C391613DF168E6267C39D74426EB19C6A55A";
const PUBLIC_KEY_2:&str="04240FC04FD997E7A466BD592BA364691EBA0AF23AD53BF343687D05667E4CAC1E0E0649112F770E9C5D10350644C4D4FF8B3AE3F0D3993A03AA3131C5E631CCF7";
const ADDR_2:&str="THpuXbUBxFWhzUKdTves8HKUJ4CEGoYscG";
const ADDR_HEX_2:&str="41562f17a8d2f0897ff5396d646df7bd705ddaa747";

static ABI: Lazy<Abi> = Lazy::new(|| {
    const USDT_ABI: &str = r#"[{"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_upgradedAddress","type":"address"}],"name":"deprecate","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_spender","type":"address"},{"name":"_value","type":"uint256"}],"name":"approve","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"deprecated","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_evilUser","type":"address"}],"name":"addBlackList","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_from","type":"address"},{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transferFrom","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"upgradedAddress","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"balances","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"decimals","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"maximumFee","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"_totalSupply","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[],"name":"unpause","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_maker","type":"address"}],"name":"getBlackListStatus","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"},{"name":"","type":"address"}],"name":"allowed","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"paused","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"who","type":"address"}],"name":"balanceOf","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[],"name":"pause","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"getOwner","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"owner","outputs":[{"name":"","type":"address"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transfer","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"newBasisPoints","type":"uint256"},{"name":"newMaxFee","type":"uint256"}],"name":"setParams","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"amount","type":"uint256"}],"name":"issue","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"amount","type":"uint256"}],"name":"redeem","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[{"name":"_owner","type":"address"},{"name":"_spender","type":"address"}],"name":"allowance","outputs":[{"name":"remaining","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[],"name":"basisPointsRate","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":true,"inputs":[{"name":"","type":"address"}],"name":"isBlackListed","outputs":[{"name":"","type":"bool"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"_clearedUser","type":"address"}],"name":"removeBlackList","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"MAX_UINT","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"},{"constant":false,"inputs":[{"name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":false,"inputs":[{"name":"_blackListedUser","type":"address"}],"name":"destroyBlackFunds","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"inputs":[{"name":"_initialSupply","type":"uint256"},{"name":"_name","type":"string"},{"name":"_symbol","type":"string"},{"name":"_decimals","type":"uint256"}],"payable":false,"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":false,"name":"amount","type":"uint256"}],"name":"Issue","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"name":"amount","type":"uint256"}],"name":"Redeem","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"name":"newAddress","type":"address"}],"name":"Deprecate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"name":"feeBasisPoints","type":"uint256"},{"indexed":false,"name":"maxFee","type":"uint256"}],"name":"Params","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"name":"_blackListedUser","type":"address"},{"indexed":false,"name":"_balance","type":"uint256"}],"name":"DestroyedBlackFunds","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"name":"_user","type":"address"}],"name":"AddedBlackList","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"name":"_user","type":"address"}],"name":"RemovedBlackList","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"owner","type":"address"},{"indexed":true,"name":"spender","type":"address"},{"indexed":false,"name":"value","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"name":"from","type":"address"},{"indexed":true,"name":"to","type":"address"},{"indexed":false,"name":"value","type":"uint256"}],"name":"Transfer","type":"event"},{"anonymous":false,"inputs":[],"name":"Pause","type":"event"},{"anonymous":false,"inputs":[],"name":"Unpause","type":"event"}]"#;
    let abi_bytes = Vec::from(USDT_ABI.as_bytes());
    Abi::load(&*abi_bytes).expect("load abi error")
});

#[tokio::main]
async fn main() {
// a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::INFO)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    get_trc20_fee_test().await;
}

async fn get_client()->WalletClient<Channel>{
    tron_grpc::wallet_client::WalletClient::connect("grpc://grpc.nile.trongrid.io:50051".to_string()).await.expect("connect error")
}

async fn get_balance_tst(){
    let mut client =  get_client().await;

    let addr="41f4f917ee28322a788a8e712fa6bbcf4f8e149f33";
    let request_obj=Account{
        account_name: vec![],
        r#type: 0,
        address: hex::decode(addr).expect("decode addr error"),
        balance: 0,
        votes: vec![],
        asset: Default::default(),
        asset_v2: Default::default(),
        frozen: vec![],
        net_usage: 0,
        acquired_delegated_frozen_balance_for_bandwidth: 0,
        delegated_frozen_balance_for_bandwidth: 0,
        old_tron_power: 0,
        tron_power: None,
        asset_optimized: false,
        create_time: 0,
        latest_opration_time: 0,
        allowance: 0,
        latest_withdraw_time: 0,
        code: vec![],
        is_witness: false,
        is_committee: false,
        frozen_supply: vec![],
        asset_issued_name: vec![],
        asset_issued_id: vec![],
        latest_asset_operation_time: Default::default(),
        latest_asset_operation_time_v2: Default::default(),
        free_net_usage: 0,
        free_asset_net_usage: Default::default(),
        free_asset_net_usage_v2: Default::default(),
        latest_consume_time: 0,
        latest_consume_free_time: 0,
        account_id: vec![],
        account_resource: None,
        code_hash: vec![],
        owner_permission: None,
        witness_permission: None,
        active_permission: vec![]
    };
    let mut request_obj:Account=Default::default();
    request_obj.address= hex::decode(addr).expect("decode addr error");

    let resp= client.get_account(request_obj).await.expect("get account error");

    let account= resp.into_inner();
    println!("Hello, world!:{:?}",&account);
}

async fn get_trc10_token_info(){
    let asset_id="1000016";
    let mut client =  get_client().await;
    let result= client.get_asset_issue_by_id(tron_grpc::BytesMessage{
        value: asset_id.as_bytes().to_vec()
    }).await.expect("get asset info error");

    println!("result:{:?}",&result);

    let result= result.into_inner();
    println!("full name:{}", String::from_utf8(result.name.to_vec()).expect("decode error"));
    println!("symbol:{}", String::from_utf8(result.abbr.to_vec()).expect("decode error"));
    println!("id:{}", &result.id);
    println!("total_supply::{}", &result.total_supply);
    println!("precision::{}", &result.precision);
}

async fn get_trc20_token_info(){
    let mut contract_addr_bytes=   "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj".from_base58().expect("base 58 decode error");
    let contract_addr_bytes=contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec();

    let mut client =  get_client().await;

   let name:String = call_trc20(&mut client,&contract_addr_bytes,"name",()).await;
   let symbol:String = call_trc20(&mut client,&contract_addr_bytes,"symbol",()).await;
    let decimal:u128 = call_trc20(&mut client,&contract_addr_bytes,"decimals",()).await;
    let total_supply:u128 = call_trc20(&mut client,&contract_addr_bytes,"totalSupply",()).await;

    println!("full name:{}",name);
    println!("symbol:{}", symbol);
    println!("decimal:{}", decimal);
    println!("total_supply:{}", total_supply);
}

async fn get_trc20_balance(){
    let mut contract_addr_bytes=   "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj".from_base58().expect("base 58 decode error");
    let contract_addr_bytes=contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec();
    let from= hex::decode( ADDR_HEX_1).expect("decode error");

    let mut client =  get_client().await;

    let balance_result:u128 = call_trc20(&mut client,&contract_addr_bytes,"balanceOf",(Address::from_slice(&from[1..]),)).await;

    println!("balance result:{}",balance_result);
}

async fn call_trc20<TInput:Tokenize,TOutput:Detokenize>(client_obj:&mut WalletClient<Channel>,contract_addr:&[u8],fn_name:&str,param_data:TInput)->TOutput{
    let fn_obj= ABI.function(fn_name).expect("get fn error");
    let param_data= ethers::contract::encode_function_data(fn_obj,param_data).expect("encode fn param error");
    let caller=tron_grpc::TriggerSmartContract{
        owner_address: vec![],
        contract_address: contract_addr.to_vec(),
        call_value: 0,
        data: param_data.to_vec(),
        call_token_value: 0,
        token_id: 0
    };

    let result= client_obj.trigger_constant_contract(caller.clone()).await.expect("get asset info error");
    println!("result:{:?}",&result);

    let result =result.into_inner();
    println!("energy_used:{}",result.energy_used);
    ethers::contract::decode_function_data::<TOutput,_>(fn_obj,&result.constant_result[0],false).expect("decode error")
}

// 代码借鉴自: https://github.com/andelf/rust-tron/blob/master/wallet-cli/src/utils/trx.rs#L179
// trx\rust-tron\wallet-cli\src\utils\trx.rs
async fn transfer_trx(){
    let mut client =  get_client().await;
    let from=hex::decode( ADDR_HEX_1).expect("decode error");
    let to=hex::decode( ADDR_HEX_2).expect("decode error");
    let amount=20000; // 2个
    let secret_obj=private::Private::from_hex (PRIVATE_KEY_1).expect("decode error");

    let now_block= client.get_now_block2(EmptyMessage{}).await.expect("get block error");
    let now_block= now_block.into_inner();

    let contract_type=tron_grpc::transaction::contract::ContractType::TransferContract;
    let transfer_tx = tron_grpc::TransferContract{
        owner_address: from,
        to_address: to,
        amount
    };

    let mut raw_bytes:Vec<u8>=Vec::new();
    transfer_tx.encode(&mut raw_bytes);
    let tx_any= prost_types::Any{
        type_url: format!("type.googleapis.com/protocol.{:?}", &contract_type),
        value: raw_bytes
    };

    let contract=tron_grpc::transaction::Contract{
        r#type: contract_type.clone() as i32 ,
        parameter: Some(tx_any),
        provider: vec![],
        contract_name: vec![],
        permission_id: 0
    };

    let block_num=now_block.block_header.expect("get header error") .raw_data.expect("get raw data error") .number;

    let now=chrono::Utc::now().timestamp_millis();
    let mut raw= tron_grpc::transaction::Raw{
        ref_block_bytes: vec![
            ((block_num & 0xff00) >> 8) as u8,
            (block_num & 0xff) as u8,
        ],
        ref_block_num:block_num ,
        ref_block_hash: now_block.blockid[8..16].to_owned(),
        expiration: now+61*1000, // 61秒过期
        auths: vec![],
        data: vec![],
        contract: vec![contract],
        scripts: vec![],
        timestamp:now,
        fee_limit: 0
    };

    //prost::Message::encode()
    let mut raw_bytes:Vec<u8>=Vec::new();
    raw.encode(&mut raw_bytes);

    let txid = crypto::sha256(&raw_bytes);
    println!("txid:{}",hex::encode(&txid));

    let sign_val = secret_obj.sign_digest(txid.as_slice()).expect("sign error");

    let req=tron_grpc::Transaction{
        raw_data: Some(raw),
        signature: vec![
            sign_val.to_vec()
        ],
        ret: vec![]
    };

    let result= client.broadcast_transaction(req).await.expect("broadcast error");
    println!("result: {:?}",&result);
}

async fn transfer_trc10(){
    let mut client =  get_client().await;
    let from=hex::decode( ADDR_HEX_1).expect("decode error");
    let to=hex::decode( ADDR_HEX_2).expect("decode error");
    let amount=100000; // 2个
    let secret_obj=private::Private::from_hex (PRIVATE_KEY_1).expect("decode error");
    let asset_id="1000016";

    let now_block= client.get_now_block2(EmptyMessage{}).await.expect("get block error");
    let now_block= now_block.into_inner();

    let contract_type=tron_grpc::transaction::contract::ContractType::TransferAssetContract;
    let transfer_tx = tron_grpc::TransferAssetContract{
        asset_name: asset_id.as_bytes().to_vec(),
        owner_address: from,
        to_address: to,
        amount
    };

    let mut raw_bytes:Vec<u8>=Vec::new();
    transfer_tx.encode(&mut raw_bytes);
    let tx_any= prost_types::Any{
        type_url: format!("type.googleapis.com/protocol.{:?}", &contract_type),
        value: raw_bytes
    };

    let contract=tron_grpc::transaction::Contract{
        r#type: contract_type.clone() as i32 ,
        parameter: Some(tx_any),
        provider: vec![],
        contract_name: vec![],
        permission_id: 0
    };

    let block_num=now_block.block_header.expect("get header error") .raw_data.expect("get raw data error") .number;

    let now= chrono::Utc::now().timestamp_millis();
    let mut raw= tron_grpc::transaction::Raw{
        ref_block_bytes: vec![
            ((block_num & 0xff00) >> 8) as u8,
            (block_num & 0xff) as u8,
        ],
        ref_block_num:block_num ,
        ref_block_hash: now_block.blockid[8..16].to_owned(),
        expiration: now+61*1000, // 61秒过期
        auths: vec![],
        data: vec![],
        contract: vec![contract],
        scripts: vec![],
        timestamp:now,
        fee_limit: 0
    };

    //prost::Message::encode()
    let mut raw_bytes:Vec<u8>=Vec::new();
    raw.encode(&mut raw_bytes);

    let txid = crypto::sha256(&raw_bytes);
    println!("txid:{}",hex::encode(&txid));

    let sign_val = secret_obj.sign_digest(txid.as_slice()).expect("sign error");

    let req=tron_grpc::Transaction{
        raw_data: Some(raw),
        signature: vec![
            sign_val.to_vec()
        ],
        ret: vec![]
    };

    let result= client.broadcast_transaction(req).await.expect("broadcast error");
    println!("result: {:?}",&result);
}

async fn transfer_trc20(){
    let mut client =  get_client().await;
    let from=hex::decode( ADDR_HEX_1).expect("decode error");
    let to=hex::decode( "411F4DC709965643A75F330E554A24023AA0203CC6").expect("decode error");
    let amount:ethers::types::U256 =U256::from( 5000000u128); // 2个
    let secret_obj=private::Private::from_hex (PRIVATE_KEY_1).expect("decode error");
    let mut contract_addr_bytes=   "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj".from_base58().expect("base 58 decode error");
    println!("contract addr:{}",hex::encode(&contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec()));
    //contract_addr_bytes=contract_addr_bytes[1..].to_vec(); // 地址第一个字节是tron前缀，所以需要去掉

    let now_block= client.get_now_block2(EmptyMessage{}).await.expect("get block error");
    let now_block= now_block.into_inner();

    let fn_obj= ABI.function("transfer").expect("get fn error");
    let param_data= ethers::contract::encode_function_data(fn_obj,(Address::from_slice(&to[1..]),amount)).expect("encode fn param error");
    println!("param data:{}",hex::encode( param_data.as_ref()));

    let contract_type=tron_grpc::transaction::contract::ContractType::TriggerSmartContract;
    let transfer_tx = tron_grpc::TriggerSmartContract{
        owner_address: from,
        contract_address: contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec(),
        call_value: 0,
        data: param_data.to_vec(),
        call_token_value: 0,
        token_id: 0
    };

    let mut raw_bytes:Vec<u8>=Vec::new();
    transfer_tx.encode(&mut raw_bytes);
    let tx_any= prost_types::Any{
        type_url: format!("type.googleapis.com/protocol.{:?}", &contract_type),
        value: raw_bytes
    };

    let contract=tron_grpc::transaction::Contract{
        r#type: contract_type.clone() as i32 ,
        parameter: Some(tx_any),
        provider: vec![],
        contract_name: vec![],
        permission_id: 0
    };

    let block_num=now_block.block_header.expect("get header error") .raw_data.expect("get raw data error") .number;

    let now=chrono::Utc::now().timestamp_millis();
    let mut raw= tron_grpc::transaction::Raw{
        ref_block_bytes: vec![
            ((block_num & 0xff00) >> 8) as u8,
            (block_num & 0xff) as u8,
        ],
        ref_block_num:block_num ,
        ref_block_hash: now_block.blockid[8..16].to_owned(),
        expiration: now+61*1000, // 61秒过期
        auths: vec![],
        data: vec![],
        contract: vec![contract],
        scripts: vec![],
        timestamp:now,
        fee_limit: 6000000
    };

    //prost::Message::encode()
    let mut raw_bytes:Vec<u8>=Vec::new();
    raw.encode(&mut raw_bytes);

    let txid = crypto::sha256(&raw_bytes);
    println!("txid:{}",hex::encode(&txid));

    let sign_val = secret_obj.sign_digest(txid.as_slice()).expect("sign error");

    let req=tron_grpc::Transaction{
        raw_data: Some(raw),
        signature: vec![
            sign_val.to_vec()
        ],
        ret: vec![]
    };

    let mut raw_bytes:Vec<u8>=Vec::new();
    req.encode(&mut raw_bytes);
    println!("req len:{}",raw_bytes.len());

        let result= client.broadcast_transaction(req).await.expect("broadcast error");
        println!("result: {:?}",&result);
        let result=result.into_inner();
        println!("message:{}",   String::from_utf8(result.message).expect("decode message error"));
}

async fn get_trc20_fee_test(){
    let mut client =  get_client().await;
    let from=hex::decode( ADDR_HEX_1).expect("decode error");
    let to=hex::decode( ADDR_HEX_2).expect("decode error");
    let amount:ethers::types::U256 =U256::from( 5000000u128); // 2个
    let secret_obj=private::Private::from_hex (PRIVATE_KEY_1).expect("decode error");
    let mut contract_addr_bytes=   "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj".from_base58().expect("base 58 decode error");
    println!("contract addr:{}",hex::encode(&contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec()));
    //contract_addr_bytes=contract_addr_bytes[1..].to_vec(); // 地址第一个字节是tron前缀，所以需要去掉

    let now_block= client.get_now_block2(EmptyMessage{}).await.expect("get block error");
    let now_block= now_block.into_inner();

    let fn_obj= ABI.function("transfer").expect("get fn error");
    let param_data= ethers::contract::encode_function_data(fn_obj,(Address::from_slice(&to[1..]),amount)).expect("encode fn param error");
    println!("param data:{}",hex::encode( param_data.as_ref()));

    let transfer_tx = tron_grpc::TriggerSmartContract{
        owner_address: from.clone(),
        contract_address: contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec(),
        call_value: 0,
        data: param_data.to_vec(),
        call_token_value: 0,
        token_id: 0
    };
    // energy ->通过接口获取
    // bandwidth -->是一个常量
    // trx
    let energy_need= get_trc20_energy_need(&mut client,ADDR_HEX_1,ADDR_HEX_2,amount.as_u128()).await;
    println!("energy_need:{}",energy_need);
    let bindwidth_need=get_tx_bandwidth_need(&mut client,transfer_tx,ContractType::TriggerSmartContract).await;
    println!("bind width need:{}",bindwidth_need);

    let mut request_obj:Account=Default::default();
    request_obj.address=from.clone();
    let resp= client.get_account_resource(request_obj).await.expect("get account error");
    let resp=resp.into_inner();
    let energy_free= (resp.free_net_limit-resp.free_net_used) +(resp.net_limit-resp.net_used);
    let need_trx= ((energy_need as i64)-energy_free)* 280;
    println!("trx need:{}",need_trx);
}

async fn get_trc10_fee_test(){
    let mut client =  get_client().await;
    let from=hex::decode( ADDR_HEX_1).expect("decode error");
    let to=hex::decode( ADDR_HEX_2).expect("decode error");
    let amount =5000000u128; // 2个
    let asset_id="1000016";

    let secret_obj=private::Private::from_hex (PRIVATE_KEY_1).expect("decode error");
    let mut contract_addr_bytes=   "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj".from_base58().expect("base 58 decode error");
    println!("contract addr:{}",hex::encode(&contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec()));
    //contract_addr_bytes=contract_addr_bytes[1..].to_vec(); // 地址第一个字节是tron前缀，所以需要去掉

    let contract_type=tron_grpc::transaction::contract::ContractType::TransferAssetContract;
    let transfer_tx = tron_grpc::TransferAssetContract{
        asset_name: asset_id.as_bytes().to_vec(),
        owner_address: from,
        to_address: to,
        amount:amount as i64
    };

    // energy ->通过接口获取
    // bandwidth -->是一个常量
    // trx
    //let energy_need= get_trc20_energy_need(&mut client,ADDR_HEX_1,ADDR_HEX_2,amount.as_u128()).await;
    let bindwidth_need=get_tx_bandwidth_need(&mut client,transfer_tx,contract_type).await;
    println!("bind width need:{}",bindwidth_need);
}

async fn get_trx_fee_test(){
    let mut client =  get_client().await;
    let from=hex::decode( ADDR_HEX_1).expect("decode error");
    let to=hex::decode( ADDR_HEX_2).expect("decode error");
    let amount =5000000u128; // 2个
    let asset_id="1000016";

    let secret_obj=private::Private::from_hex (PRIVATE_KEY_1).expect("decode error");
    let mut contract_addr_bytes=   "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj".from_base58().expect("base 58 decode error");
    println!("contract addr:{}",hex::encode(&contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec()));
    //contract_addr_bytes=contract_addr_bytes[1..].to_vec(); // 地址第一个字节是tron前缀，所以需要去掉

    let contract_type=tron_grpc::transaction::contract::ContractType::TransferContract;
    let transfer_tx = tron_grpc::TransferContract{
        owner_address: from,
        to_address: to,
        amount:amount as i64
    };

    // energy ->通过接口获取
    // bandwidth -->是一个常量
    // trx
    //let energy_need= get_trc20_energy_need(&mut client,ADDR_HEX_1,ADDR_HEX_2,amount.as_u128()).await;
    let bindwidth_need=get_tx_bandwidth_need(&mut client,transfer_tx,contract_type).await;
    println!("bind width need:{}",bindwidth_need);
}

/// 计算参考文档:
/// https://cn.developers.tron.network/docs/resource-model
/// https://github.com/tronprotocol/java-tron/issues/2982
/// https://developers.tron.network/docs/faq#5-how-to-calculate-the-bandwidth-and-energy-consumed-when-calling-the-contract
async fn get_trc20_energy_need(client:&mut WalletClient<Channel>,from:&str,to:&str,amount:u128)->u64{
    let from=hex::decode( from).expect("decode error");
    let to=hex::decode( to).expect("decode error");
    let amount:ethers::types::U256 =U256::from( amount); // 2个
    let mut contract_addr_bytes=   "TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj".from_base58().expect("base 58 decode error");

    let fn_obj= ABI.function("transfer").expect("get fn error");
    let param_data= ethers::contract::encode_function_data(fn_obj,(Address::from_slice(&to[1..]),amount)).expect("encode fn param error");
    println!("param data:{}",hex::encode( param_data.as_ref()));

    let transfer_tx = tron_grpc::TriggerSmartContract{
        owner_address: from,
        contract_address: contract_addr_bytes[..contract_addr_bytes.len()-4].to_vec(),
        call_value: 0,
        data: param_data.to_vec(),
        call_token_value: 0,
        token_id: 0
    };
    // energy ->通过接口获取
    // bandwidth -->是一个常量
    // trx
    let result= client.trigger_constant_contract(transfer_tx).await.expect("broadcast error");
    let result=result.into_inner();
    result.energy_used as u64
}

/// 计算参考文档
/// https://developers.tron.network/docs/bandwith
/// https://github.com/tronprotocol/java-tron/issues/2128
/// https://cn.developers.tron.network/docs/resource-model
async fn get_tx_bandwidth_need<TInput:Message>(client:&mut WalletClient<Channel>,contract:TInput,contract_type:ContractType)->u64{
    let mut raw_bytes:Vec<u8>=Vec::new();
    contract.encode(&mut raw_bytes);
    let tx_any= prost_types::Any{
        type_url: format!("type.googleapis.com/protocol.{:?}", &contract_type),
        value: raw_bytes
    };

    let contract=tron_grpc::transaction::Contract{
        r#type: contract_type.clone() as i32 ,
        parameter: Some(tx_any),
        provider: vec![],
        contract_name: vec![],
        permission_id: 0
    };

    let block_num=0i64;

    let now=chrono::Utc::now().timestamp_millis();
    let mut raw= tron_grpc::transaction::Raw{
        ref_block_bytes: vec![
            ((block_num & 0xff00) >> 8) as u8,
            (block_num & 0xff) as u8,
        ],
        ref_block_num:block_num ,
        ref_block_hash: vec![0u8;8].to_owned(),
        expiration: now+61*1000, // 61秒过期
        auths: vec![],
        data: vec![],
        contract: vec![contract],
        scripts: vec![],
        timestamp:now,
        fee_limit: 100000
    };
    let mut raw_bytes:Vec<u8>=Vec::new();
    raw.encode(&mut raw_bytes);
    println!("raw bytes len:{}",raw_bytes.len());

    let req=tron_grpc::Transaction{
        raw_data: Some(raw),
        signature: vec![vec![0u8;65]],
        ret: vec![]
    };

    let mut raw_bytes:Vec<u8>=Vec::new();
    req.encode(&mut raw_bytes);
    println!("req bytes len:{}",raw_bytes.len());

    const REQUEST_EXTRA_BYTES:u64=70;
    REQUEST_EXTRA_BYTES+raw_bytes.len() as u64
}