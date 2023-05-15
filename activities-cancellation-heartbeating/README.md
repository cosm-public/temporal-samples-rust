# Activity Cancellation and Heartbeating

This sample demonstrates:

- How a retried Activity Task can resume from the last Activity Task's heartbeat.
- How to handle canceling a long-running Activity when its associated Workflow is canceled.

Docs: [Activity heartbeating](https://docs.temporal.io/application-development/features?lang=typescript/#activity-heartbeats) and [cancellation](https://docs.temporal.io/application-development/testing/#cancel-an-activity)

## How to run
```
# in one terminal, start temporal server
temporal server start-dev

# in another terminal, start the worker
cargo run

# in yet another terminal, send a client event
cargo run --bin client
```

## NOTES

This doesn't actually cancel anything yet. Just started it.