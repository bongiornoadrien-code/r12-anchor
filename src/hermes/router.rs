use tokio::sync::mpsc;
use crate::athena::athena::AthenaMessage;
use crate::athena::types::FaceId;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub enum HermesMessage {
    InjectImpulse(u64, Vec<u8>, FaceId),
    HaltSystem,
}

#[derive(Clone)]
pub struct HermesRouter {
    athena_tx: mpsc::Sender<AthenaMessage>,
}

impl HermesRouter {
    pub fn new(athena_tx: mpsc::Sender<AthenaMessage>) -> Self {
        Self { athena_tx }
    }

    pub async fn process_message(&self, msg: HermesMessage) -> Result<(), String> {
        match msg {
            HermesMessage::InjectImpulse(id, payload, face) => {
                println!("Hermès (Routeur) : Impulsion reçue de l'émir {:?} (ID: {}). Transmission à Athéna.", face, id);
                self.athena_tx.send(AthenaMessage::InjectDonnee(payload))
                    .await
                    .map_err(|e| format!("Erreur de routage vers Athéna : {}", e))?;
            }
            HermesMessage::HaltSystem => {
                println!("Hermès (Routeur) : Ordre d'arrêt global intercepté. Transmission à Athéna.");
                self.athena_tx.send(AthenaMessage::Halt)
                    .await
                    .map_err(|e| format!("Erreur lors de l'arrêt d'Athéna : {}", e))?;
            }
        }
        Ok(())
    }

    pub async fn demarrer_ecoute_reseau(&self, adresse: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(adresse).await?;
        println!("Hermès : Écoute réseau active sur le port TCP {}", adresse);

        loop {
            let (mut socket, addr) = listener.accept().await?;
            println!("Hermès : Nouvelle connexion établie avec {}", addr);
            
            let tx_clone = self.athena_tx.clone();
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                loop {
                    match socket.read(&mut buffer).await {
                        Ok(0) => break, // Connexion fermée par le client
                        Ok(n) => {
                            let donnees = buffer[..n].to_vec();
                            println!("Hermès : Reçu {} octets via TCP brut.", n);
                            let _ = tx_clone.send(AthenaMessage::InjectDonnee(donnees)).await;
                            let _ = socket.write_all(b"ACK_NEXUS_V2\n").await;
                        }
                        Err(_) => break,
                    }
                }
            });
        }
    }
}
