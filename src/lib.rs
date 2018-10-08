use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct Defered<T> {
    defered: Arc<Mutex<T>>,
    f: Box<Fn() -> T>,
}

impl<T> Defered<T>
where
    T: Clone + Send + 'static,
{
    pub fn new<F>(f: F, time: time::Duration) -> Self
    where
        F: Fn() -> T + Clone + Send + 'static,
    {
        let defered = Arc::new(Mutex::new(f()));
        let c_mutex = defered.clone();
        let c_f = f.clone();
        thread::spawn(move || loop {
            {
                let mut lock = c_mutex.lock();
                if let Ok(ref mut mutex) = lock {
                    **mutex = c_f();
                }
            }
            thread::sleep(time);
        });

        Self { defered, f: Box::from(f) }
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
    extern crate rand;
    #[test]
    fn it_works() {
        use super::thread;
        use super::time;
        use super::Defered;
        let x = Defered::new(|| rand::random::<f64>(), time::Duration::from_millis(10));
        for _ in 0..3 {
            println!("{:?}", x.value());
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
