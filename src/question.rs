use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize)]
pub enum Schwierigkeit {
    Einfach,
    Mittel,
    Schwer,
}
#[derive(Serialize)]
pub struct Frage{
    pub frage: String,
    #[serde(skip_serializing)]
    pub antwort: String,
    pub schwierigkeit: Schwierigkeit
}

impl Frage{
    pub fn new(frage:String, antwort:String, schwierigkeit:Schwierigkeit) -> Self {
        Frage {
            frage,
            antwort,
            schwierigkeit
        }
    }
}

pub struct FragenSet{
    fragen: Vec<Arc<Frage>>
}

impl FragenSet{
    pub fn dummie() -> Self{
        let mut fragen:Vec<Arc<Frage>> = Vec::new();
        fragen.push(Arc::new(Frage::new("Test".to_string(), "Penis".to_string(), Schwierigkeit::Einfach)));
        fragen.push(Arc::new(Frage::new("Ist Leon Toll?".to_string(),"100%".to_string(), Schwierigkeit::Mittel)));
        fragen.push(Arc::new(Frage::new("Eine Hütte hat den Durchmesser 13.4m und eine Seitenfläche von 5 Fuß. Auf ihrem Dach, dass einem Volumen von 3 Litern entspricht befindet sich kein Schornstein. Wie viele Huren passen in die Grage?".to_string(), "Keine".to_string(), Schwierigkeit::Schwer)));
        FragenSet{
            fragen
        }
    }

    pub fn n_te_frage(&self, n:i32) -> Option<Arc<Frage>>{
        Some(self.fragen[n as usize].clone())
    }

    pub fn count(&self) -> usize{
        self.fragen.len()
    }
}