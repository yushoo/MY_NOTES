# NFTS

- https://github.com/buildspace/buildspace-projects/blob/main/NFT_Collection/en/Section_1/Lesson_1_What_Is_A_NFT.md

# General notes minting NFTS

- Goal: Learn how to let users connect their wallet, click mint, and receive an NFT from out collection in their wallet.

### Metalplex

- Metaplex is the NFT standard on Solana and has created a set of standardized tools and libraries to create NFTs.
- On Ethereum, to create an NFT what we'do do is create our own custom OpenZeppelin ERC-721 NFT contract and deploy that. Tehn when we want to mint an NFT, we just call the mint function on our custom contract.
- Using Metaplex is very differet. With Metaplex we don't need to write our own contract. Metaplex has already deployed its own standard NFT contracts that any dev an interact with and build their own NFT collections on.
- Smart contract as a service. Why do this? This is mostly because Solana allows for parallet transactions. So, your code needs to account for cases like "uf 5 people go to mint an NFT at the same time and htere are only 2 left, who get's it?"
- Resource to create mint contract: https://github.com/metaplex-foundation/metaplex/blob/master/rust/nft-candy-machine/src/lib.rs
- In Ethereum it's easy. It's all synchronous and atomix so we don't need to think about that. But, part of Solana's sell is that it can do parallel transactions which makes it faster. But, this makes the code more complex. So, tools like Metaplex are extremely usefull. They handle the edge cases for us and give us a smart contract we can interact with.

### Candy Machine

- A candy machine is what Metaplex class a basic NFT drop where users can come in, click mint, and get an NFT.
- To fix the said issues about minting NFTS in parallet computing is to use something called a mutex along with an atomix transaction which are both devently complex to implement.
  - Mutex: https://doc.rust-lang.org/std/sync/struct.Mutex.html
    - A mutual exclusion primitive useful for protecting shared data.
    - This mutex will block threads waiting for the lock to become available.
    - Uses lock and try_lock, which guarantees that the data is only ever accessed when the mutex is locked.
  - Atomic transaction: https://en.wikipedia.org/wiki/Atomicity_(database_systems)
    - Either it all happens or it doesn't. No partiallity.

### Starter Code

- https://github.com/buildspace/nft-drop-starter-project

### Process

- Set up the frontend
  - If a wallet exist, it should inject a solana object into the window. From there we can connect the wallet to the app. If the user already added the app as a trusted app, then it should auto connect using the useEffect on component mount. Else the user should press the connect button that should render if user hasn't added the app.
  - Be sure to store the publicKey for future use using useState
  ```javascript
  const response = await solana.connect({ onlyIfTrusted: true });
  const response = await solana.connect();
  ```
- Get local slana env running

  - The goal is to create a web app that lets users connect their wallet, click mint, and receive an NFT from our collection in their wallet.
  - Download metaplex

- Set up NFT assets

  - Metaplex CLI offers a really simple way to tell your candy machine what NFTs you have available, for what price and more. An NFT is a JSON payload that has some asset attached to it.
  - Set up an assets folder in the same directory as app
  - In the assets folder, we are going to have a pair of files associated to each other. The nft asset (ie image) and a json file with the metadata for that specific NFT that Metaplex needs to set everything up on our behalf.

  ```
  // NFT #1
  0.png
  0.json

  // NFT #2
  1.png
  1.json

  // NFT #3
  2.png
  2.json

  ```

- Deploy NFTs to Solana's Devnet

  - 3 stage process
  - 1. Upload NFT to Arweave (A decentralized file-store) and initialize your candy machine's config
    - https://www.arweave.org/
  - 2. Create your candy machine on Metaplex's contract
  - 3. Update your candy machine with proper drop date for NFTs - this is the date when people can actually start minting your NFTs

  - Setting up a Solana keypair
    - In order to get uploading, we need to set up a local solana keypair.
    - In order for us to upload the NFTs to Solana, we need to have a "local wallet" to work with in the command line.
