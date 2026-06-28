//! IPC server implementation managing Unix domain sockets asynchronously.

use std::fs;
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;
use tracing::{error, info};

/// Async Unix Domain Socket server skeleton.
pub struct IpcServer {
    socket_path: PathBuf,
}

impl IpcServer {
    /// Create a new IpcServer instance.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            socket_path: path.as_ref().to_path_buf(),
        }
    }

    /// Start the IPC server and listen for incoming streams.
    pub async fn run(&self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        // Clean up pre-existing socket files
        if self.socket_path.exists() {
            let _ = fs::remove_file(&self.socket_path);
        }

        let listener = match UnixListener::bind(&self.socket_path) {
            Ok(l) => {
                info!("IPC server successfully bound to {:?}", self.socket_path);
                l
            }
            Err(err) => {
                error!(
                    "IPC server failed to bind to {:?}: {}",
                    self.socket_path, err
                );
                return;
            }
        };

        loop {
            tokio::select! {
                accept_res = listener.accept() => {
                    match accept_res {
                        Ok((mut stream, _addr)) => {
                            info!("Accepted new local connection.");
                            tokio::spawn(async move {
                                let mut buf = [0u8; 1024];
                                while let Ok(n) = stream.read(&mut buf).await {
                                    if n == 0 {
                                        break;
                                    }
                                    info!("Read {} bytes from stream.", n);
                                }
                                info!("Local connection closed.");
                            });
                        }
                        Err(err) => {
                            error!("Error accepting stream: {}", err);
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    info!("IPC server shutting down...");
                    break;
                }
            }
        }

        // Clean up socket file on shutdown
        if self.socket_path.exists() {
            let _ = fs::remove_file(&self.socket_path);
        }
    }
}
