use serde::Serialize;

#[derive(Serialize)]
pub enum Schwierigkeit {
    Einfach,
    Mittel,
    Schwer,
}
#[derive(Serialize)]
pub struct Frage(pub String, pub Schwierigkeit);

struct FragenSet{
    fragen: Vec<Frage>
}

impl FragenSet{
    fn n_te_frage(&self, n:u16){

    }
}