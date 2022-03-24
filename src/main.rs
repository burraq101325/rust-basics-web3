use std::env;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::types::{Address, H160, U256};

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_WSS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("Eth balance of {:?}: {}", account, wei_to_eth(balance));
    }

    let aave_addr = Address::from_str("0x7D1AfA7B718fb893dB30A3aBc0Cfc608AaCfeBB0").unwrap();
    let token_contract =
        Contract::from_json(web3s.eth(), aave_addr, include_bytes!("erc20_abi.json")).unwrap();

    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap();

    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap();
    
    let token_decimal: u128 = token_contract
        .query("decimals", (), None, Options::default(), None)
        .await
        .unwrap();    
    
    let is_paused: bool = token_contract
        .query("paused", (), None, Options::default(), None)
        .await
        .unwrap();    
    let symbol: String = token_contract
        .query("symbol", (), None, Options::default(), None)
        .await
        .unwrap();
        
    let random_address = Address::from_str("0xC2d1266205aa5c80984a3D56d3FFbC23C971FB05").unwrap();
    let balance: U256 = token_contract
        .query("balanceOf", random_address, None, Options::default(), None)
        .await
        .unwrap();
        println!("
        Token name: {}
        Token Symbol: {}
        Token decimals: {}
        Total supply: {}
        isPaused: {}
        balanceOf Random Address {}", token_name, symbol,token_decimal, wei_to_eth(total_supply), is_paused, wei_to_eth(balance));

    Ok(())
}