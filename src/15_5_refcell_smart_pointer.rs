pub trait Messenger {
  fn send(&self, msg: &str);
}

// This code simulate a library that check if the number of send messages has
// ultrapassed the max attribute.
pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max >= 0.9 {
      self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
    } else if percentage_of_max >= 0.75 {
      self.messenger.send("Warning: You've used up over 75% of your quota!");
    }
  }
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessages {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessages {
    fn new() -> MockMessages {
      MockMessages {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  // The code bellow takes a immutable reference and change its value in runtime.
  // It's be possible because its use a RefCell smart pointer, wich ables a immutable
  // reference change its value as it is mutable, but only in runtime.
  // To get reference to the pointer is used the method borrow and borrow_mut.
  impl Messenger for MockMessages {
    fn send(&self, message: &str) {
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessages::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}