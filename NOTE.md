## Making inter-canister calls

get publisher id

```
dfx canister id publisher
```

it will return the publisher id

```
cuj6u-c4aaa-aaaaa-qaajq-cai
```

then, let's subscribe to a topic. For example, to subscribe to the "Apples" topic, use the command:

```
dfx canister call subscriber setup_subscribe '(principal "cuj6u-c4aaa-aaaaa-qaajq-cai","Apples")'
```

Then, to publish a record to the "Apples" topic, use the command:

```
dfx canister call publisher publish '(record { "topic" = "Apples"; "value" = 2 })'
```

Then, you can query and receive the subscription record value with the command:

```
dfx canister call subscriber get_count
```

The output should resemble the following:

```
(2 : nat)
```

#### cheat

```
dfx deploy

dfx canister id publisher

dfx canister call subscriber setup_subscribe '(principal "publisher_id","Apples")'

dfx canister call publisher publish '(record { "topic" = "Apples"; "value" = 2 })'

dfx canister call subscriber get_count

dfx canister call subscriber get_all_count

dfx canister call publisher publish '(record { "topic" = "Apples"; "value" = 1 })'

dfx canister call subscriber get_count

dfx canister call subscriber get_all_count

dfx canister call subscriber setup_subscribe '(principal "publisher_id","Oranges")'

dfx canister call publisher publish '(record { "topic" = "Oranges"; "value" = 12 })'

dfx canister call subscriber get_count

dfx canister call subscriber get_all_count
```

## Test state

**reinstalling** an canister will remove all state of the canister, while **upgrading** an canister will persist/keep the old state if we want to with some mechanism (is it will by default?i dont think so).

### 1. deploy canister

first, deploy and get publisher id:

```shell
dfx deploy

dfx canister id publisher

```

we want to set some value to state:

```shell

dfx canister call subscriber setup_subscribe '(principal "publisher_id","Apples")'

dfx canister call publisher publish '(record { "topic" = "Apples"; "value" = 2 })'

dfx canister call subscriber get_count
dfx canister call publisher get_subscribers
```

above will return something like this:

```shell
()
()
(2 : nat64)
(
  vec {
    record {
      principal "avqkn-guaaa-aaaaa-qaaea-cai";
      record { 338_645_423 = "Apples" };
    };
  },
)
```

> the first two empty return value is the return value of the `setup_subscribe` and `publish` method. the third return value is the return value of the `get_count` method (will return state value of **subscriber** canister). the last return value is the return value of the `get_subscribers` method (will return state value of **publisher** canister).

before rebuild the code, we should make changes to the code, for example add print random string to the `get_subscribers` method:

```rust
// /ic-rust-pub-sub/src/publisher/src/lib.rs
#[query]
fn get_subscribers() -> SubscriberStore {
    println!("get_subscribers called.");
    SUBSCRIBERS.with(|subscribers| subscribers.borrow().clone())
}

```

and build it again:

```shell
dfx build publisher
```

then, your can stop canister that want to reinstall/upgrade:

```shell
dfx canister stop publisher
```

next step is to reinstall the canister **or** upgrade the canister.

### 2.1. re-install canister

after rebuild the code, we can re-install the canister:

```shell
dfx canister install publisher --mode reinstall

```

then, deploy and start the canister again:

```shell
dfx deploy publisher
dfx canister start publisher
```

`reinstall` mode will remove all state of the **publisher** canister.
we can check it by calling the getter method to get the value of the state, but we should deploy the canister again first:

```shell
dfx canister call subscriber get_count
dfx canister call publisher get_subscribers
```

above will return

```shell
(2 : nat64)
(vec {})
```

> since we only reinstall the publisher canister, so only the state of the publisher canister that reset to default value.

### 2.2. upgrade canister

after rebuild the code, we can upgrade the canister:

```shell
dfx canister install publisher --mode upgrade

```

then, deploy and start the canister again:

```shell
dfx deploy publisher
dfx canister start publisher
```

`upgrade` mode will keep state of the **publisher** canister. we can check it by calling the getter method to get the value of the state, but we should deploy the canister again first:

```shell
dfx canister call subscriber get_count
dfx canister call publisher get_subscribers
```

above will return

```shell
(2 : nat64)
(
  vec {
    record {
      principal "avqkn-guaaa-aaaaa-qaaea-cai";
      record { 338_645_423 = "Apples" };
    };
  },
)
```

> as you can see, in the second output, the value of subscribers state form publisher canister is still there.
