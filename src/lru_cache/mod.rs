use std::rc::Rc;
use std::collections::HashMap;
use std::hash::Hash;
use std::cell::RefCell;
use std::fmt::Debug; 

// Single Threaded Doubly Linked List
/* Requirements
 * O(1) insertion at front and back
 * O(1) deletion anywhere in the list
 * */

type NodeRef<T> = Option<Rc<RefCell<Node<T>>>>;

// reason for writing a constructor macro: we can impl on foreign types
macro_rules! node_ref {
    ($node:expr) => {
        Rc::new(RefCell::new($node))
    }
}

#[derive(PartialEq, Debug)]
pub struct Node<T: Clone> {
    pub val: T,
    pub prev: NodeRef<T>,
    pub next: NodeRef<T>
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


pub struct DoublyLL<T: Hash + Eq + Clone + Debug> {
    pub head: NodeRef<T>,
    pub tail: NodeRef<T>,
    // for our purposes we can assume all nodes are unique in this list
    // since it is being used for a key-value pair cache
    map: HashMap<T, NodeRef<T>>,
}

impl<T> DoublyLL<T> 
    where T: Hash + Eq + Clone + Debug {
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

    pub fn erase(&mut self, key: &T) -> Result<T, &str> { 
        if !self.map.contains_key(key) {
            return Err("Key Not Found!");
        }
        // println!("checkpiont 1"); 
        let node = self.map.remove(key).unwrap().unwrap();
        
        if self.head.as_ref().unwrap().borrow().val == node.borrow().val {
            self.head = node.borrow().next.clone();
        }

        if self.tail.as_ref().unwrap().borrow().val == node.borrow().val {
            self.tail = node.borrow().prev.clone();    
        }
        
        // let node = self.map.remove(key).unwrap().unwrap();
        // prev->next = node->next
        node.borrow().prev.as_ref().map(|prev_node| { prev_node.borrow_mut().next = node.borrow().next.clone(); }); 

        // node->next = node->prev
        node.borrow().next.as_ref().map(|next_node| { next_node.borrow_mut().prev = node.borrow().prev.clone(); }); 
        
        // remove references to other nodes for drop check 
        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;

        // println!("{}", Rc::strong_count(&node));

        Ok(Rc::try_unwrap(node).expect("Dangling References to the erased Node found.").into_inner().val)
    }

    pub fn push_front(&mut self, val: T) {
        let new_head = node_ref!(Node::new(val.clone()));
        
        self.head.take().map(|old_head| {
            // modify links
            new_head.borrow_mut().next = Some(old_head.clone());
            old_head.borrow_mut().prev = Some(new_head.clone()); 
        }); 

        self.map.insert(val, Some(new_head.clone()));
        
        self.head = Some(new_head); 

        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }

    pub fn inspect_list(&self) {
        let mut curr = self.head.clone();
        
        loop {
            match curr {
                Some(node) => { 
                    println!("{:?}", node);
                    curr = node.borrow().next.clone();
                },
                None => { break; }
            }
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = node_ref!(Node::new(val.clone()));

        self.tail.take().map(|old_tail| {
            // modify links
            new_tail.borrow_mut().prev = Some(old_tail.clone());
            old_tail.borrow_mut().next = Some(new_tail.clone()); 
        }); 
        
        self.map.insert(val, Some(new_tail.clone()));
        
        self.tail = Some(new_tail);

        if self.head.is_none() {
            self.head = self.tail.clone(); 
        }
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
