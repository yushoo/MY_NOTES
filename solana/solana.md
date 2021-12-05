# Notes

## How does Solana work

- Solana
  - A set of validators working together to serve client transactions and maintain the intergrity of the ledger
  - Solana defines confirmation as the duration of time from when the leader timestamps a new entry to the moment when it recognizes a supermajority of ledger votes
  - Traditional blockchains synchronize on large chunks of transactions called blocks. By synchronizing on blocks, a transaction cannot be processed until a duration, called "block time", has passed. In a proof of work consensus, these block times need to be very large to minimize the odds of multiple validators producing a new valid block at the same time. There's no such contraint in proof of stake consensus, but without reliable timestamps, a calidator cannot determine the order of incoming blocks.
  - Solana takes a very different approach, which it calls Proof of history. Leader nodes "timestamp" blocks with cryptographic proofs that some duration of time has passed since the last proof.
  - Stakers are rewarded for helping to validate the ledger

## Building on Solana

- Programming Model
  - An app interacts with a Solana cluster by sending it transactions with one or more instructions. The Solana runtime passes those instructions to programs deployed by app developers beforehand.
  - Transactions:

## What is an RPC Node

- Remote procedure call
- Allows developers to communicate to a server in order to remotely execute code.
- Since blockchains maintain immutable records and there is no point in having records like updates or edits, RPCs are more suitable option as they are quite generic and entertain GET and POST operations olnly.

- How to use Blockchain RPCs and choose the right provider?
  - We generally refer to servers with the term node.
  - In order to run blockcahin RPCs, you need a node.
  - Considerations
    - They should offer end to end support so you do not have to switch thrid parties multiple times while connecting to your node
    - They should have multople load-balanced nodes deployed in different regions to offer low latency
    - You should be able to retrieve the complete data of the node indexed properly so it can be queried whenever required
    - The provider shoukd be following an internationally recofnized data protection law
    - Real time alerts on node health
    - Archive data should be included for queries

## Solana CLI

    - Allow us to deploy smart contract to devnet, an actual blockchain run by real miners

## Metaplex CLI

    - Allow us to interact with Metaplex's deployed NFT contracts. Using their smart-contracts-as-a service we can
        -1: create our own candy machine
        -2: upload our NFTs to our candy machine
        -3: Allow users to actually hit our candy machine to mint an NFT
