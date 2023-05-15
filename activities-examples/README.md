# Activities examples

## Installation

You'll need to clone the repo for the rust sdk and put it at a level one above this repo. They don't publish crates yet so you'll need this.

You can get it here:

```
# clone the sdk
git clone https://github.com/temporalio/sdk-core.git
# clone this repo
git clone https://github.com/bdbelevate/samples-rust

cd samples-rust
```

Install the temporal server
```
brew install temporal
```

## How to run

```
# in one terminal, start temporal server
temporal server start-dev

# in another terminal, start the worker
cargo run

# in yet another terminal, send a client event
cargo run --bin client
```
## API demos

### Activity APIs and design patterns
- Activities Examples:
  - makeHTTPRequest: Make an external HTTP request in an Activity (using reqwest).
  - doSomethingAsync: Complete an Activity async with AsyncCompletionClient.