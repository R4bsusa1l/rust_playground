use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::accept_async;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");
    let clients: Arc<tokio::sync::Mutex<HashMap<String, mpsc::Sender<String>>>> = Arc::new(tokio::sync::Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        let clients_clone = clients.clone();
        tokio::spawn(async move {
            handle_connection(stream, clients_clone).await.unwrap();
        });
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, clients: Arc<Mutex<HashMap<String, mpsc::Sender<String>>>>) -> Result<(), Box<dyn std::error::Error>> {
    let websocket = accept_async(stream).await?;

    let (mut write, mut read) = websocket.split();

    let (tx, mut rx) = mpsc::channel(32);
    let client_id = "some_unique_id".to_string(); // In a real app generate a unique ID.
    clients.lock().await.insert(client_id.clone(), tx);

    let clients_clone_read = clients.clone();
    let client_id_read = client_id.clone();

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            write.send(Message::text(message)).await.unwrap();
        }
    });

    while let Some(message) = read.next().await {
        let data = message?.into_text()?;
        println!("Received: {}", data);
        let clients_clone_send = clients_clone_read.clone();
        let client_id_send = client_id_read.clone();
        for (id, tx) in clients_clone_send.lock().await.iter() {
            if *id != client_id_send {
                tx.send(data.clone()).await.unwrap();
            }
        }
    }

    clients.lock().await.remove(&client_id);

    Ok(())
}