// Copyright (c) Akindo Labs
// SPDX-License-Identifier: Apache-2.0

/*! ABI of the PulseMarket Prediction Market Application */

use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{AccountOwner, Amount, ContractAbi, ServiceAbi, Timestamp},
};
use serde::{Deserialize, Serialize};

pub struct PulseMarketAbi;

impl ContractAbi for PulseMarketAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for PulseMarketAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// The outcome of a binary prediction market.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash, async_graphql::Enum)]
pub enum Outcome {
    /// The "Yes" outcome
    Yes,
    /// The "No" outcome
    No,
}

/// The instantiation data required to create a prediction market.
#[derive(Clone, Debug, Deserialize, Serialize, SimpleObject)]
pub struct InstantiationArgument {
    /// The creator/owner of the market who can resolve it.
    pub owner: AccountOwner,
    /// The deadline after which the market can be resolved.
    pub deadline: Timestamp,
    /// The question being predicted.
    pub question: String,
    /// Optional description of the market.
    pub description: Option<String>,
}

impl std::fmt::Display for InstantiationArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("Serialization failed")
        )
    }
}

/// Operations that can be executed by the application.
#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    /// Place a bet on a specific outcome (from an account on the current chain to the market chain).
    PlaceBet {
        owner: AccountOwner,
        outcome: Outcome,
        amount: Amount,
    },
    /// Resolve the market with the winning outcome (market chain only, owner only).
    ResolveMarket { winning_outcome: Outcome },
    /// Claim winnings after the market has been resolved (for winners).
    ClaimWinnings { owner: AccountOwner },
    /// Cancel the market and refund all bets (market chain only, owner only, before deadline).
    CancelMarket,
}

/// Messages that can be exchanged across chains from the same application instance.
#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    /// Place a bet from an account on the receiver chain.
    PlaceBetWithAccount {
        owner: AccountOwner,
        outcome: Outcome,
        amount: Amount,
    },
}
