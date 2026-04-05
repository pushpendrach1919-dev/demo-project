🏦 Micro-Saving Club
The first truly permissionless micro-savings protocol on Stellar.

Micro-Saving Club is a decentralized application (dApp) built on the Stellar Network using Soroban smart contracts. It allows any user to create a savings goal, invite others to contribute, and lock funds in a secure, code-governed escrow until a specific financial milestone is reached.

🚀 Core Philosophy: Permissionless by Design
Unlike traditional savings groups or centralized apps, this dApp has no admins.

Anyone can create a club: No "approved" list of creators.

Anyone can contribute: No "invite-only" gatekeeping at the contract level.

Immutable Rules: Once a club is created with a goal and a target asset, the rules cannot be changed.

Math-Based Escrow: Funds are released to the creator only when the mathematical goal is met.

🛠 Features
Multi-Asset Support: Create savings circles for USDC, XLM, or any Stellar-wrapped asset.

Automated Escrow: Logic-gated withdrawals ensure funds are only moved when the goal is reached.

State Persistence: Built-in TTL (Time To Live) management to ensure your club data stays active on the Stellar ledger.

Zero Fees: The protocol takes 0%—only network gas fees apply.

🏗 Project Structure
Plaintext
.
├── contracts/
│   └── micro_saving/
│       ├── src/
│       │   └── lib.rs       # The Soroban Smart Contract logic
│       └── Cargo.toml       # Rust dependencies
├── public/                  # Frontend assets (Logo, Favicon)
├── src/
│   ├── components/          # React components (Navbar, Hero)
│   └── lib/                 # Stellar/Freighter wallet integration
└── README.md
🔧 Getting Started
1. Smart Contract (Soroban)
Prerequisites: Rust, Stellar CLI.

Bash
# Build the contract
stellar contract build

# Deploy to Testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/micro_saving.wasm \
  --source-account my-account \
  --network testnet
2. Frontend Integration
The UI is built with React and Tailwind CSS.

Bash
# Install dependencies
npm install

# Run the development server
npm run dev
Configuration:
Update your .env file with the deployed Contract ID:

Code snippet
NEXT_PUBLIC_CONTRACT_ID="C..."
NEXT_PUBLIC_NETWORK="testnet"
📝 Smart Contract API
create_club
Initializes a new savings pool.

club_id: A unique Symbol name for the club.

creator: The Address that will receive the funds once the goal is met.

goal: The total i128 amount required to unlock the escrow.

token_addr: The contract address of the Stellar Asset (e.g., USDC).

deposit
Permissionless function to add funds.

user: The address of the depositor.

amount: Amount to save.

club_id: The ID of the club to support.

withdraw
Executes the transfer of all pooled funds to the creator.

Requirement: current_balance >= goal.

🎨 UI Customization
To match the Micro-Saving Club brand, the following changes are applied:

Hero Text: "Permissionless Micro-Savings for Everyone. Start a savings circle, reach your goals, no managers required."

Navbar: Features a 🏦 SaveTogether logo and a real-time "Total Value Locked" tracker.

⚖️ License
This project is open-source and permissionless. Use it to build a more inclusive financial future.
id : CAXSAUR7KM27MJ7GMBMSM7X3XK57KW4GQFL6WQKEMJNUG4LYZUABAGF6
url :https://stellar.expert/explorer/testnet/contract/CAXSAUR7KM27MJ7GMBMSM7X3XK57KW4GQFL6WQKEMJNUG4LYZUABAGF6
<img width="1904" height="1028" alt="Screenshot 2026-04-05 161056" src="https://github.com/user-attachments/assets/5565ba7b-192a-4296-9d69-db81917ba3c7" />
