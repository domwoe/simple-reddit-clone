
use candid::{CandidType, Decode, Deserialize, Encode};

use ic_cdk::{query, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable, storable::Bound};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType, Clone, Deserialize)]
struct Item {
    id: u32,
    content: String,
    votes: i32,
}

// For a type to be used in a `StableBTreeMap`, it needs to implement the `Storable`
// trait, which specifies how the type can be serialized/deserialized.
//
// In this example, we're using candid to serialize/deserialize the struct, but you
// can use anything as long as you're maintaining backward-compatibility. The
// backward-compatibility allows you to change your struct over time (e.g. adding
// new fields).
//
// The `Storable` trait is already implemented for several common types (e.g. u64),
// so you can use those directly without implementing the `Storable` trait for them.
impl Storable for Item {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}


thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static ITEMS: RefCell<StableBTreeMap<u32, Item, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}


#[query]
fn get(key: u32) -> Option<Item> {
    ITEMS.with(|p| p.borrow().get(&key))
}

#[query]
fn get_posts() -> Vec<Item> {
    ITEMS.with(|p| p.borrow().iter().map(|(_id, item)| item ).collect())
}

// Inserts an entry into the map and returns the previous value of the key if it exists.
#[update]
fn insert(value: Item) -> Option<Item> {
    let key = value.id;
    ITEMS.with(|p| p.borrow_mut().insert(key, value))
}

#[update]
fn vote(key: u32, increment: i32) -> Option<Item> {
    ITEMS.with(|p| {
        let post = p.borrow().get(&key).unwrap();
        let mut post = post.clone();
        post.votes += increment;
        p.borrow_mut().insert(key, post.clone());
        Some(post)
    })
}

#[update]
fn remove(key: u32) -> Option<Item> {
    ITEMS.with(|p| p.borrow_mut().remove(&key))
}
