K.A.R.T.S. DApp
Khurts Automated Retail Trust System - Blockchain-Based Social Commerce Escrow

Project Description
K.A.R.T.S. is a decentralized escrow smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure, immutable payment gateway for independent live-sellers and student resellers operating on platforms like Facebook, Instagram, and TikTok. The contract ensures that buyer funds are locked safely on-chain before items are shipped, eliminating the risk of "joy reservers" and wasted motorcycle courier fees without relying on centralized payment middlemen.

The system allows buyers to lock USDC or PHPC stablecoins into a time-locked vault, and sellers to claim those funds once a 48-hour delivery countdown concludes. Every transaction is uniquely identified and stored within the contract's instance storage, ensuring absolute financial reliability.

Project Vision
Our vision is to revolutionize peer-to-peer social commerce in the digital age by:

Eliminating Transaction Fraud: Protecting micro-sellers from bogus buyers and protecting buyers from unverified shipping claims.

Securing Logistics Cash-Flow: Guaranteeing that independent sellers are not left paying out-of-pocket for motorcycle delivery dispatches on canceled orders.

Guaranteeing Immutability: Providing a permanent, tamper-proof record of checkout invoices that cannot be altered or deleted.

Enhancing Financial Inclusion: Leveraging Stellar's negligible fees to make smart-contract escrow economically viable for low-margin thrift and secondhand items.

Building Trustless Systems: Creating a local digital economy where execution is guaranteed by code, not by verbal promises.

We envision a future where digital peer-to-peer commerce is entirely trustless, empowering everyday individuals with complete autonomy over their retail transactions.

Key Features
1. Secure Invoice Creation
Create locked escrow invoices with a single function call.

Specify exact stablecoin amounts and a customized time-lock delivery window.

Automated ID generation for unique order tracking.

Persistent funding storage on the Stellar blockchain.

2. Automated Time-Locks
Funds remain strictly locked during the transit duration.

Enforces a strict 48-hour (or custom) countdown based on the blockchain's ledger timestamp.

Protects buyers from premature fund extraction.

3. Guaranteed Payouts
Sellers can extract payments instantly once the delivery window safely expires.

Clean and efficient storage management (clears the invoice upon settlement).

Immediate, trustless transfer of USDC/PHPC directly to the seller's wallet.

4. Transparency and Security
View all escrow initializations and payouts on the public blockchain.

Cryptographic verification to ensure only authorized sellers can claim the funds.

Protected against unauthorized modifications or third-party interceptions.

5. Stellar Network Integration
Leverages the high speed and near-zero cost of the Stellar network.

Built using the modern Soroban Smart Contract SDK in Rust.

Scalable architecture ready to handle thousands of live-selling checkouts.

Contract Details
Contract Address: [INSERT_YOUR_DEPLOYED_CONTRACT_ID_HERE]

Future Scope
Short-Term Enhancements
Frontend Web App: A mobile-first UI allowing sellers to generate quick "Checkout Links" or QR codes during live streams.

Multi-Currency Support: Allow buyers to lock in varying Stellar-based local fiat tokens (like PHPC).

Buyer Confirmation: Add an early-release function so buyers can manually unlock funds the moment they receive the package, bypassing the 48-hour wait.

Medium-Term Development
Motorcycle Delivery Integration: Direct off-chain bridge integration with local motorcycle ride-hailing and delivery dispatch APIs to automatically trigger the smart contract countdown exactly when the rider picks up the package.

Dispute Resolution Oracles: Implement multi-signature requirements allowing a trusted campus organization or local arbiter to pause the timer if an item is reported missing.

Passkey Abstraction: Allow non-crypto native buyers to sign transactions using biometrics (FaceID) instead of seed phrases.

Long-Term Vision
Reputation Scoring: Issue soulbound tokens (SBTs) to wallets with a history of successful, scam-free settlements.

Cross-Border Commerce: Extend the escrow logic to handle international shipping windows and automated currency swaps via Stellar's built-in DEX.

DAO Governance: Community-driven protocol improvements to adjust default platform fees and dispute window limits.

Technical Requirements
Soroban SDK

Rust programming language

Stellar blockchain network (Testnet)

Getting Started
Deploy the smart contract to Stellar's Soroban network and interact with it using the two main functions:

secure_invoice() - Buyer locks the stablecoin payment and initiates the delivery countdown.

claim_payment() - Seller retrieves the locked funds once the delivery timestamp has safely expired.

K.A.R.T.S. DApp - Securing Social Commerce on the Blockchain