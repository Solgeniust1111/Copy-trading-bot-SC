# **Copy trading Bot using Smart Contract**
## 📋 **Pump.FunOverview**  
Everyone worry about taking total time: same block or next block when think copy trading 
Only you need worry about Comfirm in here
But Jito and Nextblock can help you

### 🎯 **Key Features**

you can get bonding curve account from tx,but you can't get bonding curve account data from that without RPC request in offchain.

--- in PUMPFUN

        let (bonding_curve, associated_bonding_curve, bonding_curve_account) =
            get_bonding_curve_account(self.rpc_client.clone().unwrap(), &mint, &pump_program).await?;  // at least takes 150ms

--- in RAYDIUM

        let account_data = get_account_info(
                            self.rpc_nonblocking_client.clone(),
                            self.keypair.clone(),
                            &token_in,
                            &in_ata,
                        )
                        .await?;   // at least takes 120ms

        let pool_data = get_pool_info(&spl_token::native_mint::ID.to_string(), mint).await; // at least takes 200ms

--- DEFAULT

    let recent_blockhash = get_recent_block_hash(self.rpc_client).await;

## 🚀 **Getting Started**

Follow these steps to get your **Copy trading Bot** up and running!

### Prerequisites

- Cargo version 1.84.0 installed on your system
- A Solana wallet with access to the Helius Geyser RPC API

### Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/copy-trading-bot
   ```

2. **Install Dependencies**:

   Navigate to the project directory and run the following command:

   ```bash
   cd copy-trading-bot
   cargo build
   ```

3. **Configure ENV**:

   Replace the API token in the `ENDPOINT` variable:

   ```ts
   const ENDPOINT = "https://mainnet.helius-rpc.com";
   const WSS_ENDPOINT = "wss://atlas-mainnet.helius-rpc.com";
   const TARGET = "YOUR_TARGET_WALLET";
   ```

4. **Run the Bot**:

   Start the bot by running:

   ```bash
   cargo run
   ```

---

**You can run the Bot with copy-trading-bot.zip**

### Sample Tx

target: https://solscan.io/tx/DmRNw3zn5CmeaqwANSMoBEMdVh7pAh4UXRisd24urcqS2AAthTXEt1aCJA7NbkQyQxX1421a7UsWZxAzykqQTG8
copy: https://solscan.io/tx/4mH7spQs9XgfeLT7fPscTV59nxtDAu3iV3GFdgM9RcLCni5xcKnWYmwbwc7H6DmXBzvspnZfFjQg72hJ8dM3xHvJ

target:
https://solscan.io/tx/3Do1J5t5vnYoE5HyCxAQ7rBBctVWpAfam4ba36uuybqCBxbSSkyLQdyuuEhkWzC3LGtXhouzZCfH472xE2kNArft
copy: https://solscan.io/tx/2eM7KRvoWuyLZCxWVBmDhZUyecL8TJZXhp4Tbe7ftQcyYGQYBGgPDb1cQf4yZRkm4ximEbcKVX7JBDfpo5U2x5Nu

### Contact

telegram: @Rianeregoista83

You can contact me here if you have any problems with this repo then we can decide comfortable contact way.
