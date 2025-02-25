use std::{collections::HashMap, sync::{Arc, RwLock}};

use opentelemetry::KeyValue;

/// The storage for sums.
///
/// This structure is parametrized by an `Operation` that indicates how
/// updates to the underlying value trackers should be performed.
pub(crate) struct AggregationStore
{
    /// Trackers store the values associated with different attribute sets.
    trackers: RwLock<HashMap<Vec<KeyValue>, Arc<A>>>,

    /// Used ONLY by Delta collect. The data type must match the one used in
    /// `trackers` to allow mem::swap. Wrapping the type in `OnceLock` to
    /// avoid this allocation for Cumulative aggregation.
    trackers_for_collect: OnceLock<RwLock<HashMap<Vec<KeyValue>, Arc<A>>>>,

    /// Number of different attribute set stored in the `trackers` map.
    count: AtomicUsize,
    /// Indicates whether a value with no attributes has been stored.
    has_no_attribute_value: AtomicBool,
    /// Tracker for values with no attributes attached.
    no_attribute_tracker: A,
    /// Configuration for an Aggregator
    config: A::InitConfig,
}