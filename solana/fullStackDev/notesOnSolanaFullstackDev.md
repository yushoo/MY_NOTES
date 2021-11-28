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
