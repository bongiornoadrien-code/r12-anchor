use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::athena::sandbox::OndePulsation;

const CAPACITE_MAX_OMBRE: usize = 10_000; 

pub struct ArchivesOmbre {
    stockage_negatif: VecDeque<Vec<u8>>,
    dossier_disque: String,
}

impl ArchivesOmbre {
    pub fn new(dossier: &str) -> Self {
        if !Path::new(dossier).exists() {
            let _ = fs::create_dir_all(dossier);
        }

        Self { 
            stockage_negatif: VecDeque::with_capacity(CAPACITE_MAX_OMBRE),
            dossier_disque: dossier.to_string(),
        }
    }
    
    pub fn archiver(&mut self, duplicata: Vec<u8>) {
        if self.stockage_negatif.len() == CAPACITE_MAX_OMBRE {
            self.stockage_negatif.pop_front();
        }

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let chemin_fichier = format!("{}/ombre_{}.bin", self.dossier_disque, timestamp);
        if let Ok(mut fichier) = File::create(chemin_fichier) {
            let _ = fichier.write_all(&duplicata);
        }

        self.stockage_negatif.push_back(duplicata);
    }
}

pub struct Triumvirat {
    archives: ArchivesOmbre,
}

impl Triumvirat {
    pub fn new() -> Self {
        Self { archives: ArchivesOmbre::new("./archives_ombre") }
    }

    pub fn juger_onde(&mut self, onde: OndePulsation) {
        println!("=== [ CONSEIL ] === Jugement de l'onde émise par : {:?}", onde.emetteur());
        
        let adn_pur = onde.adn_purifie();
        if adn_pur.is_empty() {
            println!("Platon : Le noyau pur est vide après décantation. Rejet de la projection.");
        } else {
            println!("Apollon : Le noyau est consistant. Projection de l'hologramme léger dans le Réel autorisée.");
        }

        let duplicata = onde.duplicata_negatif();
        println!("Socrate : Examen du duplicata négatif...");
        
        self.archives.archiver(duplicata.to_vec());
        println!("Athéna : Duplicata scellé et gravé sur le disque dur. Cohérence du monde parallèle pérennisée.\n");
    }
}
