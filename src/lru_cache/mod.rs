use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;
use std::cell::RefCell;

// Single Threaded Doubly Linked List
/* Requirements
 * O(1) insertion at front and back
 * O(1) deletion anywhere in the list
 * */

type NodeRef<T: Clone> = Option<Rc<RefCell<Node<T>>>>;

// reason for writing a constructor macro: we can impl on foreign types
macro_rules! node_ref {
    ($node:expr) => {
        Some(Rc::new(RefCell::new($node)))
    }
}

struct Node<T: Clone> {
    val: T,
    prev: NodeRef<T>,
    next: NodeRef<T>
}

impl<T> Node<T> 
    where T: Clone {
    fn new(val: T) -> Self {
        Self {
            val,
            prev: None, 
            next: None
        }
    } 
}


pub struct DoublyLL<T: Hash + Eq + Clone> {
    head: NodeRef<T>,
    tail: NodeRef<T>,
    // for our purposes we can assume all nodes are unique in this list
    // since it is being used for a key-value pair cache
    map: HashMap<T, NodeRef<T>>,
}

impl<T> DoublyLL<T> 
    where T: Hash + Eq + Clone {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            map: HashMap::new()
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|head| unsafe { &(*head.as_ptr()).val })
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.as_ref().map(|tail| unsafe { &(*tail.as_ptr()).val })
    }

    pub fn erase(&mut self, key: &T) -> Result<(), &str> { 
        if !self.map.contains_key(key) {
            return Err("Key Not Found!");
        }
        
        let node = self.map.get_mut(key).unwrap().take().unwrap();
        
        // prev->next = node->next
        node.borrow_mut().prev.as_ref().map(|prev_node| { prev_node.borrow_mut().next = node.borrow_mut().next.clone(); }); 

        // node->next = node->prev
        node.borrow_mut().next.as_ref().map(|next_node| { next_node.borrow_mut().prev = node.borrow_mut().prev.clone(); }); 

        self.map.remove(key);

        todo!("update self.head and self.tail");
        
        Ok(())
    }

    pub fn push_front(&mut self, val: T) {
        let new_head = node_ref!(Node::new(val.clone()));

        self.head.take().map(|old_head| {
            // modify links
            new_head.as_ref().unwrap().borrow_mut().next = old_head.borrow().next.clone();
            old_head.borrow_mut().prev = new_head.clone(); 
        }); 

        self.map.insert(val, new_head.clone());
        
        self.head = new_head; 
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = node_ref!(Node::new(val.clone()));

        self.tail.take().map(|old_tail| {
            // modify links
            new_tail.as_ref().unwrap().borrow_mut().prev = old_tail.borrow().next.clone();
            old_tail.borrow_mut().next = new_tail.clone(); 
        }); 
        
        self.map.insert(val, new_tail.clone());
        
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
