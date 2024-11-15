use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;
#[derive(Clone)]
struct Node {
    value: String,
    next: Link
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(
            RefCell::new(
                Node {
                    value,
                    next: None
                }
            )
        )
    }
}

struct TransactionLog {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    pub length: usize
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(old_tail) => old_tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone())
        };

        self.length += 1;
        self.tail = Some(new_node.clone());
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(old_head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction_log() {
        let log = TransactionLog::new_empty();

        assert_eq!(log.length, 0);
        assert!(log.head.is_none());
        assert!(log.tail.is_none());
    }

    #[test]
    fn test_single_append() {
        let mut log = TransactionLog::new_empty();
        log.append(String::from("Transaction 1"));

        assert_eq!(log.length, 1);
        assert!(log.head.is_some());
        assert!(log.tail.is_some());

        assert!(Rc::ptr_eq(
            &log.head.as_ref().unwrap(),
            &log.tail.as_ref().unwrap()
        ));
    }

    #[test]
    fn test_pop_single_elements() {
        let mut log = TransactionLog::new_empty();
        log.append(String::from("Transaction 1"));

        let popped = log.pop();

        assert_eq!(popped, Some(String::from("Transaction 1")));
        assert_eq!(log.length, 0);

        assert!(log.head.is_none());
        assert!(log.tail.is_none());
    }

    #[test]
    fn test_pop_multiple_elements() {
        let mut log = TransactionLog::new_empty();
        log.append(String::from("Transaction 1"));
        log.append(String::from("Transaction 2"));
        log.append(String::from("Transaction 3"));

        assert_eq!(log.pop(), Some(String::from("Transaction 1")));
        assert_eq!(log.length, 2);
        assert_eq!(log.pop(), Some(String::from("Transaction 2")));
        assert_eq!(log.length, 1);
        assert_eq!(log.pop(), Some(String::from("Transaction 3")));
        assert_eq!(log.length, 0);

        assert_eq!(log.pop(), None);
    }

    #[test]
    fn test_mixed_operations() {
        let mut log = TransactionLog::new_empty();

        log.append(String::from("Transaction 1"));
        log.append(String::from("Transaction 2"));
        assert_eq!(log.pop(), Some(String::from("Transaction 1")));

        log.append(String::from("Transaction 3"));
        assert_eq!(log.length, 2);

        assert_eq!(log.pop(), Some(String::from("Transaction 2")));
        assert_eq!(log.pop(), Some(String::from("Transaction 3")));
        assert_eq!(log.pop(), None);
    }
}