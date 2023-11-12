use candid::{CandidType, Decode, Deserialize, Encode, Principal};

use ic_cdk::{caller, query, trap, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{storable::Bound, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Enable if you're using a Gitpod environment
const ALLOW_ANONYMOUS: bool = true;

const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType, Clone, Deserialize)]
struct Post {
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
impl Storable for Post {
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
#[derive(CandidType, Clone, Copy, Deserialize, PartialEq)]
enum Vote {
    Up,
    Down,
}

#[derive(CandidType, Clone, Deserialize)]
struct Votes {
    votes: Vec<(Principal, Vote)>,
}

impl Storable for Votes {
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
    static POSTS: RefCell<StableBTreeMap<u32, Post, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static VOTES: RefCell<StableBTreeMap<u32, Votes, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}

// Return a single post by id.
#[query]
fn get(key: u32) -> Option<Post> {
    POSTS.with(|p| p.borrow().get(&key))
}

// Returns all posts. In a real application you would use pagination to limit the number of posts
// returned with each query.
#[query]
fn get_posts() -> Vec<Post> {
    POSTS.with(|p| p.borrow().iter().map(|(_id, item)| item).collect())
}

// Inserts an entry into the map and returns the previous value of the key if it exists.
#[update(guard = is_authorized)]
fn insert(value: Post) -> Option<Post> {
    let key = value.id;
    POSTS.with(|p| p.borrow_mut().insert(key, value))
}

#[update(guard = is_authorized)]
fn vote(key: u32, v: Vote) -> Post {
    let mut votes = VOTES.with(|v| v.borrow().get(&key).unwrap_or(Votes { votes: vec![] }));

    let voted = votes
        .votes
        .iter()
        .enumerate()
        .find(|&(_idx, &(voter, _))| voter == caller());

    match voted {
        Some((idx, &value)) => {
            if value.1 == v {
                trap("You already voted for this post");
            } else {
                votes.votes.remove(idx);
            }
        }
        None => {
            votes.votes.push((caller(), v));
            VOTES.with(|v| v.borrow_mut().insert(key, votes.clone()));
        }
    }

    let inc: i32 = match v {
        Vote::Up => 1,
        Vote::Down => -1,
    };

    POSTS.with(|p| {
        let post = p.borrow().get(&key).unwrap();
        let mut post = post.clone();
        post.votes += inc;
        p.borrow_mut().insert(key, post.clone());
        post
    })
}

#[update(guard = is_authorized)]
fn remove(key: u32) -> Option<Post> {
    // Remove the votes for the post
    VOTES.with(|p| p.borrow_mut().remove(&key));
    // Remove the post
    POSTS.with(|p| p.borrow_mut().remove(&key))
}

// Check if anonymous usage is allowed
#[query]
fn is_anonymous_allowed() -> bool {
    ALLOW_ANONYMOUS
}

// We use a guard function to ensure that a caller is authenticated
pub fn is_authorized() -> Result<(), String> {
    if !ALLOW_ANONYMOUS && caller() == Principal::anonymous() {
        return Err(String::from(caller().to_text()));
    }

    Ok(())
}
