use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(
            RefCell::new(
                Node {
                    value: value,
                    next: None,
                    prev: None,
                }
            )
        )
    }
}


pub struct ListIterator {
    current: Link,
}

impl ListIterator {
    fn new(start_at: Link) -> ListIterator {
        ListIterator {
            current: start_at,
        }
    }
}

impl Iterator for ListIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None,
        };
        result
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            },
            None => None,
        };
        result
    }
}


#[derive(Clone)]
pub struct BetterTransactionLog {
    head: Link,
    tail: Link,
    pub length: usize
}

impl BetterTransactionLog {
    pub fn new() -> BetterTransactionLog {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => {
                old.borrow_mut().next = Some(new.clone());
                new.borrow_mut().prev = Some(old);
            }
            None => self.head = Some(new.clone()),
        };

        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;

            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }

    pub fn back_iter(self) -> ListIterator {
        ListIterator::new(self.tail)
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator::new(self.head.clone())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction_log() {
        let log = BetterTransactionLog::new();

        assert_eq!(log.length, 0);
        assert!(log.head.is_none());
        assert!(log.tail.is_none());
    }

    #[test]
    fn test_append_transaction_log() {
        let mut log = BetterTransactionLog::new();
        log.append(String::from("Transaction 1"));

        assert_eq!(log.length, 1);
        assert!(log.head.is_some());
        assert!(log.tail.is_some());

        log.append(String::from("Transaction 2"));
        assert_eq!(log.length, 2);

        assert_eq!(log.head.as_ref().unwrap().borrow().value, "Transaction 1");
        assert_eq!(log.tail.as_ref().unwrap().borrow().value, "Transaction 2");
    }

    #[test]
    fn test_pop() {
        let mut log = BetterTransactionLog::new();

        log.append(String::from("Transaction 1"));
        log.append(String::from("Transaction 2"));
        log.append(String::from("Transaction 3"));

        assert_eq!(log.head.as_ref().unwrap().borrow().value, "Transaction 1");
        assert_eq!(log.tail.as_ref().unwrap().borrow().value, "Transaction 3");

        assert_eq!(log.pop(), Some(String::from("Transaction 1")));
        assert_eq!(log.head.as_ref().unwrap().borrow().value, "Transaction 2");
        assert_eq!(log.tail.as_ref().unwrap().borrow().value, "Transaction 3");
        assert_eq!(log.length, 2);

        assert_eq!(log.pop(), Some(String::from("Transaction 2")));
        assert_eq!(log.head.as_ref().unwrap().borrow().value, "Transaction 3");
        assert_eq!(log.tail.as_ref().unwrap().borrow().value, "Transaction 3");
        assert_eq!(log.length, 1);

        assert_eq!(log.pop(), Some(String::from("Transaction 3")));
        assert_eq!(log.length, 0);
    }
}