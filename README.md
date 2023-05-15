# Temporal Rust SDK Samples

This is an attempt to start documenting the rust sdk for temporal and how to use it following some of the examples in typescript. For reference the typescript examples are available at [https://github.com/temporalio/samples-typescript](https://github.com/temporalio/samples-typescript).

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

## API demos

### Activity APIs and design patterns
- Activities Examples:
  - makeHTTPRequest: Make an external HTTP request in an Activity (using reqwest).
  - doSomethingAsync: Complete an Activity async with AsyncCompletionClient.
