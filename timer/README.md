# Timer example

In this sample case, we want to demo a use case where the workflow starts
a long running order processing operation and in the case that the processing
takes too long, we want to send out a notification email to user about the delay,
but we won't cancel the operation. If the operation finishes before the timer fires,
then we want to cancel the timer.

The processing time takes a random amount of time between 1 and 10 seconds. Bsaed on that
you may or may not get the email. It sends it if it takes more than 2 seconds.

## How to run

```
# in one terminal, start temporal server
temporal server start-dev

# in another terminal, start the worker
cargo run

# in yet another terminal, send a client event
cargo run --bin client

# do that a few times because there is a random length of time the "processing" can take more than 2 seconds
cargo run --bin client
cargo run --bin client
...
```
