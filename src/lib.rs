use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct Defered<T> {
    defered: Arc<Mutex<T>>,
    f: fn() -> T,
}

impl<T> Defered<T>
where
    T: Clone + std::marker::Send + 'static,
{
    pub fn new(f: fn() -> T, time: time::Duration) -> Self {
        let defered = Arc::new(Mutex::new(f()));
        let c_mutex = defered.clone();
        let c_time = time;
        thread::spawn(move || loop {
            {
                let mut lock = c_mutex.lock();
                if let Ok(ref mut mutex) = lock {
                    **mutex = f();
                }
            }
            thread::sleep(c_time);
        });

        Self { defered, f }
    }

    pub fn value(&self) -> T {
        if let Ok(val) = self.defered.clone().lock() {
            return (*val).clone();
        }
        (self.f)()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::thread;
        use super::time;
        use super::Defered;
        let x = Defered::new(|| 1, time::Duration::from_millis(10));
        loop {
            println!("{:?}", x.value());
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
