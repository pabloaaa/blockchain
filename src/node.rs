use tokio::{io::{AsyncWriteExt}, net::TcpListener};
use std::{error::Error, sync::mpsc};
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::{block::{Block}, blockchain::Blockchain};

#[derive(Clone)]
#[allow(dead_code)]
pub struct Node {
    blockchain: Arc<Mutex<Blockchain>>,
    clients: Arc<Mutex<Vec<tokio::net::TcpStream>>>,
    new_block_receiver: Arc<Mutex<mpsc::Receiver<Block>>>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(blockchain: Blockchain, new_block_receiver: mpsc::Receiver<Block>) -> Node {
        Node {
            blockchain: Arc::new(Mutex::new(blockchain)),
            clients: Arc::new(Mutex::new(Vec::new())),
            new_block_receiver: Arc::new(Mutex::new(new_block_receiver)),
        }
    }

    pub async fn get_blockchain(&self) -> Blockchain {
        let blockchain = self.blockchain.lock().await;
        blockchain.clone()
    }

    pub async fn start(&self, address: &str) -> Result<(), Box<dyn Error>> {
        let listener = TcpListener::bind(address).await?;
        let blockchain = Arc::clone(&self.blockchain);
        let clients = Arc::clone(&self.clients);
        let new_block_receiver = Arc::clone(&self.new_block_receiver);
    
        tokio::spawn(async move {
            while let Ok(block) = {
                let receiver = new_block_receiver.lock().await;
                receiver.recv()
            } {
                let last_block;
                {
                    let mut blockchain = blockchain.lock().await;
                    last_block = blockchain.last().unwrap().clone();
    
                    if block.index == last_block.index + 1 && block.previous_hash == last_block.hash {
                        blockchain.add_block(block.clone());
                    } else {
                        eprintln!("Received block is invalid");
                    }
                }
    
                let mut clients = clients.lock().await;
                for client in clients.iter_mut() {
                    let message = serde_json::to_string(&block).unwrap();
                    client.write_all(message.as_bytes()).await.unwrap();
                }
            }
        });
    
        loop {
            let (socket, _) = listener.accept().await?;
            let clients = Arc::clone(&self.clients);
            let mut clients = clients.lock().await;
            clients.push(socket);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpStream;
    use tokio::io::AsyncWriteExt;
    use std::time::Duration;
    use tokio::time::sleep;
    use crate::blockchain::{self, Blockchain};
    use crate::block::Block;

    #[tokio::test]
    async fn test_node_start() {
        let blockchain = Blockchain::new();
        let node = Node::new(blockchain);
        let node_clone = node.clone();
        let address = "127.0.0.1:8080";

        tokio::spawn(async move {
            node_clone.start(address).await.unwrap();
        });

        // Give the server a little bit of time to start.
        sleep(Duration::from_secs(1)).await;

        // Connect to the server
        let mut stream = TcpStream::connect(address).await.unwrap();
        let blockchain = node.get_blockchain();

        // Create a new block and calculate its hash
        let last_block_hash = blockchain.last().unwrap().hash.clone();
        let mut block = Block::new(1, 0, vec![], last_block_hash, 10);
        block.calculate_hash();

        let message = serde_json::to_string(&block).unwrap();
        stream.write_all(message.as_bytes()).await.unwrap();

        // Give the server a little bit of time to process the message.
        sleep(Duration::from_secs(2)).await;

        // Check that the block was added to the blockchain, we need to get the blockchain again because the node is running in a different thread!!!
        let blockchain = node.get_blockchain();
        assert_eq!(blockchain.last().unwrap().index, 1);
        
    }
}