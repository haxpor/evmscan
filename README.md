# bscscan
bscscan.com non-async API in Rust

**WIP** and will be gradually filled with APIs as provided on bscscan.com side.

# API supports

See which APIs this project supports as seen in checking mark below.

## Accounts
- [x] Get BNB balance for a single address
- [x] Get BNB balance for multiple addresses in a single cal
- [ ] Get historical BNB balance for a single address by block number
- [x] Get a list of normal transactions by address (limited by API to only maximum of 10,000 records)
- [x] Get a list of internal transactions by address (limited by API to only maximum of 10,000 records)
- [ ] Get 'internal transactions' by transaction hash
- [ ] Get 'internal transactions' by block range
- [x] Get a list of 'BEP-20' token transfer events from an EOA address (specify `address`)
- [ ] Get a list of 'BEP-20' token transfer events from a contract address (specify `contractaddress`)
- [ ] Get a list of 'BEP-20' token transfer events from an EOA address filtered by a token contract (specify `address` and `contractaddress`)
- [ ] Get a list of 'BEP-721' token transfer events from an EOA address (specify `address`)
- [ ] Get a list of 'BEP-721' token transfer events from a contract address (specify `contractaddress`)
- [ ] Get a list of 'BEP-721' token transfer events from an EOA address filtered by a token contract (specify `address` and `contractaddress`)
- [ ] Get a list of blocks validated by address

## Contracts

- [ ] Get contract ABI for verified contract source code
- [ ] Get contract source code for verified contract source codes
- [ ] Verify source code
- [ ] Verify proxy contract

## Transactions

- [ ] Check transaction receipt status

## Blocks

- [ ] Get block rewards by block number
- [ ] Get estimated block countdown time by block number
- [ ] Get block number by timestamp
- [ ] `PRO API` Get daily average block size
- [ ] `PRO API` Get daily block count and rewards
- [ ] `PRO API` Get daily block rewards
- [ ] `PRO API` Get daily average time for a block to be included in the BNB Smart Chain

## Logs

- [ ] Get logs using filter parameters

## Geth Proxy

- [ ] `eth_blockNumber` - returns the number of most recent block
- [ ] `eth_getBlockByNumber` - returns information about a block by block number
- [ ] `eth_getBlockTransactionCountByNumber` - returns the number of transactions in a block
- [ ] `eth_getTransactionByHash` - returns information about a transaction requested by transaction hash
- [ ] `eth_getTransactionByBlockNumberAndIndex` - returns information about a transaction by block number and transaction index position
- [ ] `eth_getTransactionCount` - returns the number of transactions performed by an address
- [ ] `eth_sendRawTransaction` - submits a pre-signed transaction for broadcast to the BNB Smart Chain network
- [ ] `eth_getTransactionReceipt` - returns the receipt of a transaction that has been validated
- [ ] `eth_call` - executes a new message call (read function) immediately without creating a transaction on the blockchain
- [ ] `eth_getCode` - returns code a given address
- [ ] `eth_getStorageAt` (`experimental`) - returns the value from a storage position at a given address
- [ ] `eth_gasPrice` - returns the current price per gas in wei
- [ ] `eth_estimateGas` - makes a call or transaction, which won't be added to the blockchain and returns the gas used

## Tokens

- [ ] Get 'BEP-20' token total supply by contract address
- [ ] Get 'BEP-20' token circulating supply by contract address
- [ ] Get 'BEP-20' token account balance by contract address
- [ ] `PRO API` Get token holder list by contract address
- [ ] `PRO API` Get historical 'BEP-20' token total supply by contract address & block number
- [ ] `PRO API` Get historical 'BEP-20' token account balance by contract address & block number
- [ ] `PRO API` Get token info by contract address
- [ ] `PRO API` Get address 'BEP-20' token holding
- [ ] `PRO API` Get address 'BEP-721' token holding
- [ ] `PRO API` Get address 'BEP-721' token inventory by contract address

## Gas Tracker

- [ ] Get gas oracle
- [ ] `PRO API` Get daily average gas limit
- [ ] `PRO API` Get BNB Smart Chain Daily total gas used
- [ ] `PRO API` Get daily average gas price

## Stats

- [ ] Get total supply of BNB on the BNB Smart Chain
- [ ] Get validators list on the BNB Smart Chain
- [ ] Get BNB last price
- [ ] `PRO API` Get BNB historical price
- [ ] `PRO API` Get daily network transaction fee
- [ ] `PRO API` Get daily new address count
- [ ] `PRO API` Get daily network utilization
- [ ] `PRO API` Get daily transaction count

# Test

* Define your bscscan.com's api key via environment variable namely `BSCSCAN_TEST_APIKEY` (this is used for cargo testing only, you don't have to define it to use this library).
* Execute `cargo test`.

# License
MIT, Wasin Thonkaew
