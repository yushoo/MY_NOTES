## Notes on Fullstack development on Solana

### Stack

- Frontend: React
- Backend: Rust/typescript faciliated by Anchor

### Setup

- React Frontend
  - Check if wallet is connected when component is mounted
    ```
    /*
    * When our component first mounts, let's check to see if we have a connected
    * Phantom Wallet
    */
    useEffect(() => {
        const onLoad = async () => {
        await checkIfWalletIsConnected();
        };
        window.addEventListener('load', onLoad);
        return () => window.removeEventListener('load', onLoad)
    },[])
    ```
