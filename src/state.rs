// Copyright (c) Akindo Labs
// SPDX-License-Identifier: Apache-2.0

use async_graphql::scalar;
use pulse_market::{InstantiationArgument, Outcome};
use linera_sdk::{
    linera_base_types::{AccountOwner, Amount},
    views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext},
};
use serde::{Deserialize, Serialize};

/// The status of a prediction market.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub enum MarketStatus {
    /// The market is active and can receive bets.
    #[default]
    Active,
    /// The market has been resolved with a winning outcome.
    Resolved(Outcome),
    /// The market was cancelled, all bets have been returned.
    Cancelled,
}

scalar!(MarketStatus);

/// A bet placed by a user on a specific outcome.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, async_graphql::SimpleObject)]
pub struct Bet {
    /// The outcome the user bet on.
    pub outcome: Outcome,
    /// The amount bet.
    pub amount: Amount,
}

/// The prediction market's state.
#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = ViewStorageContext)]
pub struct PulseMarketState {
    /// The status of the market.
    pub status: RegisterView<MarketStatus>,
    /// The map of bets on Yes outcome: user -> amount.
    pub yes_bets: MapView<AccountOwner, Amount>,
    /// The map of bets on No outcome: user -> amount.
    pub no_bets: MapView<AccountOwner, Amount>,
    /// The total amount bet on Yes.
    pub total_yes: RegisterView<Amount>,
    /// The total amount bet on No.
    pub total_no: RegisterView<Amount>,
    /// The instantiation data that determines the details of the market.
    pub instantiation_argument: RegisterView<Option<InstantiationArgument>>,
}

impl MarketStatus {
    /// Returns `true` if the market status is [`MarketStatus::Resolved`].
    pub fn is_resolved(&self) -> bool {
        matches!(self, MarketStatus::Resolved(_))
    }

    /// Returns `true` if the market status is [`MarketStatus::Cancelled`].
    pub fn is_cancelled(&self) -> bool {
        matches!(self, MarketStatus::Cancelled)
    }

    /// Returns `true` if the market is active.
    pub fn is_active(&self) -> bool {
        matches!(self, MarketStatus::Active)
    }
}
