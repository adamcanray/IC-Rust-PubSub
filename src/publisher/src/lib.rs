use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::BTreeMap;

type SubscriberStore = BTreeMap<Principal, Subscriber>;

thread_local! {
    // static, RefCell is a stable memory.
    static SUBSCRIBERS: RefCell<SubscriberStore> = RefCell::default();
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Counter {
    topic: String,
    value: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Subscriber {
    topic: String,
}

#[update]
fn subscribe(subscriber: Subscriber) {
    let subscriber_principal_id = ic_cdk::caller();
    SUBSCRIBERS.with(|subscribers| {
        subscribers
            .borrow_mut()
            .insert(subscriber_principal_id, subscriber)
    });
}

#[update]
async fn publish(counter: Counter) {
    SUBSCRIBERS.with(|subscribers| {
        for (k, v) in subscribers.borrow().iter() {
            if v.topic == counter.topic {
                let _call_result: Result<(), _> =
                    ic_cdk::notify(*k, "update_count", (&counter,));
            }
        }
    });    
}

#[query]
fn get_subscribers() -> SubscriberStore {
    println!("get_subscribers called. test reinstall");
    SUBSCRIBERS.with(|subscribers| subscribers.borrow().clone())
}


#[pre_upgrade]
// fn pre_upgrade() {
fn canister_pre_upgrade() {
    SUBSCRIBERS.with(|subscribers| ic_cdk::storage::stable_save((subscribers,)).unwrap());
}

#[post_upgrade]
// fn post_upgrade() {
fn canister_post_upgrade() {
    println!("post_upgrade");
    let (old_subscribers,): (SubscriberStore,) = ic_cdk::storage::stable_restore().unwrap();
    SUBSCRIBERS.with(|subscribers| *subscribers.borrow_mut() = old_subscribers);
}

