use dashmap::DashMap;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::Mutex;

trait ConcurrentHashMap<K, V> {
    fn new() -> Self;
    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn get(&self, key: &K) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}
struct HashMapMutex<K: Eq + Hash, V> {
    map: Arc<Mutex<HashMap<K, V>>>,
}

impl<K: Eq + Hash, V: Clone> Clone for HashMapMutex<K, V> {
    fn clone(&self) -> Self {
        HashMapMutex {
            map: self.map.clone(),
        }
    }
}

impl<K: Eq + Hash, V: Clone> ConcurrentHashMap<K, V> for HashMapMutex<K, V> {
    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.lock().unwrap().remove(key)
    }
    fn new() -> Self {
        HashMapMutex {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.lock().unwrap().insert(key, value)
    }
    fn get(&self, key: &K) -> Option<V> {
        let lock = self.map.lock().unwrap();
        lock.get(key).map(|v| v.clone())
    }
}

#[tokio::main]
async fn main() {
    let map = HashMapMutex::new();
    let n = 10;

    let mut handlers = vec![];
    for id in 0..n {
        let mut map = map.clone();
        let handle = tokio::spawn(async move {
            map.insert("key".to_string(), "value".to_string() + &id.to_string());
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.await.unwrap();
    }
    println!("{:?}", map.get(&"key".to_string()));

    let mut handlers = vec![];
    let map = Arc::new(DashMap::new());
    for id in 0..n {
        let map = map.clone();
        let handle = tokio::spawn(async move {
            map.insert("key".to_string(), "value".to_string() + &id.to_string());
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.await.unwrap();
    }
    println!("{:?}", map.get(&"key".to_string()));
}
