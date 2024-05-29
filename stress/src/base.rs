/*
    Stress test results:
    OS: Ubuntu 22.04.3 LTS (5.15.146.1-microsoft-standard-WSL2)
    Hardware: AMD EPYC 7763 64-Core Processor - 2.44 GHz, 16vCPUs,
    RAM: 64.0 GB
    1.25 B/sec
*/

use lazy_static::lazy_static;
use opentelemetry::KeyValue;
use rand::{
    rngs::{self},
    Rng, SeedableRng,
};
use std::sync::{atomic::AtomicU64, Mutex, RwLock};
use std::{cell::RefCell, env};

mod throughput;

lazy_static! {
    static ref VALUE_MAP_MUTEX: Mutex<std::collections::HashMap<KeyValue, u64>> =
        Mutex::new(std::collections::HashMap::new());
    static ref VALUE_MAP_RWLOCK: RwLock<std::collections::HashMap<KeyValue, AtomicU64>> =
        RwLock::new(std::collections::HashMap::new());
    static ref VALUE_MAP_MUTEX_MMSC: Mutex<std::collections::HashMap<KeyValue, MMSC>> =
    Mutex::new(std::collections::HashMap::new());
    static ref VALUE_MAP_RWLOCK_MMSC: RwLock<std::collections::HashMap<KeyValue, Mutex<MMSC>>> =
    RwLock::new(std::collections::HashMap::new());
    static ref SIZE: i64 = 2000;
}

thread_local! {
    /// Store random number generator for each thread
    static CURRENT_RNG: RefCell<rngs::SmallRng> = RefCell::new(rngs::SmallRng::from_entropy());
}

struct MMSC {
    sum: u64,
    count: u64,
    min: u64,
    max: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "mutex" => {
                println!("Running mutex test");
                throughput::test_throughput(mutex)
            }
            "mutex_mmsc" => {
                println!("Running mutex_mmsc test");
                throughput::test_throughput(mutex_mmsc)
            }
            "rwlock" => {
                println!("Running rwlock test");
                throughput::test_throughput(rwlock)
            }
            "rwlock_mmsc" => {
                println!("Running rwlock_mmsc test");
                throughput::test_throughput(rwlock_mmsc)
            }

            _ => println!("Invalid argument. Please specify 'mutex' or 'rwlock'."),
        }
    } else {
        println!("No argument provided. Please specify 'mutex' or 'rwlock'.");
    }
}

fn mutex() {
    let rand = CURRENT_RNG.with(|rng| {
        rng.borrow_mut().gen_range(0..*SIZE)
    });

    let key = KeyValue::new("key", rand);
    if let Ok(mut map) = VALUE_MAP_MUTEX.lock() {
        if let Some(value) = map.get_mut(&key) {
            *value += 1;
        } else {
            map.insert(key.clone(), 1);
        }
    }
}

fn mutex_mmsc() {
    let rand = CURRENT_RNG.with(|rng| {
        rng.borrow_mut().gen_range(0..*SIZE)
    });

    let key = KeyValue::new("key", rand);
    if let Ok(mut map) = VALUE_MAP_MUTEX_MMSC.lock() {
        if let Some(value) = map.get_mut(&key) {
            value.count += 1;
            value.sum += 1;
            value.min = value.min.min(1);
            value.max = value.max.max(1);
        } else {
            map.insert(key.clone(), MMSC {
                sum: 1,
                count: 1,
                min: 1,
                max: 1,
            });
        }
    }
}

fn rwlock() {
    let rand = CURRENT_RNG.with(|rng| {
        rng.borrow_mut().gen_range(0..*SIZE)
    });

    let key = KeyValue::new("key", rand);
    if let Ok(map) = VALUE_MAP_RWLOCK.read() {
        if let Some(value) = map.get(&key) {
            value.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        } else {
            drop(map);
            let mut map = VALUE_MAP_RWLOCK.write().unwrap();
            map.insert(key.clone(), AtomicU64::new(1));
        }
    }
}

fn rwlock_mmsc() {
    let rand = CURRENT_RNG.with(|rng| {
        rng.borrow_mut().gen_range(0..*SIZE)
    });

    let key = KeyValue::new("key", rand);
    if let Ok(map) = VALUE_MAP_RWLOCK_MMSC.read() {
        if let Some(value) = map.get(&key) {
            let mut value = value.lock().unwrap();
            value.count += 1;
            value.sum += 1;
            value.min = value.min.min(1);
            value.max = value.max.max(1);
        } else {
            drop(map);
            let mut map = VALUE_MAP_RWLOCK_MMSC.write().unwrap();
            map.insert(key.clone(), Mutex::new(MMSC {
                sum: 1,
                count: 1,
                min: 1,
                max: 1,
            }));
        }
    }
}
