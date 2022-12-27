// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::config::MAX_APPLICATION_MESSAGE_SIZE;
use serde::{Deserialize, Serialize};

pub const DEFAULT_BROADCAST_BUCKETS: &[u64] =
    &[0, 150, 300, 500, 1000, 3000, 5000, 10000, 100000, 1000000];

//////// 0L //////// // comments only    
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct MempoolConfig {
    // the total size of the mempool queue, including invalid txs. 
    pub capacity: usize,
    // 
    pub capacity_bytes: usize,
    // How many txs can each user have in the mempool at a given time.
    pub capacity_per_user: usize,
    // a threshold for fullnodes to determine which peers to broadcast to.
    // peers which are go over this threshold, will receive broadcasts.
    // number of failovers to broadcast to when the primary network is alive
    pub default_failovers: usize,
    // number of times a mempool broadcast gets re-sent to a peer if
    // the previous was unacknowledged.
    pub max_broadcasts_per_peer: usize,
    // how often to snapshot the mempool for analytics purposes.
    pub mempool_snapshot_interval_secs: u64,
    // how long to wait for a peer after a broadcast was submitted,
    // before we mark it as unacknowledged.
    pub shared_mempool_ack_timeout_ms: u64,
    // if peer_manager is in backoff mode mempool/src/shared_mempool/peer_manager.rs
    // this is the base interval for backing off.
    pub shared_mempool_backoff_interval_ms: u64,
    // size of batch from mempool timeline to broadcast to peers.
    pub shared_mempool_batch_size: usize,
    //
    pub shared_mempool_max_batch_bytes: u64,
    // Number of workers to be spawned to receive inbound shared mempool broadcasts.
    pub shared_mempool_max_concurrent_inbound_syncs: usize,
    // the default interval to execute shared mempool broadcasts to peers.
    // this is overriden when peer is in backoff mode.
    pub shared_mempool_tick_interval_ms: u64,
    // when a transaction gets automatically garbage collected by system.
    // Different than user tx expiry which has separate GC
    pub system_transaction_timeout_secs: u64,
    // tick interval for system GC.
    pub system_transaction_gc_interval_ms: u64,
    //
    pub broadcast_buckets: Vec<u64>,
}

impl Default for MempoolConfig {
    fn default() -> MempoolConfig {
        MempoolConfig {
            shared_mempool_tick_interval_ms: 5_000, //////// 0L //////// 
            shared_mempool_backoff_interval_ms: 3_000, //////// 0L ////////
            shared_mempool_batch_size: 100,
            shared_mempool_max_batch_bytes: MAX_APPLICATION_MESSAGE_SIZE as u64,
            shared_mempool_ack_timeout_ms: 20_000, ///////// 0L /////////
            shared_mempool_max_concurrent_inbound_syncs: 10,  ///////// 0L /////////
            max_broadcasts_per_peer: 5, //////// 0L ////////
            mempool_snapshot_interval_secs: 180,
            capacity: 100, ///////// 0L //////// Reduce size of mempool due to VDF cost
            capacity_bytes: 2 * 1024 * 1024 * 1024,
            ///////// 0L ////////
            // no reason for a given user to be able to submit more than tree txs to mempool
            capacity_per_user: 1,
            //////// 0L //////// transactions should timeout under this time
            default_failovers: 3,
            system_transaction_timeout_secs: 1000,
            /////// 0L //////// increase rate of GC
            system_transaction_gc_interval_ms: 1000,
            broadcast_buckets: DEFAULT_BROADCAST_BUCKETS.to_vec(),
        }
    }
}
