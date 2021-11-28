## Notes on Fullstack development on Solana

### Stack

- Frontend: React
- Backend: Rust/typescript faciliated by Anchor
- Documentation:
  - https://docs.phantom.app/integrating/establishing-a-connection#eagerly-connecting
  - https://stevedonovan.github.io/rust-gentle-intro/4-modules.html
  - https://doc.rust-lang.org/std/result/
  - https://docs.solana.com/developing/runtime-facilities/programs#system-program

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
