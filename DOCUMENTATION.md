# Endpoints

| Methode | Payload      | Respose                                                                            | Beschreibung                                                                                     |
|---------|--------------|------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| answer  | antwort:str  | correct:bool<br/> timeout:int<br/> next:Frage<br/>block:bool                       | Eine Antwort ausprobieren (next wird nur zurückgegeben wenn die Antwort richtig war)             |
| stats   |              | FragenAnzahl:int<br/>Aktueller Fortschritt:int<br/> Fortschritt Bester Spieler:int | Serverdaten fetchen                                                                              |
| rename  | nickname:str | "ok"                                                                               | Legt den Nicknamen des Spielers fest (geht erst nachdem er eine Frage richtig beantwortet hat) |
| ranking |              | topSpieler:list(str)                                                               | Gibt die Top Spieler zurück                                                                      |
| cq      |              | _:Frage                                                                            | Gibt die nächste Frage für den Accounts zurück                                                   |

### Korrektur:
Alle Anfragen gehen auf den /api Endpoint und die ehemalige Route wird als Methode an 0. Stelle in einer Liste im Request Body übergeben.<br>
Der AuthToken wird an 1. Stelle übergeben. (Also der Authtoken wird bei jeder Request automatisch mitgesendet)<br>
Und die Payload an 2. Stelle.<br>

Beispiel-Payload (Requst Body):
["stats", "wCuuP47U30Wz8dpDQh2qNUHhPtmBhSbaPfAqoQ4HEezGiblvVItBzwIU57kKvqvf", {}]
