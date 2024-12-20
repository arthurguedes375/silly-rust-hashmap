use std::hash::{DefaultHasher, Hash, Hasher};

const DEFAULT_BUFFER_CAPACITY: usize = 30;

trait KTraits: Eq + Clone + Hash {}
trait VTraits: Copy {}

impl<T> KTraits for T where T: Eq + Clone + Hash {}
impl<T> VTraits for T where T: Copy {}

#[derive(Debug)]
struct Node<K: KTraits, V: VTraits> {
    key: K,
    val: V,
    next: Option<Box<Node<K, V>>>,
}

impl<'a, K, V> Node<K, V>
where
    K: KTraits,
    V: VTraits,
{
    pub fn new(key: &K, val: &V) -> Self {
        Node {
            key: key.clone(),
            val: *val,
            next: None,
        }
    }

    pub fn upsert(&mut self, key: &K, val: &V) {
        if self.key == *key {
            self.val = *val;
        } else {
            if let Some(next) = &mut self.next {
                next.upsert(key, val);
            } else {
                self.next = Some(Box::new(Node::new(key, val)));
            }
        }
    }

    pub fn search_by_key(&'a self, key: &K) -> Option<&'a Node<K, V>> {
        if self.key != *key {
            if let Some(next) = &self.next {
                return next.search_by_key(key);
            }
            return None;
        }

        Some(&self)
    }
}

#[derive(Debug)]
struct MyHashMap<K: KTraits, V: VTraits> {
    buffer: Vec<Option<Node<K, V>>>,
}

impl<'a, K, V> MyHashMap<K, V>
where
    K: KTraits,
    V: VTraits,
{
    pub fn new() -> Self {
        let mut buffer = Vec::with_capacity(DEFAULT_BUFFER_CAPACITY);
        for i in 0..buffer.capacity() {
            buffer.push(None);
        }
        Self { buffer }
    }

    pub fn key_to_hash(key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    pub fn insert(&mut self, key: &K, val: &V) {
        let hashed_key = Self::key_to_hash(key);
        let bucket_index = hashed_key as usize % self.buffer.capacity();
        let possible_bucket = &mut self.buffer[bucket_index];
        if let Some(bucket) = possible_bucket {
            bucket.upsert(key, val);
        } else {
            *possible_bucket = Some(Node::new(key, val));
        }
    }

    pub fn get(&'a self, key: &K) -> Option<&'a V> {
        let hashed_key = Self::key_to_hash(key) as usize;
        let bucket_index = hashed_key % self.buffer.capacity();
        if let Some(bucket) = &self.buffer[bucket_index] {
            let possible_result: Option<&'a Node<K, V>> = bucket.search_by_key(key);
            if let Some(result) = possible_result {
                return Some(&result.val)
            }
        }
        None
    }
}

fn main() {
    let mut hash: MyHashMap<String, i32> = MyHashMap::new();

    hash.insert(&String::from("Test"), &6);
    hash.insert(&String::from("Test2"), &7);
    hash.insert(&String::from("Test"), &8);
    hash.insert(&String::from("Test1"), &20);
    hash.insert(&String::from("Test2"), &15);
    hash.insert(&String::from("Test1"), &230);

    println!("{:#?}", hash);

    println!("\n\n{:#?}", hash.get(&String::from("Test")));
    println!("\n\n{:#?}", hash.get(&String::from("Test1")));
    println!("\n\n{:#?}", hash.get(&String::from("Test2")));
    println!("\n\n{:#?}", hash.get(&String::from("Te1")));

}
