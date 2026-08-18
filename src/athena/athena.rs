use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use crate::athena::sandbox::{HologrammeOmbre, VirtualClock};
use crate::athena::triumvirat::Triumvirat;

pub enum AthenaMessage {
    InjectDonnee(Vec<u8>),
    Halt,
}

pub struct AthenaSystem;

impl AthenaSystem {
    pub fn boot() -> (tokio::task::JoinHandle<()>, mpsc::Sender<AthenaMessage>) {
        let (tx, mut rx) = mpsc::channel(100);

        let handle = tokio::spawn(async move {
            let mut clock = VirtualClock::new();
            let mut sandbox = HologrammeOmbre::new();
            let mut triumvirat = Triumvirat::new();
            let mut ticker = interval(Duration::from_millis(100));

            println!("Athéna : Système cardiaque et bouclier de conscience activés.");

            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        clock.battement_suivant();
                        if let Some(onde) = sandbox.expirer_et_feconder() {
                            triumvirat.juger_onde(onde);
                        }
                    }
                    Some(msg) = rx.recv() => {
                        match msg {
                            AthenaMessage::InjectDonnee(donnee) => {
                                sandbox.inspirer_donnee_brute(donnee);
                            }
                            AthenaMessage::Halt => {
                                println!("Athéna : Signal d'arrêt reçu. Fermeture du cœur.");
                                break;
                            }
                        }
                    }
                }
            }
        });

        (handle, tx)
    }
}
