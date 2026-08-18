use crate::athena::types::FaceId;

pub trait Chevalier: Send + Sync {
    fn traiter_fardeau(&self, fardeau: &[u8]) -> Vec<u8>;
}

pub struct ChevalierScorpion;
impl Chevalier for ChevalierScorpion {
    fn traiter_fardeau(&self, fardeau: &[u8]) -> Vec<u8> {
        fardeau.iter().map(|b| b ^ 0xFF).collect()
    }
}

pub struct ChevalierLion;
impl Chevalier for ChevalierLion {
    fn traiter_fardeau(&self, fardeau: &[u8]) -> Vec<u8> {
        let mut res = fardeau.to_vec();
        res.reverse();
        res
    }
}

pub struct ChevalierVerseau;
impl Chevalier for ChevalierVerseau {
    fn traiter_fardeau(&self, fardeau: &[u8]) -> Vec<u8> {
        fardeau.iter().map(|b| b.rotate_left(1)).collect()
    }
}

pub struct ChevalierBier;
impl Chevalier for ChevalierBier {
    fn traiter_fardeau(&self, fardeau: &[u8]) -> Vec<u8> {
        let mut res = fardeau.to_vec();
        res.extend_from_slice(b"_bili");
        res
    }
}

pub struct StarMatrix;

impl StarMatrix {
    pub fn invoquer_chevalier(face: FaceId) -> Box<dyn Chevalier> {
        match face {
            FaceId::Scorpion => Box::new(ChevalierScorpion),
            FaceId::Lion => Box::new(ChevalierLion),
            FaceId::Verseau => Box::new(ChevalierVerseau),
            FaceId::Bier => Box::new(ChevalierBier),
        }
    }
}
