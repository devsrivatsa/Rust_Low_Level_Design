# This is an implmentation of a distributed message queue in rust using Actix-web.

### To test project:
- download the folder in repo.
- cd into the folder and cargo run.
- curl -X POST http://127.0.0.1:3030/enqueue -H "Content-Type: application/json" -d "\"Your message here\""
- curl http://127.0.0.1:3030/dequeue
