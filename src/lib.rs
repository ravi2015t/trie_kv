use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct TrieNode<T> {
    children: HashMap<char, Box<TrieNode<T>>>,
    value: Option<T>,
    is_end_of_key: bool,
}

impl<T> TrieNode<T> {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            value: None,
            is_end_of_key: false,
        }
    }
}

struct Trie<T: Clone> {
    root: Arc<Mutex<TrieNode<T>>>,
}

impl<T: Clone> Trie<T> {
    fn new() -> Self {
        Trie {
            root: Arc::new(Mutex::new(TrieNode::new())),
        }
    }

    fn insert(&self, key: String, value: T) {
        let mut current = self.root.lock().unwrap();
        let mut node: Option<&mut TrieNode<T>> = None;
        for ch in key.chars() {
            match node {
                Some(n) => {
                    node = Some(
                        n.children
                            .entry(ch)
                            .or_insert_with(|| Box::new(TrieNode::new()))
                            .as_mut(),
                    );
                }
                None => {
                    node = Some(
                        current
                            .children
                            .entry(ch)
                            .or_insert_with(|| Box::new(TrieNode::new()))
                            .as_mut(),
                    );
                }
            }
        }
        match node {
            Some(n) => {
                n.is_end_of_key = true;
                n.value = Some(value);
            }
            None => {}
        }
    }

    fn get(&self, key: &str) -> Option<T> {
        let current = self.root.lock().unwrap();

        let mut node: Option<&TrieNode<T>> = None;
        for ch in key.chars() {
            match node {
                Some(n) => {
                    if let Some(no) = n.children.get(&ch) {
                        node = Some(no.as_ref());
                    } else {
                        return None;
                    }
                }
                None => {
                    if let Some(no) = current.children.get(&ch) {
                        node = Some(no.as_ref());
                    } else {
                        return None;
                    }
                }
            }
        }
        match node {
            Some(n) => {
                if n.is_end_of_key {
                    n.value.clone()
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    #[test]
    fn it_works() {
        let trie = Arc::new(Trie::new());
        let mut handles = vec![];
        let mut map = HashMap::new();
        map.insert("hello", 10);
        map.insert("world", 20);
        map.insert("rust", 30);

        let map = Arc::new(map);

        for (k, v) in map.iter() {
            trie.insert(k.to_string(), v.clone());
        }

        for key in vec!["hello", "world", "rust", "foo"] {
            let trie_ref = Arc::clone(&trie);
            let map_ref = Arc::clone(&map);
            let handle = thread::spawn(move || {
                let value = trie_ref.get(key);
                if let Some(val) = value {
                    assert_eq!(map_ref.get(key).unwrap().clone(), val);
                } else {
                    assert_eq!(value, None);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
