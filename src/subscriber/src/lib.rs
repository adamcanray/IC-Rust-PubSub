use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::Cell;

thread_local! {
    // static, Cell will be initialized to default value every time the canister is started/deployed
    static COUNTER: Cell<u64> = Cell::new(0);
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
async fn setup_subscribe(publisher_id: Principal, topic: String) {
    let subscriber = Subscriber { topic };
    let _call_result: Result<(), _> =
        ic_cdk::call(publisher_id, "subscribe", (subscriber,)).await;
}

#[update]
fn update_count(counter: Counter) {
    COUNTER.with(|c| {
        c.set(c.get() + counter.value);
    });
}

#[query]
fn get_count() -> u64 {
    COUNTER.with(|c| {
        c.get()
    })
}

#[query]
// fn get_all_count() -> Vec<u64> {
fn get_all_count() -> String {
    let mut all_count = Vec::new();
    COUNTER.with(|c| {
        all_count.push(c.get());
    });
    // convert all_count to a string so that it visinle in the console
    let all_count_string = format!("{:?}", all_count);
    all_count_string
}
