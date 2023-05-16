# Activity Cancellation and Heartbeating

This sample demonstrates:

- How a retried Activity Task can resume from the last Activity Task's heartbeat.
- How an activity can be skipped.
- How to have a cleanup activity.
- How to handle canceling a long-running Activity when its associated Workflow is canceled.

Docs: [Activity heartbeating](https://docs.temporal.io/application-development/features?lang=typescript/#activity-heartbeats) and [cancellation](https://docs.temporal.io/application-development/testing/#cancel-an-activity)

## Preparation

Make sure you have the temporal server running in one window.

```
# in one terminal, start temporal server
temporal server start-dev
```

Now follow the instructions below.

## How it works

Start the worker `cargo run` and then start the client in another terminal `cargo run --bin client`. The below should be similar output for what you get in the worker terminal.
```
cargo run
   Compiling activities-cancellation-heartbeating v0.1.0 (/Users/bdeboer/workspaces/cosm/temporal/samples-rust/activities-cancellation-heartbeating)
    Finished dev [unoptimized + debuginfo] target(s) in 1.33s
     Running `target/debug/main`
[2023-05-16T16:00:13Z INFO  activities_cancellation_heartbeating::workflows] Inside run_cancellable_activity
[2023-05-16T16:00:13Z INFO  activities_cancellation_heartbeating::activities::fake_progress_activity] Starting fake progress activity
Progress: 1
[2023-05-16T16:00:13Z INFO  activities_cancellation_heartbeating::activities::skipped_activity] Starting skipped activity
[2023-05-16T16:00:13Z INFO  activities_cancellation_heartbeating::workflows] Inside run_cancellable_activity
Progress: 2
Progress: 3
Progress: 4
Progress: 5
Progress: 6
^C
```

Hit Control-C while it is running and wait long enough that the heartbeat timeout is exceeded (3s). Then restart the worker `cargo run` and it will pick up where it left off (or about there). Then the client will send a cancellation and you'll see that it cancels the activity. The skipped activity is never reached because the future is dropped by the tokio::select!. And then the cleanup activity is run at the very end.

```
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/main`
[2023-05-16T16:00:22Z INFO  activities_cancellation_heartbeating::activities::fake_progress_activity] Starting fake progress activity
Progress: 5
Progress: 6
Progress: 7
Progress: 8
Progress: 9
Progress: 10
Progress: 11
Progress: 12
Progress: 13
Progress: 14
Progress: 15
Progress: 16
Progress: 17
Progress: 18
Progress: 19
Progress: 20
Progress: 21
Progress: 22
Progress: 23
Progress: 24
Progress: 25
Progress: 26
Progress: 27
Progress: 28
Progress: 29
Progress: 30
Progress: 31
Progress: 32
Progress: 33
Progress: 34
Progress: 35
Progress: 36
Progress: 37
Progress: 38
Progress: 39
Progress: 40
Progress: 41
Progress: 42
Progress: 43
Progress: 44
Progress: 45
Progress: 46
Progress: 47
[2023-05-16T16:00:43Z INFO  activities_cancellation_heartbeating::workflows] Inside run_cancellable_activity
[2023-05-16T16:00:43Z WARN  activities_cancellation_heartbeating::workflows] ## workflow canceled ##
[2023-05-16T16:00:43Z INFO  activities_cancellation_heartbeating::activities::cleanup_activity] Starting cleanup activity
[2023-05-16T16:00:43Z INFO  activities_cancellation_heartbeating::workflows] Inside run_cancellable_activity
[2023-05-16T16:00:43Z WARN  activities_cancellation_heartbeating::workflows] ## workflow canceled ##
[2023-05-16T16:00:43Z INFO  activities_cancellation_heartbeating::workflows] In the end the value was: 0
Progress: 48
### Activity canceled <cancel handle> ###
```