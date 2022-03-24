# Rust Web3 basics tutorial

The inspiration of this repo and detailed explaination:
[Rust Web3: Blog post](https://tms-dev-blog.com/rust-web3-connect-to-ethereum/).
[Repo](https://github.com/tmsdev82/rust-web3-basics-tutorial.git)

Additional: tried it on mainnet, get the erc20 balanace of balance of a random address, symbol. 
## Set up and run

The project requires a .env file in the root directory to run properly. The .env file should contain the following:

```bash
INFURA_WSS=wss://rinkeby.infura.io/ws/v3/xxxxxxx
ACCOUNT_ADDRESS=xxxxxxxxxx
```

The `INFURA_WSS` value is an endpoint address from [infura.io](https://infura.io), however it can be any valid address to an Ethereum network WebSocket endpoint.

The `ACCOUNT_ADDRESS` is the address of a Ethereum wallet, not including the "0x" prefix.

If this configuration is in place the program can be run using cargo:

```bash
cargo run
```
