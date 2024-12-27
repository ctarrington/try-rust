use std::cell::RefCell;
use std::rc::Rc;

pub trait Observer<T> {
    fn update(&mut self, new_value: T);
}

pub trait Subject<T> {
    fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer<T>>>);
    fn notify_observers(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct NumberHolder {
        value: i32,
        observers: Vec<Rc<RefCell<dyn Observer<i32>>>>,
    }

    impl NumberHolder {
        fn update(&mut self, new_value: i32) {
            self.value = new_value;
            self.notify_observers();
        }
    }

    impl Subject<i32> for NumberHolder {
        fn add_observer(&mut self, observer: Rc<RefCell<dyn Observer<i32>>>) {
            self.observers.push(observer);
        }

        fn notify_observers(&mut self) {
            for observer in &mut self.observers {
                observer.borrow_mut().update(self.value);
            }
        }
    }

    struct NumberObserver {
        value: i32,
    }

    impl NumberObserver {
        pub fn get_value(&self) -> i32 {
            self.value
        }
    }

    impl Observer<i32> for NumberObserver {
        fn update(&mut self, new_value: i32) {
            self.value = new_value;
        }
    }

    #[test]
    fn just_number() {
        let mut holder = NumberHolder {
            value: 0,
            observers: Vec::new(),
        };
        let observer1 = Rc::new(RefCell::new(NumberObserver { value: 0 }));
        let observer2 = Rc::new(RefCell::new(NumberObserver { value: 0 }));

        holder.add_observer(observer1.clone());
        holder.add_observer(observer2.clone());

        holder.update(10);
        assert_eq!(observer1.borrow().get_value(), 10);
        assert_eq!(observer2.borrow().get_value(), 10);

        holder.update(20);
        assert_eq!(observer1.borrow().get_value(), 20);
        assert_eq!(observer2.borrow().get_value(), 20);
    }
}
