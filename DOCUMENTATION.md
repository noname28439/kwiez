# Endpoints

| Methode | Payload                         | Respose                                                                   | Beschreibung                                   |
|---------|---------------------------------|---------------------------------------------------------------------------|------------------------------------------------|
| answer  | AuthToken:str<br/>AntwortString:str | Richtig:bool<br/> Evl. Timeout:objekt<br/> NächsteFrage:str                        | Eine Antwort ausprobieren                      |
| stats   | AuthToken:str                   | FragenAnzahl:int<br/>Aktueller Fortschritt:int<br/> Fortschritt Bester Spieler:int | Serverdaten fetchen                            |
| rename  | AuthToken:str<br/>NeuerName:str     |                                                                           | Legt den Nicknamen des Spielers fest           |
| ranking |                                 | topSpieler:list(str)                                                      | Gibt die Top Spieler zurück                    |
| cq      | AuthToken:str                   | aktuelleFrage:str                                                         | Gibt die nächste Frage für den Accounts zurück |

### Anmkerkung:
Alle Anfragen gehen auf den /api Endpoint und die ehemalige Route wird als Methode an 0. Stelle in einer Liste im Request Body übergeben
(Die Payload an 1. Stelle)
