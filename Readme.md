1- Last games: il s'agit de recupérer les scores des derniers match (today moins 1 jour). Pour cela, il faut appeler l'api suivant: 
"todayScoreboard": "/prod/v2/{gameDate}/scoreboard.json"
Exemple de gameDate: si on est le 13/03/2021, il faut recupérer les scores du 12/03/2021. Le gameDate sera alors 20210312.
Dans la réponse, recupérer les informations suivantes:
    -gameId
    -arena: name, city, stateAbbr
    -statusNum: 3 (si le match a dejà eu lieu). Pour information, les autres valeurs possibles sont : 1 (si le match n'a pas encore eu lieu). 2 si le match est en cours. Ici on devrait recevoir normalement que 3 si le match s'est bien passé (car on requête sur les matchs eu la veille). cependant, c'est important de vérifier le statusNum car on n'est pas à l'abris d'un match annulé ou reporté.
    -teamId
    -triCode
    -win
    -loss
    -score
    -linescore
Affichage si le match a eu lieu (çàd statusNum===3):
Game 1: 

Team         | 1  | 2  | 3  | 4  | Total
----------------------------------------
PHIL(26-12)  | 32 | 31 | 31 | 33 | 127
----------------------------------------
WAS(14-22)   | 15 | 38 | 23 | 25 | 101

Players stats

PHIL

Player          | Min | Reb | Ass | Pts
----------------------------------------
Joel Embiid     | 34  | 3   | 10  | 18 
Ben Simmons     | 19  | 1   | 0   | 7

WAS

Player          | Min | Reb | Ass | Pts
----------------------------------------
Bradley Beal    | 34  | 3   | 10  | 18 
Russel Westbrook| 19  | 1   | 0   | 7


Arena: Capital One Arena
City: Washington
Attendance: 0

2-Today games: il s'agit de recupérer les informations sur les matchs du jour.
Pour cela, il faut appeler l'api suivant: 
"todayScoreboard": "/prod/v2/{gameDate}/scoreboard.json"
Exemple de gameDate: si on est le 13/03/2021, le gameDate sera alors 20210313.

Dans la réponse, recupérer les informations suivantes:
    -gameId
    -arena: name, city, stateAbbr
    -startTimeUTC
    -statusNum: 1 (si le match n'a pas encore eu lieu). Toujours vérifier que le statusNum est égal à 1.
    -teamId
    -triCode
    -win
    -loss
    -watch:broadcast->broadcasters: national, canadian, vTeam, hTeam: shortName
Game 1: 

Game                      | Start Time (UTC) | Broadcast
----------------------------------------------------------------
PHIL(26-12) @  WAS(14-22) | 3:30:00          | NBCSCA, FSSE-CHA
----------------------------------------------------------------

Arena: Capital One Arena
City: Washington

Exemples de command lines:
-Last games: Enter 'last_games'
-Last games with stats: Enter 'last_games_stats'
-Today games: Enter 'today_games'
