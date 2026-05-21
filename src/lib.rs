pub mod lru_cache;

#[cfg(test)]
mod tests {
    use crate::lru_cache::DoublyLL; 

    #[test]
    fn insertion() {
        // ["LRUCache","put","put","get","put","get","put","get","get","get"]
        // [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
        let mut dll = DoublyLL::new();
        dll.push_back(2);
        dll.push_back(3);
        dll.push_front(1);
        
        assert_eq!(dll.peek_front(), Some(&1));
        assert_eq!(dll.peek_back(), Some(&3));
   } 

    #[test]
    fn erasure() {
        // ["LRUCache","put","put","get","put","get","put","get","get","get"]
        // [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]
        let mut dll = DoublyLL::new();
        dll.push_back(2);
        dll.push_back(3);
        dll.push_front(1);
        
        // 1 , 2 , 3
        dll.erase(&1).ok();
        // 2, 3
        assert_eq!(dll.peek_front(), Some(&2));
        assert_eq!(dll.peek_back(), Some(&3));
        
        dll.erase(&2).ok();
        assert_eq!(dll.peek_front(), Some(&3));
        assert_eq!(dll.peek_back(), Some(&3));
   }
}
