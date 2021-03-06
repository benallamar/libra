// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::FuzzTargetImpl;
use libra_mempool::fuzzing::{
    mempool_incoming_transactions_strategy, test_mempool_process_incoming_transactions_impl,
};
use libra_proptest_helpers::ValueGenerator;
use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::{self, TestRunner},
};
use rand::RngCore;

#[derive(Debug, Default)]
pub struct MempoolIncomingTransactions;

impl FuzzTargetImpl for MempoolIncomingTransactions {
    fn name(&self) -> &'static str {
        module_name!()
    }

    fn description(&self) -> &'static str {
        "Transactions submitted to mempool"
    }

    fn generate(&self, _idx: usize, _gen: &mut ValueGenerator) -> Option<Vec<u8>> {
        let mut output = vec![0u8; 4096];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut output);
        Some(output)
    }

    fn fuzz(&self, data: &[u8]) {
        let passthrough_rng =
            test_runner::TestRng::from_seed(test_runner::RngAlgorithm::PassThrough, &data);

        let config = test_runner::Config::default();
        let mut runner = TestRunner::new_with_rng(config, passthrough_rng);

        let strategy = mempool_incoming_transactions_strategy();
        let strategy_tree = match strategy.new_tree(&mut runner) {
            Ok(x) => x,
            Err(_) => return,
        };
        let data = strategy_tree.current();

        test_mempool_process_incoming_transactions_impl(data);
    }
}
