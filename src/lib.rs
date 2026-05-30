pub mod lru_cache;

#[cfg(test)]
mod tests {
    use crate::lru_cache::{LRUCache, DoublyLL}; 

    #[test]
    fn insertion() {
        // ["LRUCache","put","put","get","put","get","put","get","get","get"]
        // [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
        let mut dll = DoublyLL::new();
        dll.push_back(2);
        assert_eq!(dll.head.as_ref().unwrap().borrow().val, 2);
        assert_eq!(dll.tail.as_ref().unwrap().borrow().val, 2);

        dll.push_back(3);
        assert_eq!(dll.head.as_ref().unwrap().borrow().val, 2);
        assert_eq!(dll.tail.as_ref().unwrap().borrow().val, 3);
        
        dll.push_front(1);
        assert_eq!(dll.head.as_ref().unwrap().borrow().val, 1);
        assert_eq!(dll.tail.as_ref().unwrap().borrow().val, 3);
        
        assert_eq!(dll.peek_front(), Some(&1));
        assert_eq!(dll.peek_back(), Some(&3));
   } 

    /*
    #[test]
    fn erasure() {
        // ["LRUCache","put","put","get","put","get","put","get","get","get"]
        // [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
        let mut dll = DoublyLL::new();
        dll.push_back(2);
        // dll.inspect_list();
        
        dll.push_back(3);
        // dll.inspect_list();
        dll.push_front(1);

        // dll.inspect_list();
        
        // 1 , 2 , 3
        dll.erase(&1).ok();


        // 2, 3
        assert_eq!(dll.peek_front(), Some(&2));
        assert_eq!(dll.peek_back(), Some(&3));
        
        dll.erase(&2).ok();
        assert_eq!(dll.peek_front(), Some(&3));
        assert_eq!(dll.peek_back(), Some(&3));
   }
    */

    #[test]
    fn test_lru() {
        /*
        LRUCache lRUCache = new LRUCache(2);
        lRUCache.put(1, 1); // cache is {1=1}
        lRUCache.put(2, 2); // cache is {1=1, 2=2}
        lRUCache.get(1);    // return 1
        lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
        lRUCache.get(2);    // returns -1 (not found)
        lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
        lRUCache.get(1);    // return -1 (not found)
        lRUCache.get(3);    // return 3
        lRUCache.get(4);    // return 4
        */
        let mut lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(3, 3);
        assert_eq!(lru.get(2), -1);
        lru.put(4, 4);
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(4), 4);
    }
}
