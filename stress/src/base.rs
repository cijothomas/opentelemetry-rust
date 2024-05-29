/*
    Stress test results:
    OS: Ubuntu 22.04.3 LTS (5.15.146.1-microsoft-standard-WSL2)
    Hardware: AMD EPYC 7763 64-Core Processor - 2.44 GHz, 16vCPUs,
    RAM: 64.0 GB
    1.25 B/sec
*/

use std::sync::{atomic::AtomicU64, Mutex, RwLock};

use lazy_static::lazy_static;

mod throughput;

lazy_static! {
    static ref SUM_MUTEX: Mutex<u64> = Mutex::new(0);
    static ref SUM_RWLOCK: RwLock<AtomicU64> = RwLock::new(AtomicU64::new(0));
}

fn main() {
    throughput::test_throughput(rwlock);
    // throughput::test_throughput(mutex);
}

fn mutex() {
    let mut sum = SUM_MUTEX.lock().unwrap();
    *sum += 1;
}

fn rwlock() {
    let sum = SUM_RWLOCK.read().unwrap();
    sum.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}
