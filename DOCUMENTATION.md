# Endpoints

| Methode | Payload                         | Respose                                                                   | Beschreibung                                   |
|---------|---------------------------------|---------------------------------------------------------------------------|------------------------------------------------|
| answer  | AntwortString:str | Richtig:bool<br/> Evl. Timeout:objekt<br/> NächsteFrage:str                        | Eine Antwort ausprobieren                      |
| stats   |               | FragenAnzahl:int<br/>Aktueller Fortschritt:int<br/> Fortschritt Bester Spieler:int | Serverdaten fetchen                            |
| rename  | NeuerName:str     |                                                                           | Legt den Nicknamen des Spielers fest           |
| ranking |                                 | topSpieler:list(str)                                                      | Gibt die Top Spieler zurück                    |
| cq      |                    | aktuelleFrage:str                                                         | Gibt die nächste Frage für den Accounts zurück |

### Korrektur:
Alle Anfragen gehen auf den /api Endpoint und die ehemalige Route wird als Methode an 0. Stelle in einer Liste im Request Body übergeben.<br>
Der AuthToken wird an 1. Stelle übergeben. (Also der Authtoken wird bei jeder Request automatisch mitgesendet)<br>
Und die Payload an 2. Stelle.<br>

Beispiel-Payload (Requst Body):
["stats", "wCuuP47U30Wz8dpDQh2qNUHhPtmBhSbaPfAqoQ4HEezGiblvVItBzwIU57kKvqvf", {}]
