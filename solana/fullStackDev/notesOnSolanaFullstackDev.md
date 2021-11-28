## Notes on Fullstack development on Solana

### Stack

- Frontend: React
- Backend: Rust/typescript faciliated by Anchor
- Documentation:
  - https://docs.phantom.app/integrating/establishing-a-connection#eagerly-connecting

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
    - running the program with ```anchor test```

### Solana program

    - A basic program (lib.rs)
    ```rust
    use anchor_lang::prelude::*;

    declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

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
