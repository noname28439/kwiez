use std::sync::Arc;
use serde::Serialize;

#[derive(Serialize)]
pub enum Schwierigkeit {
    Einfach,
    Mittel,
    Schwer,
}
#[derive(Serialize)]
pub struct Frage(pub String, pub Schwierigkeit);

pub struct FragenSet{
    fragen: Vec<Arc<Frage>>
}

impl FragenSet{
    pub fn dummie() -> Self{
        let mut fragen:Vec<Arc<Frage>> = Vec::new();
        fragen.push(Arc::new(Frage("Test".to_string(), Schwierigkeit::Einfach)));
        fragen.push(Arc::new(Frage("Ist Leon Toll?".to_string(), Schwierigkeit::Mittel)));
        fragen.push(Arc::new(Frage("Eine Hütte hat den Durchmesser 13.4m und eine Seitenfläche von 5 Fuß. Auf ihrem Dach, dass einem Volumen von 3 Litern entspricht befindet sich kein Schornstein. Wie viele Huren passen in die Grage?".to_string(), Schwierigkeit::Schwer)));
        FragenSet{
            fragen
        }
    }

    pub(crate) fn n_te_frage(&self, n:i32) -> Arc<Frage>{
        return self.fragen[n as usize].clone();
    }
}