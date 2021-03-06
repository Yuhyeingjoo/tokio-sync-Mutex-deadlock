use tokio::sync::Mutex;
use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::cell::RefCell;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
#[tokio::main]
async fn main() {
    let lock = Arc::new(Mutex::new(5));
    let lock2 = Arc::clone(&lock);
    let h3_lock = Arc::clone(&lock);
    
    let lock3 = Arc::new(Mutex::new(5));
    let lock4 = Arc::clone(&lock3);

    let h1 = tokio::spawn(async move{
        let a = lock.lock().await;
        thread::sleep(Duration::from_millis(2000));
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
        let (socket, _) = listener.accept().await.unwrap();
        let b = lock4.lock().await;
    });
	let h2 = tokio::spawn(async move{
        let a = lock3.lock().await;
        thread::sleep(Duration::from_millis(2000));
        let listener = TcpListener::bind("127.0.0.1:6380").await.unwrap();
        let (socket, _) = listener.accept().await.unwrap();
        let b = lock2.lock().await;
    });
    let h3 = tokio::spawn(async move{
        thread::sleep(Duration::from_millis(1000));
        let a = h3_lock.lock().await;

        
    });
	h1.await;
	h2.await;
    h3.await;
}
