use core::sync::atomic::{AtomicU64, Ordering};
use core::time::Duration;
use crate::os::Mutex;
use crate::prelude::*;

pub struct CostBreakdown {
    pub encrypt_cost: AtomicU64,
    pub decrypt_cost: AtomicU64,
    pub buf_cost: AtomicU64,
    pub io_cost: AtomicU64,
}

impl CostBreakdown {
    pub fn print_cost(&self) {
        warn!("encrypt_cost: {:?}", self.encrypt_cost);
        warn!("decrypt_cost: {:?}", self.decrypt_cost);
        warn!("buf_cost: {:?}", self.buf_cost);
        warn!("io_cost: {:?}", self.io_cost);

        let total_cost = (self.encrypt_cost.load(Ordering::Relaxed) + 
        self.decrypt_cost.load(Ordering::Relaxed) + 
        self.buf_cost.load(Ordering::Relaxed) + 
        self.io_cost.load(Ordering::Relaxed) )as f64;
        warn!("encrypt_percent: {:?}", self.encrypt_cost.load(Ordering::Relaxed) as f64 / total_cost);
        warn!("decrypt_percent: {:?}", self.decrypt_cost.load(Ordering::Relaxed) as f64 / total_cost);
        warn!("buf_percent: {:?}", self.buf_cost.load(Ordering::Relaxed) as f64 / total_cost);
        warn!("io_percent: {:?}", self.io_cost.load(Ordering::Relaxed) as f64 / total_cost);
    }

    pub fn reset(&self) {
        self.encrypt_cost.store(0, core::sync::atomic::Ordering::Relaxed);
        self.decrypt_cost.store(0, core::sync::atomic::Ordering::Relaxed);
        self.buf_cost.store(0, core::sync::atomic::Ordering::Relaxed);
        self.io_cost.store(0, core::sync::atomic::Ordering::Relaxed);

    }
}

lazy_static::lazy_static! {
    pub static ref COST_BREAKDOWN: CostBreakdown = CostBreakdown {
        encrypt_cost: AtomicU64::new(0),
        decrypt_cost: AtomicU64::new(0),
        buf_cost: AtomicU64::new(0),
        io_cost: AtomicU64::new(0),
    };
}

