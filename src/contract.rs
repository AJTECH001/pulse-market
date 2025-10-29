// Copyright (c) Akindo Labs
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use pulse_market::{InstantiationArgument, Message, Operation, Outcome, PulseMarketAbi};
use linera_sdk::{
    abis::fungible::{FungibleOperation, FungibleTokenAbi},
    linera_base_types::{Account, AccountOwner, Amount, ApplicationId, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};
use state::{MarketStatus, PulseMarketState};

pub struct PulseMarketContract {
    state: PulseMarketState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(PulseMarketContract);

impl WithContractAbi for PulseMarketContract {
    type Abi = PulseMarketAbi;
}

impl Contract for PulseMarketContract {
    type Message = Message;
    type InstantiationArgument = InstantiationArgument;
    type Parameters = ApplicationId<FungibleTokenAbi>;
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = PulseMarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        PulseMarketContract { state, runtime }
    }

    async fn instantiate(&mut self, argument: InstantiationArgument) {
        // Validate that the application parameters were configured correctly.
        let _ = self.runtime.application_parameters();

        self.state.instantiation_argument.set(Some(argument));

        let deadline = self.instantiation_argument().deadline;
        assert!(
            deadline > self.runtime.system_time(),
            "Prediction market cannot start after its deadline"
        );

        // Initialize totals to zero
        self.state.total_yes.set(Amount::ZERO);
        self.state.total_no.set(Amount::ZERO);
    }

    async fn execute_operation(&mut self, operation: Operation) -> Self::Response {
        match operation {
            Operation::PlaceBet { owner, outcome, amount } => {
                if self.runtime.chain_id() == self.runtime.application_creator_chain_id() {
                    self.execute_bet_with_account(owner, outcome, amount).await;
                } else {
                    self.execute_bet_with_transfer(owner, outcome, amount);
                }
            }
            Operation::ResolveMarket { winning_outcome } => {
                self.resolve_market(winning_outcome);
            }
            Operation::ClaimWinnings { owner } => {
                self.claim_winnings(owner).await;
            }
            Operation::CancelMarket => {
                self.cancel_market().await;
            }
        }
    }

    async fn execute_message(&mut self, message: Message) {
        match message {
            Message::PlaceBetWithAccount { owner, outcome, amount } => {
                assert_eq!(
                    self.runtime.chain_id(),
                    self.runtime.application_creator_chain_id(),
                    "Action can only be executed on the chain that created the prediction market"
                );
                self.execute_bet_with_account(owner, outcome, amount).await;
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl PulseMarketContract {
    fn fungible_id(&mut self) -> ApplicationId<FungibleTokenAbi> {
        self.runtime.application_parameters()
    }

    /// Places a bet from a local account to the remote market chain.
    fn execute_bet_with_transfer(&mut self, owner: AccountOwner, outcome: Outcome, amount: Amount) {
        assert!(amount > Amount::ZERO, "Bet amount must be positive");
        assert!(
            self.state.status.get().is_active(),
            "Market is not active"
        );

        // The market chain.
        let chain_id = self.runtime.application_creator_chain_id();
        // First, move the funds to the market chain (under the same owner).
        let target_account = Account { chain_id, owner };
        let call = FungibleOperation::Transfer {
            owner,
            amount,
            target_account,
        };
        let fungible_id = self.fungible_id();
        self.runtime
            .call_application(/* authenticated by owner */ true, fungible_id, &call);
        // Second, schedule the bet to be recorded on the market chain.
        self.runtime
            .prepare_message(Message::PlaceBetWithAccount { owner, outcome, amount })
            .with_authentication()
            .send_to(chain_id);
    }

    /// Places a bet from a local account on the market chain.
    async fn execute_bet_with_account(&mut self, owner: AccountOwner, outcome: Outcome, amount: Amount) {
        assert!(amount > Amount::ZERO, "Bet amount must be positive");
        assert!(
            self.state.status.get().is_active(),
            "Market is not active"
        );

        self.receive_from_account(owner, amount);
        self.finish_bet(owner, outcome, amount).await;
    }

    /// Records a bet in the application state.
    async fn finish_bet(&mut self, owner: AccountOwner, outcome: Outcome, amount: Amount) {
        match outcome {
            Outcome::Yes => {
                self.state
                    .yes_bets
                    .get_mut_or_default(&owner)
                    .await
                    .expect("view access should not fail")
                    .saturating_add_assign(amount);
                let new_total = self.state.total_yes.get().saturating_add(amount);
                self.state.total_yes.set(new_total);
            }
            Outcome::No => {
                self.state
                    .no_bets
                    .get_mut_or_default(&owner)
                    .await
                    .expect("view access should not fail")
                    .saturating_add_assign(amount);
                let new_total = self.state.total_no.get().saturating_add(amount);
                self.state.total_no.set(new_total);
            }
        }
    }

    /// Resolves the market with the winning outcome.
    fn resolve_market(&mut self, winning_outcome: Outcome) {
        // Only the market owner can resolve
        let owner = self.instantiation_argument().owner;
        let authenticated = self.runtime.authenticated_owner();
        assert_eq!(
            authenticated,
            Some(owner),
            "Only the market owner can resolve the market"
        );

        assert!(
            self.state.status.get().is_active(),
            "Market is not active"
        );

        assert!(
            self.runtime.system_time() >= self.instantiation_argument().deadline,
            "Market deadline has not been reached yet"
        );

        self.state.status.set(MarketStatus::Resolved(winning_outcome));
    }

    /// Claims winnings for a user after the market has been resolved.
    async fn claim_winnings(&mut self, owner: AccountOwner) {
        let status = *self.state.status.get();

        let winning_outcome = match status {
            MarketStatus::Resolved(outcome) => outcome,
            _ => panic!("Market has not been resolved yet"),
        };

        let (winner_bet, total_winning_pool, total_losing_pool) = match winning_outcome {
            Outcome::Yes => {
                let bet = self.state.yes_bets.get(&owner).await
                    .expect("view access should not fail")
                    .unwrap_or(Amount::ZERO);
                (bet, *self.state.total_yes.get(), *self.state.total_no.get())
            }
            Outcome::No => {
                let bet = self.state.no_bets.get(&owner).await
                    .expect("view access should not fail")
                    .unwrap_or(Amount::ZERO);
                (bet, *self.state.total_no.get(), *self.state.total_yes.get())
            }
        };

        assert!(winner_bet > Amount::ZERO, "No winning bet found for this user");

        // Calculate winnings: original bet + proportional share of losing pool
        let winnings = if total_winning_pool > Amount::ZERO {
            // Calculate proportionally using u128 arithmetic
            let winner_bet_u128: u128 = winner_bet.into();
            let total_yes_u128: u128 = total_winning_pool.into();
            let total_no_u128: u128 = total_losing_pool.into();

            let share_of_losing_pool = total_no_u128
                .saturating_mul(winner_bet_u128)
                .saturating_div(total_yes_u128);

            // Total winnings = original bet + share of losing pool
            let total_winnings_u128 = winner_bet_u128.saturating_add(share_of_losing_pool);
            Amount::from_attos(total_winnings_u128)
        } else {
            winner_bet
        };

        // Transfer winnings to the user
        self.send_to(winnings, owner);

        // Remove the bet from the map to prevent double claiming
        match winning_outcome {
            Outcome::Yes => {
                self.state.yes_bets.remove(&owner).expect("Failed to remove bet");
            }
            Outcome::No => {
                self.state.no_bets.remove(&owner).expect("Failed to remove bet");
            }
        }
    }

    /// Cancels the market and refunds all bets.
    async fn cancel_market(&mut self) {
        // Only the market owner can cancel
        let owner = self.instantiation_argument().owner;
        let authenticated = self.runtime.authenticated_owner();
        assert_eq!(
            authenticated,
            Some(owner),
            "Only the market owner can cancel the market"
        );

        assert!(
            self.state.status.get().is_active(),
            "Market is not active"
        );

        // Refund all Yes bets
        let mut yes_bets = Vec::new();
        self.state
            .yes_bets
            .for_each_index_value(|bettor, amount| {
                yes_bets.push((bettor, amount.into_owned()));
                Ok(())
            })
            .await
            .expect("view iteration should not fail");
        for (bettor, amount) in yes_bets {
            self.send_to(amount, bettor);
        }

        // Refund all No bets
        let mut no_bets = Vec::new();
        self.state
            .no_bets
            .for_each_index_value(|bettor, amount| {
                no_bets.push((bettor, amount.into_owned()));
                Ok(())
            })
            .await
            .expect("view iteration should not fail");
        for (bettor, amount) in no_bets {
            self.send_to(amount, bettor);
        }

        self.state.yes_bets.clear();
        self.state.no_bets.clear();
        self.state.status.set(MarketStatus::Cancelled);
    }

    /// Transfers `amount` tokens from the funds in custody to the `owner`'s account.
    fn send_to(&mut self, amount: Amount, owner: AccountOwner) {
        let target_account = Account {
            chain_id: self.runtime.chain_id(),
            owner,
        };
        let transfer = FungibleOperation::Transfer {
            owner: self.runtime.application_id().into(),
            amount,
            target_account,
        };
        let fungible_id = self.fungible_id();
        self.runtime.call_application(true, fungible_id, &transfer);
    }

    /// Calls into the Fungible Token application to receive tokens from the given account.
    fn receive_from_account(&mut self, owner: AccountOwner, amount: Amount) {
        let target_account = Account {
            chain_id: self.runtime.chain_id(),
            owner: self.runtime.application_id().into(),
        };
        let transfer = FungibleOperation::Transfer {
            owner,
            amount,
            target_account,
        };
        let fungible_id = self.fungible_id();
        self.runtime.call_application(true, fungible_id, &transfer);
    }

    pub fn instantiation_argument(&self) -> &InstantiationArgument {
        self.state
            .instantiation_argument
            .get()
            .as_ref()
            .expect("Application is not running on the host chain or was not instantiated yet")
    }
}
