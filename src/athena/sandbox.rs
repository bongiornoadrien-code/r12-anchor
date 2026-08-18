use crate::athena::types::FaceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseCardiaque {
    Inspiration,
    Compression,
    ExpirationOmbre,
}

pub struct VirtualClock {
    ticks: u64,
    phase: PhaseCardiaque,
}

impl VirtualClock {
    pub fn new() -> Self {
        Self { ticks: 0, phase: PhaseCardiaque::Inspiration }
    }

    pub fn battement_suivant(&mut self) {
        self.ticks += 1;
        self.phase = match self.ticks % 3 {
            1 => PhaseCardiaque::Inspiration,
            2 => PhaseCardiaque::Compression,
            _ => PhaseCardiaque::ExpirationOmbre,
        };
    }

    #[allow(dead_code)]
    pub fn phase(&self) -> PhaseCardiaque {
        self.phase
    }
}

pub struct OndePulsation {
    emetteur: FaceId,
    adn_purifie: Vec<u8>,
    duplicata_negatif: Vec<u8>,
}

impl OndePulsation {
    pub fn new(emetteur: FaceId, adn: &[u8]) -> Self {
        let milieu = adn.len() / 2;
        let adn_purifie = adn[..milieu].to_vec();
        let duplicata_negatif = adn[milieu..].to_vec();
        Self { emetteur, adn_purifie, duplicata_negatif }
    }

    pub fn emetteur(&self) -> FaceId {
        self.emetteur
    }

    pub fn adn_purifie(&self) -> &[u8] {
        &self.adn_purifie
    }

    pub fn duplicata_negatif(&self) -> &[u8] {
        &self.duplicata_negatif
    }
}

pub struct HologrammeOmbre {
    tampon_brut: Vec<u8>,
}

impl HologrammeOmbre {
    pub fn new() -> Self {
        Self { tampon_brut: Vec::new() }
    }

    pub fn inspirer_donnee_brute(&mut self, mut donnee: Vec<u8>) {
        self.tampon_brut.append(&mut donnee);
    }

    pub fn expirer_et_feconder(&mut self) -> Option<OndePulsation> {
        if self.tampon_brut.is_empty() {
            None
        } else {
            let donnee = std::mem::take(&mut self.tampon_brut);
            Some(OndePulsation::new(FaceId::Scorpion, &donnee))
        }
    }
}
