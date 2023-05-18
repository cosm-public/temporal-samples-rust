# Schedule example

## What it does

The client goes through a series of steps related to demonstrate the managing of a schedule. After each step, it pauses for 5 seconds.

- It creates a future schedule initially paused with an interval of 10s.
- It updates the schedule to change the interval to every 30s.
- It triggers the schedule manually.
- It unpauses the schedule.
- Finally it deletes the schedule.

Monitor all of this in the temporal web client http://localhost:8233/namespaces/default/schedules and keep refreshing at each step.

## How to run

```
# in one terminal, start temporal server
temporal server start-dev

# in another terminal, start the worker
cargo run

# in yet another terminal, run the client to create and modify the schedule
cargo run --bin client
```
