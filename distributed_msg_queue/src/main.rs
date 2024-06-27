use std::sync::{Mutex, Arc};
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;
use uuid::Uuid;
use actix_web::{web, App, HttpServer, HttpResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    id: Uuid,
    payload: String
}

struct MessageQueue {
    queue: Mutex<VecDeque<Message>>
}
impl MessageQueue {
    fn new() -> Self {
        MessageQueue {
            queue: Mutex::new(VecDeque::new())
        }
    }

    fn enqueue(&self, payload:String) -> Uuid {
        let mut queue = self.queue.lock().unwrap();
        let message = Message {
            id: Uuid::new_v4(),
            payload
        };
        let id = message.id;
        queue.push_back(message);
        id
    }

    fn dequeue(&self) -> Option<Message> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}

async fn enqueue_handler(queue: web::Data<MessageQueue>,payload:web::Json<String>) -> HttpResponse {
    let id = queue.enqueue(payload.into_inner());
    HttpResponse::Ok().json(id)
}

async fn dequeue_handler(queue: web::Data<MessageQueue>) -> HttpResponse {
    match queue.dequeue() {
        Some(msg) => HttpResponse::Ok().json(msg),
        None => HttpResponse::Ok().json("Queue is empty")
    }
}


#[tokio::main]
async fn main() -> std::io::Result<()>{
    //automatically wraps the MessageQueue in Arc
    let queue = web::Data::new(MessageQueue::new());
    HttpServer::new(move ||
        {
            App::new()
                .app_data(queue.clone())
                .route("/enqueue", web::post().to(enqueue_handler))
                .route("/dequeue", web::get().to(dequeue_handler))
        }
    )
     .bind(("127.0.0.1", 3030))?
     .run()
     .await
}
