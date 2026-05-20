use std::rc::Rc;
use std::cell::RefCell;

// Single Threaded Doubly Linked List
/* Requirements
 * O(1) insertion at front and back
 * O(1) deletion anywhere in the list
 * */

type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;

// reason for writing a constructor macro: we can impl on foreign types
macro_rules! node_ref {
    ($node:expr) => {
        Some(Rc::new(RefCell::new($node)))
    }
}

struct Node<T> {
    val: T,
    prev: NodeRef<T>,
    next: NodeRef<T>
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            prev: None, 
            next: None
        }
    } 
}


struct DoublyLL<T> {
    head: NodeRef<T>,
    tail: NodeRef<T>,
}

impl<T> DoublyLL<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_head = node_ref!(Node::new(val));

        self.head.take().map(|old_head| {
            // modify links
            new_head.as_ref().unwrap().borrow_mut().next = old_head.borrow().next.clone();
            old_head.borrow_mut().prev = new_head.clone(); 
        }); 
        
        self.head = new_head; 
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = node_ref!(Node::new(val));

        self.tail.take().map(|old_tail| {
            // modify links
            new_tail.as_ref().unwrap().borrow_mut().next = old_tail.borrow().next.clone();
            old_tail.borrow_mut().prev = new_tail.clone(); 
        }); 
        
        self.tail = new_tail;
    }
}

pub struct LRUCache {

}

/* 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    pub fn new(capacity: i32) -> Self {
        unimplemented!();     
    }
    
    pub fn get(&self, key: i32) -> i32 {
        unimplemented!();     
    }
    
    pub fn put(&self, key: i32, value: i32) {
        unimplemented!();     
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
