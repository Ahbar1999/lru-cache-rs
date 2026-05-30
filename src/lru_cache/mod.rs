use std::rc::Rc;
use std::collections::HashMap;
use std::hash::{Hash};
use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::cmp::PartialEq;

// Single Threaded Doubly Linked List
/* Requirements
 * O(1) insertion at front and back
 * O(1) deletion anywhere in the list
 * */

#[derive(Clone, Debug)]
pub struct NodeRef<T: Clone + Debug + Default>(Rc<RefCell<Node<T>>>);

impl<T: Clone + Debug + Default> NodeRef<T> {
    fn new(val: T) -> Self {
        Self(Rc::new(RefCell::new(Node::new(val))))
    }
}

impl<T: Clone + PartialEq + Default + Debug> Deref for NodeRef<T> {
    type Target = Rc<RefCell<Node<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone + Default + Debug> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

/*
impl<T: Clone + PartialEq + Default> Hash for NodeRef<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::into_raw(self.0.clone()).hash(state);
    }
}
*/

// reason for writing a constructor macro: we can impl on foreign types
macro_rules! node_ref {
    ($node:expr) => {
        Rc::new(RefCell::new($node))
    }
}

#[derive(Default, Debug)]
pub struct Node<T> 
where T: Debug  + Clone + Default {
    pub val: T,
    pub prev: Option<NodeRef<T>>,
    pub next: Option<NodeRef<T>>
}

impl<T: Clone + Default + Debug> Node<T> 
    where T: Clone {
    fn new(val: T) -> Self {
        Self {
            val,
            prev: None, 
            next: None
        }
    } 
}


pub struct DoublyLL<T: Clone + Debug + Default> {
    pub head: Option<NodeRef<T>>,
    pub tail: Option<NodeRef<T>>,
    len: usize,
}

impl<T> DoublyLL<T> 
    where T: Hash + Eq + Clone + Debug + Default {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len : 0
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|head| unsafe { &(*head.as_ptr()).val })
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.as_ref().map(|tail| unsafe { &(*tail.as_ptr()).val })
    }

    pub fn len(&self) -> usize {
        self.len 
    }

    pub fn erase(&mut self, node: NodeRef<T>) -> Result<T, &str> {
        /*
        if !self.map.contains_key(key) {
            return Err("Key Not Found!");
        }
        */

        // println!("checkpiont 1"); 
        // let node = self.map.remove(key).unwrap().unwrap();
        /* 
        let node = unsafe {
            Rc::from_raw(node_ptr)
        };
        */

        // println!("{:?}", node.borrow().val);
        
        if self.head.as_ref().unwrap().borrow().val == node.borrow().val {
            self.head = node.borrow().next.clone();
        }

        if self.tail.as_ref().unwrap().borrow().val == node.borrow().val {
            self.tail = node.borrow().prev.clone();    
        }
        
        /*
        // let node = self.map.remove(key).unwrap().unwrap();
        // prev->next = node->next
        if let Some(prev_node) = &node.borrow().prev {
            prev_node.borrow_mut().next = node.borrow().next.clone();
        }
        // node.borrow().prev.as_ref().map(|prev_node| { prev_node.borrow_mut().next = node.borrow().next.clone(); }); 

        // node->next = node->prev
        // node.borrow().next.as_ref().map(|next_node| { next_node.borrow_mut().prev = node.borrow().prev.clone(); }); 
        if let Some(next_node) = &node.borrow().next {
            next_node.borrow_mut().prev = node.borrow().prev.clone();
        }
        */
        {
            // 1. Safely extract clones of next and prev, immediately dropping the borrows on `node`
            let prev_node = node.borrow().prev.clone();
            let next_node = node.borrow().next.clone();

            // 2. Update the previous node's next pointer
            if let Some(prev) = &prev_node {
                prev.borrow_mut().next = next_node.clone();
            }

            // 3. Update the next node's prev pointer
            if let Some(next) = &next_node {
                next.borrow_mut().prev = prev_node.clone();
            }
        }

        // remove references to other nodes for drop check 
        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;

        // println!("{}", Rc::strong_count(&node));
        self.len -= 1;

        Ok(Rc::try_unwrap(node.0).expect("Dangling References to the erased Node found.").into_inner().val)
    }

    pub fn push_front(&mut self, val: T) {
        // let new_head = node_ref!(Node::new(val.clone()));
        let new_head = NodeRef::new(val);
        // println!("created new {:?}", new_head);
        
        self.head.take().map(|old_head| {
            // modify links
            new_head.borrow_mut().next = Some(old_head.clone());
            old_head.borrow_mut().prev = Some(new_head.clone()); 
        }); 

        // self.map.insert(val, Some(new_head.clone()));
        
        self.head = Some(new_head); 

        if self.tail.is_none() {
            self.tail = self.head.clone();
        }

        self.len += 1;
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
        let new_tail = NodeRef::new(val);
        // node_ref!(Node::new(val.clone()));

        // println!("created new {:?}", new_tail);
        
        self.tail.take().map(|old_tail| {
            // modify links
            new_tail.borrow_mut().prev = Some(old_tail.clone());
            old_tail.borrow_mut().next = Some(new_tail.clone()); 
        }); 
        
        // self.map.insert(val, Some(new_tail.clone()));
        
        self.tail = Some(new_tail);

        if self.head.is_none() {
            self.head = self.tail.clone(); 
        }

        self.len += 1;
    }
}

pub struct LRUCache<T> where T: Clone + Debug + Default {
    pub cap: T,
    pub list: DoublyLL<T>,
    // key -> (value, node_ptr)
    pub map: HashMap<T, (T, NodeRef<T>)>,
}

/* 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache<i32> {

    pub fn new(capacity: i32) -> Self {
        Self {
            cap: capacity,
            list: DoublyLL::new(),
            map: HashMap::new()
        } 
    }
    
    pub fn get(&mut self, key: i32) -> i32 {
        if !self.map.contains_key(&key) {
            return -1;
        }

        let (_, (value, node_ref)) = self.map.remove_entry(&key).unwrap(); 
        
        self.list.push_front(key);
        self.list.erase(node_ref).ok();
        
        self.map.insert(key, (value, self.list.head.as_ref().unwrap().clone()));

        return value;
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            let (_, (_, node_ptr)) = self.map.remove_entry(&key).unwrap(); 
            
            self.list.erase(node_ptr).ok();
        }
    
        // remove the least recentyly used node 
        if self.list.len() == self.cap as usize {
            let first_key = self.list.head.as_ref().unwrap().borrow().val;
            let (_, (_, node_addr)) = self.map.remove_entry(&first_key).unwrap();
            
            self.list.erase(node_addr).ok(); 
        }

        self.list.push_front(key);
        self.map.insert(key, (value, self.list.head.as_ref().unwrap().clone()));
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
