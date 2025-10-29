# PulseMarket - Real-Time Prediction Markets on Linera



PulseMarket is a decentralized prediction market platform built on Linera's innovative microchain architecture, enabling instant settlement and real-time trading of predictions with AI-native infrastructure.

---

## 🎯 Executive Summary

PulseMarket reimagines prediction markets for the real-time Web3 era. By leveraging Linera's microchain technology, we deliver sub-second finality, unlimited scalability, and AI-agent integration—creating the first prediction market platform truly built for the age of intelligent agents and high-frequency on-chain activity.

**Market Opportunity**: The global prediction markets industry is projected to reach $5B+ by 2028, with crypto-native markets leading growth.

**Our Edge**: We're building on Linera's real-time infrastructure, enabling use cases impossible on traditional blockchains.

---

## 💡 Business Logic

### The Problem

Current prediction market platforms suffer from critical limitations:

1. **Slow Settlement**: 12+ second block times make real-time trading impossible
2. **Congestion**: Popular markets create network bottlenecks and gas wars
3. **Poor Scalability**: Adding markets/users degrades performance
4. **No AI Integration**: Existing platforms can't support autonomous AI agents
5. **Bad UX**: Web3 feels clunky compared to Web2 alternatives

### Our Solution

PulseMarket leverages Linera's unique architecture to solve these problems:

#### 1. **Microchains = Unlimited Scalability**
- Each market gets its own lightweight chain
- Markets run in parallel—no global bottlenecks
- Performance stays constant as we add markets and users
- Can support millions of markets simultaneously

#### 2. **Sub-500ms Finality**
- Instant bet confirmation
- Real-time odds updates
- Enable high-frequency trading strategies
- Web2-like responsiveness

#### 3. **AI-Native Architecture** (Wave 2+)
- Native GraphQL support for AI agents
- MCP-to-GraphQL proxy integration
- AI agents can autonomously create, trade, and resolve markets
- Enable automated market-making and strategy execution

#### 4. **Fair & Transparent Payouts**
- On-chain calculation of proportional winnings
- No house edge (market creates its own liquidity)
- Winners split the losing pool proportionally
- All transactions verifiable on-chain

### Revenue Model (Future)

**Phase 1**: Growth (Free to use)
- Focus on user acquisition and market liquidity
- Build network effects

**Phase 2**: Platform Fees
- 1-2% fee on winning payouts
- Creator fees for popular markets
- Premium features for power users

**Phase 3**: B2B Services
- White-label solutions for enterprises
- Custom market creation APIs
- AI agent infrastructure services

---

## 🚀 Go-to-Market Strategy

### Wave 1-2: Build & Validate (Current)

**Objectives**:
- ✅ Ship core prediction market functionality
- ✅ Demonstrate Linera's real-time capabilities
- 🔄 Win Linera Buildathon recognition
- 🔄 Get initial user feedback

**Tactics**:
- Submit to Linera Buildathon (all 6 waves)
- Document architecture and use cases
- Create demo videos showing <500ms settlement
- Engage with Linera community

### Wave 3-4: Alpha Launch (Q1 2026)

**Objectives**:
- Launch testnet with 10-50 early users
- Deploy 20+ test markets
- Integrate AI agent support (MCP/GraphQL)
- Build basic web frontend

**Tactics**:
- Crypto Twitter marketing campaign
- Partner with crypto prediction enthusiasts
- Host "prediction parties" for live events
- Showcase AI agents trading autonomously

**Target Markets**:
- Crypto price predictions (BTC/ETH)
- Sports betting enthusiasts  
- Crypto community governance outcomes
- Tech product launches (e.g., "Will Apple Vision Pro hit 1M sales?")

### Wave 5-6: Beta & Mainnet (Q2 2026)

**Objectives**:
- Launch on Linera mainnet
- Achieve 1,000+ active users
- Process $100K+ in prediction volume
- Attract AI agent developers

**Tactics**:
- ETH Denver 2026 Demo Day presentation
- Integrate with popular DeFi wallets
- Launch referral program
- Partner with AI agent platforms
- Media outreach to crypto publications

**Key Metrics**:
- Daily Active Users (DAU)
- Total Value Locked (TVL)
- Markets Created
- AI Agent Activity
- Transaction Volume

### Post-Mainnet: Scale (2026-2027)

**Objectives**:
- 10,000+ DAU
- $10M+ monthly volume
- Become the #1 real-time prediction market
- Launch mobile app

**Tactics**:
- Multi-chain expansion (if Linera supports bridges)
- Sports betting partnerships
- Enterprise B2B offerings
- Influencer partnerships
- Major exchange listings (for governance token)

---

## 🎮 How It Works

### Business Model Flow

```
1. User creates a market → Pays small creation fee (future)
2. Users place bets → Liquidity pools up
3. Market resolves → Winners claim proportional payouts
4. Platform takes 1-2% fee → Sustainable revenue (future)
```

### Technical Flow

```
Market Creation → Bet Placement → Resolution → Payout
       ↓              ↓              ↓          ↓
  Own Microchain | Cross-chain | Owner/Oracle | Proportional
  Instant Deploy | Token Transfer| Verification| Calculation
```

### Payout Formula

```
Winner's Payout = Original Bet + (Losing Pool × Your Bet / Total Winning Pool)

Example:
- You bet 10 tokens on "Yes"
- Total "Yes" bets: 100 tokens
- Total "No" bets: 200 tokens
- "Yes" wins!
- Your payout = 10 + (200 × 10/100) = 10 + 20 = 30 tokens
- You 3x your money! 🎉
```

---

## 🏗️ Technical Architecture

### Core Components

**1. Smart Contract ([src/contract.rs](src/contract.rs))**
- Market lifecycle management
- Bet placement logic
- Cross-chain token transfers
- Resolution mechanics
- Payout calculations

**2. State Management ([src/state.rs](src/state.rs))**
- Separate pools for Yes/No outcomes
- User bet tracking
- Market status tracking
- Metadata storage

**3. GraphQL API ([src/service.rs](src/service.rs))**
- Query market data
- Execute mutations
- Real-time subscriptions (future)

**4. ABI Definitions ([src/lib.rs](src/lib.rs))**
- Type definitions
- Operation enums
- Message passing structures

### Key Features

#### ✅ Wave 1 (Completed)
- [x] Binary prediction markets (Yes/No)
- [x] Fungible token integration
- [x] Cross-chain bet placement
- [x] Market resolution by owner
- [x] Proportional payout distribution
- [x] Cancellation & refund mechanism
- [x] Owner authentication
- [x] GraphQL query interface

#### 🚀 Wave 2-6 (Roadmap)
- [ ] **AI Integration**: MCP/GraphQL proxy for AI agents
- [ ] **Frontend**: React web app with real-time updates
- [ ] **Multi-Outcome**: Support for >2 outcomes
- [ ] **AMM Liquidity**: Automated market makers
- [ ] **Oracle Resolution**: TEE or voting-based resolution
- [ ] **Market Discovery**: Browse and search markets
- [ ] **Copy Trading**: Follow successful predictors
- [ ] **Mobile App**: iOS/Android apps
- [ ] **Analytics Dashboard**: Performance metrics
- [ ] **Social Features**: Comments, leaderboards

---

## 🛠️ Building & Usage

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Linera CLI
cargo install linera-service --git https://github.com/linera-io/linera-protocol

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Build

```bash
# Compile contracts
cargo build --release --target wasm32-unknown-unknown

# Check compilation
cargo check
```

### Deploy on Linera

```bash
# 1. Setup Linera wallet
linera wallet init --faucet https://faucet.testnet.linera.net

# 2. Create fungible token for betting
FUN_APP_ID=$(linera project publish-and-create examples/fungible \
  --json-argument '{ "accounts": { "'$OWNER'": "1000" } }' \
  --json-parameters '{ "ticker_symbol": "PRED" }')

# 3. Deploy PulseMarket
MARKET_ID=$(linera project publish-and-create pulse-market \
  --json-argument '{
    "owner": "'$OWNER'",
    "deadline": 4102473600000000,
    "question": "Will Bitcoin reach $100k by end of 2026?",
    "description": "Market resolves Yes if BTC >= $100k on Dec 31, 2026"
  }' \
  --json-parameters "'$FUN_APP_ID'" \
  --required-application-ids $FUN_APP_ID)

# 4. Start the service
linera service --port 8080
```

### Usage via GraphQL

Navigate to `http://localhost:8080/chains/$CHAIN_ID/applications/$MARKET_ID`

**Place a bet on Yes:**
```graphql
mutation {
  placeBet(
    owner: "$OWNER",
    outcome: Yes,
    amount: "10"
  )
}
```

**Query market state:**
```graphql
query {
  instantiationArgument { get { question deadline } }
  totalYes { get }
  totalNo { get }
  status { get }
}
```

**Resolve market (owner only):**
```graphql
mutation {
  resolveMarket(winningOutcome: Yes)
}
```

**Claim winnings:**
```graphql
mutation {
  claimWinnings(owner: "$OWNER")
}
```

---

## 🎯 Why Linera?

### Microchains = Game Changer

Traditional blockchains have a single chain processing all transactions sequentially. Linera assigns each user/app their own lightweight chain that runs in parallel.

**For PulseMarket, this means:**
- Each market = independent microchain
- Markets don't compete for resources
- Infinite horizontal scaling
- Consistent performance regardless of load

### Real-Time = Essential

Prediction markets need instant feedback:
- Place bet → See confirmation in <500ms
- Odds update → Reflect immediately
- Market resolves → Payouts instant

Linera's <500ms finality makes this possible.

### AI-Native = Future-Proof

(Wave 2+) Linera's GraphQL-first design means:
- AI agents can interact via standardized APIs
- No custom integration needed
- MCP protocol support out of the box
- AI-powered market creation and trading

---

## 📊 Competitive Analysis

| Platform | Finality | Scalability | AI Support | Status |
|----------|----------|-------------|------------|--------|
| **PulseMarket** | <500ms | Unlimited | Native | Building |
| Polymarket | ~2s | Limited | None | Live |
| Augur | ~15s | Limited | None | Live |
| Gnosis | ~12s | Limited | None | Live |
| Azuro | ~2s | Limited | None | Live |

**Our Advantage**: Only real-time platform built for AI agents and unlimited scale.

---

## 👥 Team

**Akindo Labs**
- Builders focused on real-time Web3 applications
- Early adopters of Linera's microchain technology
- Passionate about prediction markets and AI

---

## 📅 Roadmap

### Q4 2025 (Wave 1-2)
- ✅ Core prediction market smart contracts
- ✅ Binary outcome support
- 🔄 Buildathon submission

### Q1 2026 (Wave 3-4)
- AI agent integration (MCP/GraphQL)
- Basic web frontend
- Testnet deployment
- Alpha user testing

### Q2 2026 (Wave 5-6)
- ETH Denver 2026 Demo Day
- Mainnet launch
- Mobile-responsive design
- Public beta

### Q3-Q4 2026
- Multi-outcome markets
- AMM liquidity pools
- Oracle integration
- Mobile apps

### 2027
- Enterprise partnerships
- International expansion
- Governance token launch
- Cross-chain bridges






# Pulse-chain
# pulse-market
