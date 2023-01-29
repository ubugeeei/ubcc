use std::sync::{Arc, Mutex, Once};

pub fn rand() -> u64 {
    let rng = get_instance();
    let x = rng.inner.lock().unwrap().rand();
    x
}

#[derive(Clone)]
struct SingletonRand {
    inner: Arc<Mutex<XORShift>>,
}

fn get_instance() -> Box<SingletonRand> {
    static mut SINGLETON: Option<Box<SingletonRand>> = None;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            let singleton = SingletonRand {
                inner: Arc::new(Mutex::new(XORShift::new())),
            };

            SINGLETON = Some(Box::new(singleton));
        });

        SINGLETON.clone().unwrap()
    }
}

struct XORShift {
    seed: u64,
}

impl XORShift {
    fn new() -> XORShift {
        XORShift {
            seed: 0xf0fb588ca2196dac,
        }
    }

    fn rand(&mut self) -> u64 {
        self.next()
    }

    fn next(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 7);
        self.seed = self.seed ^ (self.seed << 17);
        self.seed
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rand() {
        for _ in 0..100 {
            assert!(rand() != rand());
        }
    }
}
