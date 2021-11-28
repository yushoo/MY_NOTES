## Notes on Fullstack development on Solana

### Stack

- Frontend: React
- Backend: Rust/typescript faciliated by Anchor
- Documentation:
  - https://docs.phantom.app/integrating/establishing-a-connection#eagerly-connecting

### Setup

- React Frontend

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
