# Endpoints

| Route    | Payload                         | Respose                                                                   | Beschreibung                                   |
|----------|---------------------------------|---------------------------------------------------------------------------|------------------------------------------------|
| /answer  | authToken:str<br/>AntwortString:str | Richtig:bool<br/> Evl. Timeout:objekt<br/> NächsteFrage:str                        | Eine Antwort ausprobieren                      |
| /stats   | authToken:str                   | FragenAnzahl:int<br/>Aktueller Fortschritt:int<br/> Fortschritt Bester Spieler:int | Serverdaten fetchen                            |
| /rename  | authToken:str<br/>NeuerName:str     |                                                                           | Legt den Nicknamen des Spielers fest           |
| /ranking |                                 | topSpieler:list(str)                                                      | Gibt die Top Spieler zurück                    |
| /cq      | authToken:str                   | aktuelleFrage:str                                                         | Gibt die nächste Frage für den Accounts zurück |


