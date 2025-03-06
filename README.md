# **Copy trading Bot using Smart Contract**
## 📋 **Pump.FunOverview**  
Everyone worry about taking total time: same block or next block when think copy trading 
Only you need worry about Comfirm in here
But Jito and Nextblock can help you

### 🎯 **Key Features**
#### **Original Bot**
you can get bonding curve account from tx,but you can't get bonding curve account data from that without RPC request in original bot.

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

#### Here

you never need anything like RPC request.
you can get all data from account in onchain(Smart contract) directly

## 🚀 **Getting Started**

Follow these steps to get your **Copy trading Bot ** up and running!

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
   anchor build
   anchor deploy
   ```

3. **Run the Bot**:

   Start the bot by running:
   this bot is https://github.com/Solgeniust1111/solana-copy-trading-bot
   this bot has integrated with this Smart contract

   ```bash
   cargo run
   ```
---

### Contact

telegram: @Rianeregoista83

You can contact me here if you have any problems with this repo then we can decide comfortable contact way.
