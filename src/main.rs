#![allow(dead_code)]

mod athena {
    pub mod types;
    pub mod sandbox;
    pub mod triumvirat;
    pub mod star_matrix;
    pub mod athena;
}

mod hermes {
    pub mod router;
}

use athena::athena::AthenaSystem;
use athena::types::FaceId;
use athena::star_matrix::StarMatrix;
use hermes::router::{HermesRouter, HermesMessage};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("=== [ BOOT UNIVERSEL ] INITIALISATION DU NEXUS SOUVERAIN V2 ===");
    
    // 1. Démarrage du cœur d'Athéna
    let (athena_task, tx) = AthenaSystem::boot();
    let hermes = HermesRouter::new(tx);

    // 2. Ouverture de la porte réseau (IP locale sur le port 8080)
    let routeur_reseau = hermes.clone();
    tokio::spawn(async move {
        if let Err(e) = routeur_reseau.demarrer_ecoute_reseau("127.0.0.1:8080").await {
            eprintln!("Erreur critique du réseau Hermès : {}", e);
        }
    });

    println!("Horloge : Battements à 100ms activés. Système non-bloquant.");
    println!("Stockage : Disque dur activé (dossier ./archives_ombre).\n");

    // 3. Test d'une impulsion locale directe (Scorpion)
    let payload_scorpion = vec![0x10, 0x00, 0x30, 0x00];
    let chevalier_scorpion = StarMatrix::invoquer_chevalier(FaceId::Scorpion);
    let _donnee_purifiee = chevalier_scorpion.traiter_fardeau(&payload_scorpion);

    hermes.process_message(
        HermesMessage::InjectImpulse(1, payload_scorpion, FaceId::Scorpion)
    ).await?;

    // 4. Test d'une impulsion lourde (Lion - Transcodage)
    let payload_lion = vec![0x55, 0xAA];
    let chevalier_lion = StarMatrix::invoquer_chevalier(FaceId::Lion);
    let donnee_amplifiee = chevalier_lion.traiter_fardeau(&payload_lion);

    hermes.process_message(
        HermesMessage::InjectImpulse(2, donnee_amplifiee, FaceId::Lion)
    ).await?;

    // Laisser le temps au système de respirer et de graver sur disque
    sleep(Duration::from_millis(400)).await;

    // 5. Arrêt propre (Apoptose)
    println!("\n=== [ HALT ] PROCÉDURE D'APOPTOSE DU NEXUS ===");
    hermes.process_message(HermesMessage::HaltSystem).await?;

    athena_task.await.map_err(|e| format!("Erreur critique : {:?}", e))?;
    
    println!("Système éteint. Données réseau gérées, ombres gravées sur disque, Chevaliers invoqués.");
    Ok(())
}
