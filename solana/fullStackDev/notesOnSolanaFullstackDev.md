## Notes on Fullstack development on Solana

### Stack

- Frontend: React
- Backend: Rust/typescript faciliated by Anchor
- Documentation:
  - https://docs.phantom.app/integrating/establishing-a-connection#eagerly-connecting
  - https://stevedonovan.github.io/rust-gentle-intro/4-modules.html
  - https://doc.rust-lang.org/std/result/
  - https://docs.solana.com/developing/runtime-facilities/programs#system-program
  - https://solana-labs.github.io/solana-web3.js/modules.html#Commitment

### Frontend Setup

- React Frontend

  - Imports:

    ```javascript
    import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";
    import { Program, Provider, web3 } from "@project-serum/anchor";
    ```

  - Check if wallet is connected when component is mounted

    ```javascript
    const checkIfWalletIsConnected = async () => {
      try {
        const { solana } = window;
        // Check if Phantom Wallet extension has injected the solana object
        if (solana) {
          if (solana.isPhantom) {
            console.log("Phantom wallet found!");
            /*
             * The solana object gives us a function that will allow us to connect
             * directly with the user's wallet!
             */
            const response = await solana.connect({ onlyIfTrusted: true });
            console.log(
              "Connected with Public Key:",
              response.publicKey.toString()
            );

            /*
             * Set the user's publicKey in state to be used later!
             */
            setWalletAddress(response.publicKey.toString());
          }
        } else {
          alert("Solana object not found! Get a Phantom Wallet 👻");
        }
      } catch {
        console.error(error);
      }
    };

    /*
     * When our component first mounts, let's check to see if we have a connected
     * Phantom Wallet
     */
    useEffect(() => {
      const onLoad = async () => {
        await checkIfWalletIsConnected();
      };
      window.addEventListener("load", onLoad);
      return () => window.removeEventListener("load", onLoad);
    }, []);
    ```

  - Add a basic connect wallet button

    ```javascript
    const connectWallet = async () => {
      const { solana } = window;

      if (solana) {
        const response = await solana.connect();
        console.log(
          "Connected with Public Key:",
          response.publicKey.toString()
        );
        setWalletAddress(response.publicKey.toString());
      }
    };

    const renderNotConnectedContainer = () => (
      <button
        className="cta-button connect-wallet-button"
        onClick={connectWallet}
      >
        Connect to Wallet
      </button>
    );
    ```

### Solana env

- Setup

  - Download rust & solana
  - Verify

    ```
    rustup --version
    rustc --version
    cargo --version

    solana --version
    ```

    - Setup test env

    ```
    solana config set --url localhost
    solana config get
    ```

    - Check if local sonala node running is capable

    ```
    solana-test-validator
    ```

    - Install OpenSSL library

    ```
    brew install openssl@1.1
    ```

    - Install node, npm, and mocha

    ```
    npm install -g mocha
    ```

    - Install Anchor

    ```
    cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
    npm install @project-serum/anchor @solana/web3.js
    ```

  - Create project with anchor

  ```
  anchor init myepicproject --javascript
  cd myepicproject
  ```

  - Create a local keypair

  ```
  solana-keygen new
  ```

        - Creates a local solana keypair which acts like our local wallet we'll use to toalk to our programs via the command line.
        - Run ```solana config get``` to see something called ```keypair path```. That's where the wallet has been created
        - Run ```solana address``` to see the wallet's public address

  - Recap for initial Setup
    - Anchor init to create a basic Solana program
      - Compile our program
      - Spin up solana-test-validator and deploy the program to our local Solana network w/ our wallet. This is kinda like deploying our local server w/ new code.
      - Actually call functions on our deployed program. This is kinda like hitting a specific route on our server to test that it's working.
  - running the program with `anchor test`

### Solana program

- A basic program (lib.rs)

  ```rust
  // import
  use anchor_lang::prelude::*;
  // program id which has info for solana on how to run ur program
  declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
  // #[program]: everything in this module below is our program that we want to create handlers for that otherpeople can call
  #[program]
  pub mod myepicproject {
      use super::\*;
      pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
         Ok(())
      }
  }

  #[derive(Accounts)]
  pub struct StartStuffOff {}
  ```

- pub mod: tells us that this is a Rust "module" which is an easy way to define a collection of functions and variables - kinda like a class.

- Write a script
  - Located in tests/asdf.js

```javascript
const anchor = require("@project-serum/anchor");

// Need the system program, will talk about this soon.
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("🚀 Starting test...");

  // Create and set the provider. We set it before but we needed to update it, so that it can communicate with our frontend!
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;

  // Create an account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();

  // Call start_stuff_off, pass it the params it needs!
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("📝 Your transaction signature", tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("👀 GIF Count", account.totalGifs.toString());
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
```

- Setup cycle

  - Write code in lib.rs
  - Test specific functions using tests/asdf.js

- Adding functionality to your program

```rust
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
```

- Initializing an account
  - tells our program what kind of account it can make and what to hold inside of it

```rust
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}
```

- Specify how to initialize and hold context

```rust
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}
```

### Adding functions to program

- Add a context which has access toa mutable reference

```rust
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}
```

- Adding pb function

```rust
pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult {
    // Get a reference to the account and increment total_gifs.
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs += 1;
    Ok(())
}
```

- Call function in script

```javascript
// Call add_gif!
await program.rpc.addGif({
  accounts: {
    baseAccount: baseAccount.publicKey,
  },
});

// Get the account again to see what changed.
account = await program.account.baseAccount.fetch(baseAccount.publicKey);
console.log("👀 GIF Count", account.totalGifs.toString());
```

### Adding vectors

- Declare struct

```rust
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}
```

- Attach vector to account

```rust
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
```

- Add to vector via script function call in js file

```javascript
// You'll need to now pass a GIF link to the function! You'll also need to pass in the user submitting the GIF!
await program.rpc.addGif("insert_a_giphy_link_here", {
  accounts: {
    baseAccount: baseAccount.publicKey,
    user: provider.wallet.publicKey,
  },
});
```

### Setting up environment for devnet

- Switch to devnet

```
solana config set --url devnet
```

- After that run

```
solana config get
```

- airdrop sol to devnet

```
solana airdrop 5
```

- Check balance

```
solana balance
```

- Changing up some vairables

```
In Anchor.toml, change [programs.localnet] to [programs.devnet].
Then, change cluster = "localnet" to cluster = "devnet".
```

- Now run

```
anchor build
```

- Access the prigram id by running

```
solana address -k target/deploy/myepicproject-keypair.json
```

- Grab program is and swap it out accordingly with old program id in lib.rs

```
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
```

- The program id specifies how to load and execute the program and contains info on how the Solana runtime should execute the program
- The program id also helps the Solana runtime see all the accounts created by the program iself. With the ID Solana can see all the accounts generated by the program and easily reference them.
- Now, go to Anchor.toml and under [programs.devnet] you'll see something like myepicproject = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS". Go ahead and change this id to the same id output when you run solana address -k target/deploy/myepicproject-keypair.json.
- Now run anchor build

- Running on devnet summary

```
solana config set --url devnet

// Make sure you're on devnet.
solana config get

anchor build

// Get the new program id.
solana address -k target/deploy/myepicproject-keypair.json

// Update Anchor.toml and lib.rs w/ new program id.
// Make sure Anchor.toml is on devnet.

// Build again.
anchor build
```

- To deploy run anchor deploy

### Setting up frontend to connect to program

- Hook up IDL file to web app

  - idl file is actually just a JSON file that has some info about our Solana program like the names of our functions and the parameters they accept. This helps our web app actually know how to interact with our deployed program.
  - Copy all conent in target/idl/myepicproject.json
  - Head over to web app
  - past content of target/idl/myepicproject.json as idl.json
  - import idl from './idl.json'; in app

- install packages

```
npm install @project-serum/anchor @solana/web3.js
```

- Import packages

```javascript
import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";
import { Program, Provider, web3 } from "@project-serum/anchor";
```

- Get provider: Create a provider which is an authenticated connection to Solana. To make a provider we need a connected wallet.

```javascript
const getProvider = () => {
  const connection = new Connection(network, opts.preflightCommitment);
  const provider = new Provider(
    connection,
    window.solana,
    opts.preflightCommitment
  );
  return provider;
};
```

- Variables required

```javascript
// SystemProgram is a reference to the Solana runtime!
const { SystemProgram, Keypair } = web3;

// Create a keypair for the account that will hold the GIF data.
let baseAccount = Keypair.generate();

// Get our program's id from the IDL file.
const programID = new PublicKey(idl.metadata.address);

// Set our network to devnet.
const network = clusterApiUrl("devnet");

// Controls how we want to acknowledge when a transaction is "done".
const opts = {
  preflightCommitment: "processed",
};
```

- SystemProgram is the a reference to the core program that runs Solana we already talked about. Keypair.generate() gives us some parameters we need to create the BaseAccount account that will hold the GIF data for our program.
- Then, we use idl.metadata.address to get our program's id and then we specify that we want to make sure we connect to devnet by doing clusterApiUrl('devnet').
- This preflightCommitment: "processed" thing is interesting. Basically, we can actually choose when to receive a confirmation for when our transaction has succeeded. Because the blockchain is fully decentralized, we can choose how long we want to wait for a transaction. Do we want to wait for just one node to acknowledge our transaction? Do we want to wait for the whole Solana chain to acknowledge our transaction?
- In this case, we simply wait for our transaction to be confirmed by the node we're connected to. This is generally okay — but if you wanna be super super sure you may use something like "finalized" instead. For now, let's roll with "processed".

- Retrieve data from program account

```javascript
const getGifList = async () => {
  try {
    const provider = getProvider();
    const program = new Program(idl, programID, provider);
    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );

    console.log("Got the account", account);
    setGifList(account.gifList);
  } catch (error) {
    console.log("Error in getGifList: ", error);
    setGifList(null);
  }
};

useEffect(() => {
  if (walletAddress) {
    console.log("Fetching GIF list...");
    getGifList();
  }
}, [walletAddress]);
```

- Initialize program from frontend

```javascript
const createGifAccount = async () => {
  try {
    const provider = getProvider();
    const program = new Program(idl, programID, provider);
    console.log("ping");
    await program.rpc.startStuffOff({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    console.log(
      "Created a new BaseAccount w/ address:",
      baseAccount.publicKey.toString()
    );
    await getGifList();
  } catch (error) {
    console.log("Error creating BaseAccount account:", error);
  }
};
```

- submitting data to our program

```javascript
const sendGif = async () => {
  if (inputValue.length === 0) {
    console.log("No gif link given!");
    return;
  }
  console.log("Gif link:", inputValue);
  try {
    const provider = getProvider();
    const program = new Program(idl, programID, provider);

    await program.rpc.addGif(inputValue, {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });
    console.log("GIF successfully sent to program", inputValue);

    await getGifList();
  } catch (error) {
    console.log("Error sending GIF:", error);
  }
};
```

- Persist the baseaccount info when setting up

  - Under the src directory, go ahead and create a file named createKeyPair.js

    ```javascript
    const fs = require("fs");
    const anchor = require("@project-serum/anchor");

    const account = anchor.web3.Keypair.generate();

    fs.writeFileSync("./keypair.json", JSON.stringify(account));
    ```

  - Then run node createKeyPair.js
  - add import

  ```javascript
  import kp from "./keypair.json";
  ```

- Next, delete let baseAccount = Keypair.generate();. Instead, we'll replace it with this:

```javascript
const arr = Object.values(kp._keypair.secretKey);
const secret = new Uint8Array(arr);
const baseAccount = web3.Keypair.fromSecretKey(secret);
```
