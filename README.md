# BazaarEx

An online marketplace for second-hand electronic goods with decentralised logistics

Built on [Fluence](https://fluence.network/) and [Ethereum](https://ethereum.org/en/)

> Do not use for production!

## Usage

### Build and deploy ERC721 smart contract using Truffle

1.  `cd nft_contract`
2.  `npm install`
3.  `truffle compile`
4.  `truffle develop` - This will start the RPC server and development blockchain environment
5.  `migrate` in Truffle console to deploy contract
6.  Note down the ERC721 contract address and owner address

> To log requests on the Truffle RPC server, run `truffle develop --log`

### Build and run BazaarEx

1.  Change `CON_OWNER` and `CON_ADD` variables to owner and contract address respectively in `src/nft_contract_adapter.rs`
2.  `./build.sh`
    > Note: For demo purposes, each account registered receives 1 ETH from the contract owner

## File structure

- `/src` BazaarEx logic code
- `/curl_adapter` [Fluence curl adapter ](https://github.com/fluencelabs/examples/tree/main/archived/web3-examples/web3-examples/curl_adapter)
- `/web3` Custom Web3 library modified from [Fluence example](https://github.com/fluencelabs/examples/tree/main/archived/web3-examples/web3-examples/facade)
- `/ethereum-tx-sign` - Custom raw transaction signing library modified from [ethereum-tx-sign](https://github.com/synlestidae/ethereum-tx-sign) to use [libsecp256k1](https://github.com/paritytech/libsecp256k1)
- `/nft_contract` Truffle project with ERC721 Solidity files
