use tokio::{io::{AsyncReadExt}, net::TcpListener};
use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::{block::Block, blockchain::Blockchain};

#[derive(Clone)]
pub struct Node {
    blockchain: Arc<Mutex<Blockchain>>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(blockchain: Blockchain) -> Node {
        Node {
            blockchain: Arc::new(Mutex::new(blockchain)),
        }
    }

    pub fn get_blockchain(&self) -> Blockchain {
        let blockchain = self.blockchain.lock().unwrap();
        blockchain.clone()
    }

    // Define an asynchronous function to start the Node
    pub async fn start(&self, address: &str) -> Result<(), Box<dyn Error>> {
        // Bind a TCP listener to the given address
        let listener = TcpListener::bind(address).await?;
        let blockchain = Arc::clone(&self.blockchain);

        loop {
            // Accept a new connection
            let (mut socket, _) = listener.accept().await?;
            let blockchain = Arc::clone(&blockchain);

            // Spawn a new asynchronous task to handle the connection
            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    // Read data from the socket
                    match socket.read(&mut buf).await {
                        // If no data was read, the connection was closed
                        Ok(n) => {
                            // handle message
                            let message = String::from_utf8_lossy(&buf[..n]);
                            let received_block: Block = serde_json::from_str(&message).unwrap();
                            let mut blockchain = blockchain.lock().unwrap();
                            let last_block = blockchain.last().unwrap();
                        
                            // Validate the received block
                            // TODO : nonce and transaction validation
                            if received_block.index == last_block.index + 1 && received_block.previous_hash == last_block.hash {
                                blockchain.add_block_from_existing(received_block);
                            } else {
                                eprintln!("Received block is invalid");
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read from socket; err = {:?}", e);
                            return;
                        }
                    }
                }
            });
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