use std::time::Duration;

use actix_web::{get, web::Data, App, HttpServer, Responder};
use tokio::sync::mpsc::{self, Receiver, Sender};

async fn local_handler(mut rx: Receiver<String>) {
    while let Some(v) = rx.recv().await {
        println!("got = {}", v);
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

#[get("/")]
async fn index(tx: Data<Sender<String>>) -> impl Responder {
    if let Err(_) = tx.send("Cool".to_string()).await {
        return "Failed to send to reciever";
    }
    "sending succedded\n"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel(5);

    tokio::spawn(local_handler(rx));

    HttpServer::new(move || App::new().service(index).app_data(Data::new(tx.clone())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
