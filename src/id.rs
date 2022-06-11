use std::time::{SystemTime, UNIX_EPOCH};

use rand::RngCore;

const EPOCH: u64 = 1420070400000;

pub struct Id {
    internal_worker:  u128,
    internal_process: u128,
    increment:        u128,
}

impl Id {
    pub fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    pub fn generate(&mut self) -> u64 {
        let mut id = Id::get_timestamp();
        id  -= &EPOCH;
        id <<= 5;
        id  += (self.internal_worker % 32) as u64;
        id <<= 5;
        id  += (self.internal_process % 32) as u64;
        id <<= 12;
        id += (self.increment % 4096) as u64;

        self.internal_process += 1;
        self.increment        += 1;
        self.internal_worker  += 1;

        id
    }

    pub fn created_at(id: u64) -> u64 {
        ((id >> 22) + EPOCH) as u64
    }

    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            internal_worker:  rng.next_u64() as u128,
            internal_process: rng.next_u64() as u128,
            increment:        rng.next_u64() as u128,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const AMOUNT: usize = 10;
    #[test]
    fn does_not_repeat() {
        let mut generator = Id::new();
        let mut ids: [u64; AMOUNT] = [0; AMOUNT];
        for i in 0..AMOUNT {
            let id= generator.generate();
            assert!(!ids.contains(&id));
            ids[i] = id;
            assert!(ids.contains(&id));
        }
    }

    #[test]
    fn created_at() {
        let mut generator = Id::new();
        let id = generator.generate();
        assert_eq!(Id::get_timestamp(), Id::created_at(id))
    }
}