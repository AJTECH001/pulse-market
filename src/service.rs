// Copyright (c) Akindo Labs
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Request, Response, Schema};
use pulse_market::Operation;
use linera_sdk::{
    abis::fungible::FungibleTokenAbi,
    graphql::GraphQLMutationRoot as _,
    linera_base_types::{ApplicationId, WithServiceAbi},
    views::View,
    Service, ServiceRuntime,
};
use state::PulseMarketState;

pub struct PulseMarketService {
    state: Arc<PulseMarketState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(PulseMarketService);

impl WithServiceAbi for PulseMarketService {
    type Abi = pulse_market::PulseMarketAbi;
}

impl Service for PulseMarketService {
    type Parameters = ApplicationId<FungibleTokenAbi>;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = PulseMarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        PulseMarketService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.state.clone(),
            Operation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish();
        schema.execute(request).await
    }
}
