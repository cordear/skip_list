use rand;
use std::{fmt::Debug, ptr::NonNull};
type Link<K, V> = Option<NonNull<Node<K, V>>>;
struct Node<K, V>
where
    K: PartialOrd,
{
    key: Option<K>,
    value: Option<V>,
    level: usize,
    pub forward: Vec<Link<K, V>>,
}

pub struct SkipList<K, V>
where
    K: PartialOrd,
{
    max_level: usize,
    current_level: usize,
    current_element: usize,
    head: Box<Node<K, V>>,
}

impl<K, V> Node<K, V>
where
    K: PartialOrd,
{
    pub fn new(key: K, value: V, level: usize) -> Self {
        Self {
            key: Some(key),
            value: Some(value),
            level,
            forward: vec![None; level + 1],
        }
    }
    pub fn get_key(&self) -> Option<&K> {
        return self.key.as_ref();
    }
    pub fn get_value(&self) -> Option<&V> {
        return self.value.as_ref();
    }
    pub fn set_value(&mut self, value: V) {
        self.value = Some(value);
    }
    pub fn is_head(&self) -> bool {
        return self.key.is_none();
    }
}

impl<K, V> Node<K, V>
where
    K: PartialOrd,
{
    fn head(max_level: usize) -> Self {
        Self {
            key: None,
            value: None,
            level: max_level,
            forward: vec![None; max_level + 1],
        }
    }
}

impl<K, V> SkipList<K, V>
where
    K: PartialOrd,
{
    pub fn new(max_level: usize) -> Self {
        Self {
            max_level,
            current_level: 0,
            current_element: 0,
            head: Box::new(Node::head(max_level)),
        }
    }
    pub fn len(&self) -> usize {
        return self.current_element;
    }
    pub fn is_empty(&self) -> bool {
        return self.current_element == 0;
    }
    pub fn insert(&mut self, key: K, value: V) {
        let mut cur: NonNull<Node<K, V>> = (&(*self.head)).into();
        let mut update: Vec<NonNull<Node<K, V>>> = vec![NonNull::dangling(); self.max_level + 1];
        for i in (0..=self.current_level).rev() {
            unsafe {
                while let Some(ptr) = cur.as_ref().forward[i] {
                    if ptr.as_ref().get_key().unwrap() < &key {
                        cur = ptr;
                    } else {
                        break;
                    }
                }
            }
            update[i] = cur;
        }
        let random_level = self.get_random_level();
        unsafe {
            if let Some(node) = cur.as_ref().forward[0] {
                if node.as_ref().get_key().unwrap() == &key {
                    return;
                }
            }
        }
        let create_node = Box::new(Node::new(key, value, random_level));
        let create_node: Link<K, V> = NonNull::new(Box::into_raw(create_node));
        if random_level > self.current_level {
            for n in update
                .iter_mut()
                .take(random_level + 1)
                .skip(self.current_level + 1)
            {
                *n = (&(*self.head)).into();
            }
            self.current_level = random_level;
        }
        unsafe {
            for (i, n) in update.iter_mut().enumerate().take(random_level + 1) {
                create_node.unwrap().as_mut().forward[i] = (*n).as_ref().forward[i];
                (*n).as_mut().forward[i] = create_node;
            }
        }
        self.current_element += 1;
    }
    pub fn search(&self, key: &K) -> Option<&V> {
        let mut cur: NonNull<Node<K, V>> = (&(*self.head)).into();
        for i in (0..=self.current_level).rev() {
            unsafe {
                while let Some(node) = cur.as_ref().forward[i] {
                    if node.as_ref().get_key().unwrap() < key {
                        cur = node;
                    } else {
                        break;
                    }
                }
            }
        }
        unsafe {
            if let Some(p) = cur.as_ref().forward[0] {
                if p.as_ref().get_key().unwrap() == key {
                    return p.as_ref().get_value();
                }
            }
        }
        return None;
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut cur: NonNull<Node<K, V>> = (&(*self.head)).into();
        let mut update = vec![NonNull::dangling(); self.max_level + 1];
        for i in (0..=self.current_level).rev() {
            unsafe {
                while let Some(ptr) = cur.as_ref().forward[i] {
                    if ptr.as_ref().get_key().unwrap() < key {
                        cur = ptr;
                    } else {
                        break;
                    }
                }
            }
            update[i] = cur;
        }
        unsafe {
            if let Some(ptr) = cur.as_ref().forward[0] {
                cur = ptr;
                if cur.as_ref().get_key().unwrap() == key {
                    for (i, n) in update.iter_mut().enumerate().take(self.current_level + 1) {
                        if let Some(node) = (*n).as_ref().forward[i] {
                            if node != cur {
                                break;
                            }
                            (*n).as_mut().forward[i] = cur.as_ref().forward[i];
                        }
                    }
                    while self.current_level > 0 && self.head.forward[self.current_level].is_none()
                    {
                        self.current_level -= 1;
                    }
                    self.current_element -= 1;
                    let return_value = cur.as_mut().value.take();
                    drop(Box::from_raw(cur.as_ptr()));
                    return return_value;
                }
            }
        }
        return None;
    }
}
impl<K, V> SkipList<K, V>
where
    K: PartialOrd + Debug,
    V: Debug,
{
    pub fn print_list(&self) {
        for i in 0..self.current_level {
            print!("Level {i}: ");
            let mut node = self.head.forward[i];
            while let Some(n) = node {
                unsafe {
                    print!(
                        "{:#?}:{:#?} ",
                        n.as_ref().get_key().unwrap(),
                        n.as_ref().get_value().unwrap()
                    );
                    node = n.as_ref().forward[i];
                }
            }
            println!();
        }
        println!();
    }
}
impl<K, V> Drop for SkipList<K, V>
where
    K: PartialOrd,
{
    fn drop(&mut self) {
        let mut cur: NonNull<Node<K, V>> = (&(*self.head)).into();
        unsafe {
            if let Some(ptr) = cur.as_mut().forward[0] {
                cur = ptr;

                while let Some(ptr) = cur.as_mut().forward[0] {
                    drop(Box::from_raw(cur.as_ptr()));
                    cur = ptr;
                }
                drop(Box::from_raw(cur.as_ptr()));
            }
        }
    }
}

impl<K, V> SkipList<K, V>
where
    K: PartialOrd,
{
    fn get_random_level(&self) -> usize {
        let mut level = 1;
        while level < self.max_level && rand::random::<usize>() % 2 == 1 {
            level += 1;
        }
        return level;
    }
}
