# Temporal Rust SDK Samples

This is an attempt to start documenting the rust sdk for temporal and how to use it following some of the examples in typescript. For reference the typescript examples are available at [https://github.com/temporalio/samples-typescript](https://github.com/temporalio/samples-typescript).

## Notes

The rust sdk is still in alpha and a lot could change in terms of how it is used. Use at your own risk. 

Also, Cosm is not associated with temporal in any way (except that we use it) so this is not official documentation for temporal.

## Installation

You'll need to clone the repo for the rust sdk and put it at a level one above this repo. They don't publish crates yet so you'll need this.

You can get it here:

```
# clone the sdk
# currently relying on some changes that aren't merged so use the fork
git clone https://github.com/bdbelevate/sdk-core.git
# clone this repo
git clone https://github.com/cosm-eng/temporal-samples-rust

cd temporal-samples-rust
```

Install the temporal server
```
brew install temporal
```

## API demos

### Activity APIs and design patterns
- [**Activities Examples**](./activities-examples):
  - makeHTTPRequest: Make an external HTTP request in an Activity (using reqwest).
  - doSomethingAsync: Complete an Activity async with AsyncCompletionClient.
- [**Activity Cancellation and Heartbeating**](./activities-cancellation-heartbeating): Heartbeat progress for long running activities and cancel them. Plus how to skip activities and cleaning up.
- [**Timer**](./timer): Timer example to send an email notification for a long running process.
- [**Schedule Workflow**](./schedule): Demonstrates a recurring Workflow Execution that occurs according to the schedule documentation: [Schedule](https://docs.temporal.io/workflows#schedule).
