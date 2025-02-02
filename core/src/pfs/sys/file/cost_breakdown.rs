use core::sync::atomic::AtomicU64;
use core::time::Duration;
use crate::os::Mutex;
use crate::prelude::*;

pub struct CostBreakdown {
    pub encrypt_cost: AtomicU64,
    pub decrypt_cost: AtomicU64,
    pub mht_cost: AtomicU64,
    pub io_cost: AtomicU64,
    pub write_num: AtomicU64,
    pub read_num: AtomicU64,
}

impl CostBreakdown {
    pub fn print_cost(&self) {
        warn!("encrypt_cost: {:?}", self.encrypt_cost);
        warn!("decrypt_cost: {:?}", self.decrypt_cost);
        warn!("mht_cost: {:?}", self.mht_cost);
        warn!("io_cost: {:?}", self.io_cost);
        warn!("write_num: {:?}", self.write_num);
        warn!("read_num: {:?}", self.read_num);
    }

    pub fn reset(&self) {
        self.encrypt_cost.store(0, core::sync::atomic::Ordering::Relaxed);
        self.decrypt_cost.store(0, core::sync::atomic::Ordering::Relaxed);
        self.mht_cost.store(0, core::sync::atomic::Ordering::Relaxed);
        self.io_cost.store(0, core::sync::atomic::Ordering::Relaxed);
        self.write_num.store(0, core::sync::atomic::Ordering::Relaxed);
        self.read_num.store(0, core::sync::atomic::Ordering::Relaxed);
    }
}

lazy_static::lazy_static! {
    pub static ref COST_BREAKDOWN: CostBreakdown = CostBreakdown {
        encrypt_cost: AtomicU64::new(0),
        decrypt_cost: AtomicU64::new(0),
        mht_cost: AtomicU64::new(0),
        io_cost: AtomicU64::new(0),
        write_num: AtomicU64::new(0),
        read_num: AtomicU64::new(0),
    };
}

