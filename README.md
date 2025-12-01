# Bull or Bear - Solana Prediction Game

A decentralized prediction game built on Solana where users bet on BTC/USD price movements using Pyth Network price feeds.

**🎲 Play Now**: [https://bullorbear.bet/](https://bullorbear.bet/)

## 🎮 Overview

Bull or Bear is a prediction game where users bet on whether the BTC/USD price will go up (Bull) or down (Bear) within fixed time intervals. The game operates in rounds, with each round having distinct lock and close phases.

## 🏗️ Architecture

### Core Components

#### Config Account
Global configuration managing game parameters:
- **Admin/Operator**: Authorized roles for game management
- **Interval**: Round duration (default: 30 seconds)
- **Min Bet Amount**: Minimum bet size (default: 0.01 SOL)
- **Treasury Fee**: Platform fee percentage (max: 10%)
- **Treasury Tracking**: `treasury_amount` (total funds), `pending_bet_amount` (locked in active bets)
- **Game State**: `locked_once`, `paused`, `current_epoch`, `last_available_epoch`

#### Round Account
Individual round data:
- **Timestamps**: `start_ts`, `lock_ts`, `close_ts`
- **Prices**: `lock_price`, `close_price` (with exponents) - nullable until set
- **Amounts**: `total_amount`, `bull_amount`, `bear_amount`
- **Bet Counts**: `bull_total_bets`, `bear_total_bets`
- **Rewards**: `reward_base`, `reward_amount` (calculated after close)
- **State**: `unresolved_bets_count` (pending claims)

#### Bet Account
User bet information:
- **User**: Bettor's public key
- **Epoch**: Round identifier
- **Position**: `Bull` or `Bear`
- **Amount**: Bet size in lamports

### PDAs (Program Derived Addresses)

- **Config**: `["config"]`
- **Treasury**: `["treasury"]`
- **Round**: `["round", epoch_bytes]`
- **Bet**: `["bet", user_pubkey, epoch_bytes]`

## 📋 Instructions

### Admin Instructions

#### `config_initialize`
Initialize game configuration with admin, operator, and game parameters.

**Authorization:**
- Requires upgrade authority via ProgramData account
- Only the program's upgrade authority can initialize config

**Parameters:**
- `interval: u16` - Round duration in seconds
- `min_bet_amount: u64` - Minimum bet in lamports
- `treasury_fee: u16` - Fee in basis points (e.g., 500 = 5%)
- `admin: Pubkey` - Admin public key (cannot be default pubkey)
- `operator: Pubkey` - Operator public key (cannot be default pubkey)

**Validation:**
- `treasury_fee <= MAX_TREASURY_FEE` (1000 = 10%)
- `admin != Pubkey::default() && operator != Pubkey::default()`

#### `config_update`
Update game configuration (admin only).

**Parameters:** All optional
- `admin: Option<Pubkey>`
- `operator: Option<Pubkey>`
- `interval_seconds: Option<u16>`
- `min_bet_amount: Option<u64>`
- `treasury_fee: Option<u16>` (max: 1000 = 10%)

#### `initialize`
Start the game by creating the first two rounds.
- Reuses existing config values if set, otherwise uses defaults
- Creates round for current epoch and next epoch
- Must be called before any betting can occur

#### `pause` / `unpause`
Pause or unpause the game (admin/operator only).
- When paused, betting and round initialization are disabled
- Existing rounds can still be closed and bets claimed

#### `withdraw_treasury`
Withdraw accumulated fees from treasury (admin only).

**Parameters:**
- `amount: u64` - Amount to withdraw in lamports

**Constraints:**
- Can only withdraw profits: `treasury_amount - pending_bet_amount`
- Ensures funds locked in active bets remain untouchable
- Prevents withdrawal of user funds before bet resolution

#### `close_config`
Close config account and recover rent (admin only).

#### `close_round`
Close a round account and recover rent (admin/operator only).
- Requires all bets to be resolved (`unresolved_bets_count == 0`)
- Does NOT transfer lamports - only validates state
- Actual account closure happens via Anchor's `close` constraint

**Parameters:**
- `epoch: u64` - Round epoch to close

### Game Instructions

#### `round_lock`
Lock a round with the current BTC/USD price from Pyth.
- Validates price timestamp matches round `lock_ts` exactly
- Sets `lock_price` and `lock_price_exponent`
- Creates next future round
- Updates `locked_once = true`, `current_epoch`, and `last_available_epoch`
- Emits `RoundStarted` event

**Requires:**
- Game not paused
- Pyth price update account with publish_time == lock_ts (exact match)
- Admin or operator authorization

#### `round_execute`
Execute round closure and start the next round.
- Validates price timestamp matches round `close_ts` exactly
- Sets `close_price` and `close_price_exponent` on current round
- Calculates rewards for winners
- Sets `lock_price` on next round (using same price)
- Creates new future round
- Updates `current_epoch` and `last_available_epoch`
- Emits `RoundClosed` and `RoundStarted` events

**Requires:**
- Pyth price update account with publish_time == close_ts (exact match)
- Admin or operator authorization

#### `round_add_future`
Manually add a future round (admin/operator only).
- Useful for maintaining round buffer
- Ensures betting can continue without interruption

### User Instructions

#### `bet`
Place a bet on a round.

**Parameters:**
- `epoch: u64` - Target round epoch
- `position: Position` - `Bull` or `Bear`
- `amount: u64` - Bet amount in lamports

**Constraints:**
- Game not paused
- Before lock time (with 3-second cutoff)
- Amount >= minimum bet + rent
- Bet account must not exist for user+epoch

**Flow:**
1. Validates game not paused and betting window open
2. Checks bet amount meets minimum and covers rent
3. Transfers amount from user to treasury
4. Creates bet PDA with treasury as payer (manual account creation)
5. Validates account creation (owner, size, discriminator)
6. Initializes bet struct with validation
7. Updates round totals (`total_amount`, `bull_amount`/`bear_amount`)
8. Updates bet counts (`bull_total_bets`/`bear_total_bets`)
9. Increments `unresolved_bets_count`
10. Updates `treasury_amount` (total funds)
11. Updates `pending_bet_amount` (locked funds)
12. Serializes bet data with verification
13. Emits `BetEvent`

#### `close_bet`
Close a bet and transfer rewards to the user (anyone can call).

**Authorization:**
- No restrictions - any wallet can call this instruction
- Rewards are always sent to the wallet that placed the bet (stored in bet PDA)
- Commonly called by admin/operator backend for batch processing
- Users can also claim their own bets directly

**Reward Calculation:**
- **Winner**: `(bet_amount * reward_amount) / reward_base`
- **Loser**: `0`
- **Draw**: `0` (house wins)
- **Unresolved**: `bet_amount` (full refund)

**Flow:**
1. Validates round is resolved
2. Calculates reward based on outcome
3. Updates `treasury_amount` (deduct reward if > 0)
4. Releases bet from `pending_bet_amount`
5. Decrements `unresolved_bets_count`
6. Transfers reward from treasury to user (if > 0)
7. Emits `Claim` event

**Note:** Accounting updates happen before transfer to ensure consistency

#### `bet_refund`
Request refund for bet on unresolved round.

**Requirements:**
- Round not fully resolved (`lock_price` or `close_price` is `None`)
- 30 minutes passed since `close_ts`
- Any user can call (for any bet)

**Flow:**
1. Refunds full bet amount to bettor
2. Deducts refund from `treasury_amount`
3. Releases bet from `pending_bet_amount`
4. Reverts round state (amounts and counts)
5. Decrements `unresolved_bets_count`

## 🔒 Security Features

### Authorization

- **Macros**: `require_admin!`, `require_admin_or_operator!`
- **Roles**:
  - Admin: Full control (config, treasury, withdrawals)
  - Operator: Game operations (rounds, pause/unpause)
  - Users: Betting only

### Validations

- **Betting Cutoff**: 3-second buffer before round lock
- **Price Age**: Pyth prices must be recent (< 30 minutes via MAXIMUM_AGE)
- **Price Timestamp**: Price publish_time must match round lock_ts or close_ts exactly (no tolerance for provable fairness)
- **Pause State**: Prevents betting and initialization when paused
- **Refund Delay**: 30-minute wait before refund available
- **Fee Limit**: Treasury fee capped at 10%

### Treasury Accounting

- **Dual Tracking**: Separates total funds from locked bet amounts
- **treasury_amount**: Total SOL in treasury PDA
- **pending_bet_amount**: SOL locked in unresolved bets
- **Withdrawable**: `treasury_amount - pending_bet_amount` (profits only)
- **Protection**: Admin cannot withdraw user funds before bet resolution
- **Real-time Updates**: Both amounts updated on bet placement, claim, and refund

### Price Feeds

- **Source**: Pyth Network BTC/USD feed
- **Integration**: PythSolanaReceiver for on-chain verification
- **Timestamp Validation**: Price publish_time must match round lock_ts or close_ts exactly (required for provable fairness)
- **No Tolerance**: Exact match ensures deterministic, verifiable outcomes
- **Staleness Check**: Prices must be no older than 30 minutes (MAXIMUM_AGE)
- **Nullable Prices**: Explicit `Option<i64>` instead of sentinel values
- **Exponents**: Stored alongside prices for accurate calculations
- **Security**: Operator cannot create fake prices - only Pyth can publish PriceUpdateV2 accounts

## 💰 Economics

### Fee Structure

- **Treasury Fee**: Configurable (default: 5%, max: 10%)
- **Collection**: Fees retained when winners claim (difference between bet amount and reward)
- **Accounting**: 
  - `treasury_amount`: Tracks all funds (bets + fees)
  - `pending_bet_amount`: Tracks locked bet amounts
  - Withdrawable profit = `treasury_amount - pending_bet_amount`

### Reward Calculation

```rust
fee_amount = total_amount * treasury_fee / 10000
reward_base = winning_side_amount
reward_amount = total_amount - fee_amount

// Per winner:
reward = (bet_amount * reward_amount) / reward_base
```

### Accounting Example

**User bets 1 SOL:**
- `treasury_amount += 1 SOL`
- `pending_bet_amount += 1 SOL`
- Withdrawable = 0 SOL ✓

**Winner claims 1.8 SOL reward:**
- `treasury_amount -= 1.8 SOL`
- `pending_bet_amount -= 1 SOL` (original bet released)
- Net change: +0.2 SOL profit remains

**Loser's bet closed (0 reward):**
- `treasury_amount` unchanged
- `pending_bet_amount -= 1 SOL` (bet released)
- Net change: +1 SOL profit available

**Result:** Admin can only withdraw actual profits, never locked bet funds.

### Edge Cases

- **No Winners** (draw): House keeps all (treasury fee only)
- **One-Sided Rounds**: Losing side redistributed to winners
- **Unresolved Rounds**: Full refunds after 30 minutes

## 🚀 Deployment

### Build
```bash
anchor build
```

### Deploy
```bash
anchor deploy --provider.cluster devnet
```

### Initialize
```bash
# 1. Create config
anchor run config-initialize --provider.cluster devnet

# 2. Start game
anchor run initialize --provider.cluster devnet
```

## 🧪 Testing

```bash
anchor test
```

## 📊 Events

### Initialized
Game started with first rounds
- `interval: u16` - Round duration
- `epoch: u64` - Starting epoch
- `lock_ts: i64` - First round lock timestamp

### RoundInitialized
New round created
- `epoch: u64` - Round epoch
- `start_ts: i64` - Round start timestamp
- `lock_ts: i64` - Round lock timestamp
- `close_ts: i64` - Round close timestamp
- `key: Pubkey` - Round account address

### RoundStarted
Round locked with price
- `key: Pubkey` - Round account address
- `epoch: u64` - Round epoch
- `close_ts: i64` - Round close timestamp
- `lock_price: i64` - BTC/USD price at lock
- `lock_price_exponent: i32` - Price exponent

### RoundClosed
Round closed with final price
- `key: Pubkey` - Round account address
- `epoch: u64` - Round epoch
- `close_ts: i64` - Round close timestamp
- `close_price: i64` - BTC/USD price at close
- `close_price_exponent: i32` - Price exponent

### BetEvent
User placed bet
- `epoch: u64` - Round epoch
- `position: Position` - Bull or Bear
- `bet_amount: u64` - Amount bet
- `total_amount: u64` - Round total amount
- `bull_amount: u64` - Round bull amount
- `bear_amount: u64` - Round bear amount
- `bull_total_bets: u32` - Round bull bet count
- `bear_total_bets: u32` - Round bear bet count
- `user: Pubkey` - Bettor address

### RewardsCalculated
Rewards computed for round
- `epoch: u64` - Round epoch
- `reward_base: u64` - Total winning bets
- `reward_amount: u64` - Total rewards to distribute
- `treasury_amount: u64` - Current treasury balance

### Claim
Bet rewards claimed
- `user: Pubkey` - Bettor address
- `epoch: u64` - Round epoch
- `amount: u64` - Reward amount
- `winning_position: Position` - Winning side
- `payout_ratio_bps: u64` - Payout ratio in basis points (10000 = 100%)

### Pause / Unpause
Game state changed
- `epoch: u64` - Current epoch

## 🛠️ Development

### Constants (`constants.rs`)

- `BETTING_CUTOFF`: 3 seconds before lock_ts
- `MAX_TREASURY_FEE`: 1000 (10%)
- `MAXIMUM_AGE`: 1800 seconds (30 minutes) - max price staleness
- `SECONDS_BEFORE_REFUND_AVAILABLE`: 1800 (30 minutes)
- `FEED_ID`: Pyth BTC/USD price feed ID
- `require_admin!`: Macro for admin-only operations (checks config.admin)
- `require_admin_or_operator!`: Macro for admin or operator operations (checks config.admin OR config.operator)

### Error Codes (`errors.rs`)

- `GenesisLockRoundOnce`: Can run genesis_lock_round only once
- `InvalidEpoch`: Invalid round epoch
- `RoundNotBettable`: Round not bettable
- `BetTooSmall`: Bet amount below minimum
- `AlreadyClaimed`: Bet already claimed
- `RoundNotEnded`: Round not ended
- `UnauthorizedOperator`: Unauthorized operator
- `Overflow`: Arithmetic overflow occurred
- `InvalidFee`: Invalid fee percentage
- `InvalidPriceFeed`: The price feed account is invalid or not BTC/USD
- `StalePrice`: The price feed is stale or not currently trading
- `MathOverflow`: Math overflow
- `BettingClosed`: Betting is closed for this round
- `UnresolvedBetsExist`: Unresolved bets exist for this round
- `InsufficientTreasuryFunds`: Insufficient treasury funds
- `GamePaused`: Game is paused
- `RefundNotYetAvailable`: Refund not yet available
- `PriceTimestampMismatch`: Price timestamp must match exactly (no tolerance)
- `InvalidAdminOrOperator`: Invalid admin or operator configuration
- `InvalidAccountOwner`: Invalid account owner
- `InvalidAccountSize`: Invalid account size
- `InsufficientAccountData`: Insufficient account data
- `BetSerializationFailed`: Bet serialization failed
- `BetVerificationFailed`: Bet verification failed
- `InvalidDiscriminator`: Invalid discriminator

### Utils (`utils.rs`)

- `require_not_paused(config)`: Validate game is not paused
- `get_next_full_minute()`: Round to next full minute for scheduling
- `initialize_round(round, epoch, interval_seconds, start_ts)`: Create new round and emit RoundInitialized event
- `get_price(price_update, expected_timestamp)`: Extract Pyth price and validate timestamp matches exactly (required for provable fairness)
- `update_round_and_bet(round, bet, user, position, amount, config)`: Process bet placement, update round amounts/counts, update treasury tracking (treasury_amount & pending_bet_amount), emit BetEvent
- `calculate_rewards(round, config)`: Compute winner payouts with dual fee modes (standard or loser-only), does NOT modify pending_bet_amount (tracks original bets only), emit RewardsCalculated event
- `validate_claim(ctx)`: Check round ended and close_price is set

## 📄 License

Copyright © 2025 Bull or Bear. All rights reserved.

This software is proprietary and confidential. No license is granted for use, modification, 
or distribution without explicit written permission from the copyright holder. See the 
[LICENSE](LICENSE) file for full terms.

## 🔗 Links

- **Pyth Network**: https://pyth.network/
- **Solana**: https://solana.com/
- **Anchor**: https://www.anchor-lang.com/
