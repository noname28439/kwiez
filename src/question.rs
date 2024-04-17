use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use log::{info, warn};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum Schwierigkeit {
    Leicht,
    Mittel,
    Schwer,
}

impl Schwierigkeit{
    pub fn i(&self) -> i32{
        match self {
            Schwierigkeit::Leicht => 0,
            Schwierigkeit::Mittel => 1,
            Schwierigkeit::Schwer => 2
        }
    }
}

impl FromStr for Schwierigkeit{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Leicht" => Ok(Schwierigkeit::Leicht),
            "Mittel" => Ok(Schwierigkeit::Mittel),
            "Schwer" => Ok(Schwierigkeit::Schwer),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Debug)]
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
    pub fn placeholder() -> Self{
        let mut fragen:Vec<Arc<Frage>> = Vec::new();
        fragen.push(Arc::new(Frage::new("Numerical?".to_string(), "1".to_string(), Schwierigkeit::Leicht)));
        fragen.push(Arc::new(Frage::new("Test".to_string(), "Penis".to_string(), Schwierigkeit::Leicht)));
        fragen.push(Arc::new(Frage::new("Ist Leon Toll?".to_string(),"100%".to_string(), Schwierigkeit::Mittel)));
        fragen.push(Arc::new(Frage::new("Eine Hütte hat den Durchmesser 13.4m und eine Seitenfläche von 5 Fuß. Auf ihrem Dach, dass einem Volumen von 3 Litern entspricht befindet sich kein Schornstein. Wie viele Huren passen in die Grage?".to_string(), "Keine".to_string(), Schwierigkeit::Schwer)));
        FragenSet{
            fragen
        }
    }

    pub fn from_file(reader: BufReader<File>) -> Self{
        let mut fragen:Vec<Arc<Frage>> = Vec::new();

        let mut invalid_ids: Vec<String> = Vec::new();

        for line in reader.lines().skip(1){
            let line = line.unwrap();
            let columns:Vec<&str> = line.split("\t").collect();

            let build_question = || -> Result<Frage, ()>{
                let id = columns[0];
                let difficulty = Schwierigkeit::from_str(columns[1])?;
                let valid = columns[2]=="Gültig";
                let question = columns[3];
                let answer = columns[4];

                Ok(Frage::new(question.to_string(), answer.to_string(), difficulty))
            };

            match build_question() {
                Ok(q) => fragen.push(Arc::new(q)),
                Err(_) => invalid_ids.push(columns[0].to_string())
            }
        }

        if invalid_ids.len() > 0{
            warn!("Invalid questions id's: {:?}", invalid_ids);
        }

        //sort by difficulty
        fragen.sort_by(|a,b| a.schwierigkeit.i().cmp(&b.schwierigkeit.i()));


        info!("Loaded {} questions", fragen.len());

        FragenSet{
            fragen
        }
    }

    pub fn n_te_frage(&self, n:i32) -> Option<Arc<Frage>>{
        if n < 0 || n >= self.fragen.len() as i32{
             None
        }else {
            Some(self.fragen[n as usize].clone())
        }
    }

    pub fn count(&self) -> usize{
        self.fragen.len()
    }
}