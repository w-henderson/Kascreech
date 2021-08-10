# Host
Initialise game:
```json
{"success": true, "gameId": "123456", "gameName": "Game Name", "questionCount": 2}
```
Player joins:
```json
{"event": "newPlayer", "playerName": "Player X"}
```
Question:
```json
{"question": "Question Text", "duration": 10, "answers": [{"text": "Wrong Answer", "correct": false},{"text": "Wrong Answer", "correct": false},{"text": "Right Answer", "correct": true},{"text": "Wrong Answer", "correct": false}]}
```
Leaderboard:
```json
{"leaderboard": [{"userName": "Top Player", "points": 1234, "streak": 5}, {"userName": "Middle Player", "points": 790, "streak": 1}, {"userName": "Bottom Player", "points": 3, "streak": 0}]}
```

# Player
Accept into lobby:
```json
{"success": true}
```
Question start:
```json
{"event": "questionStart", "numberOfAnswers": 4}
```
Question end:
```json
{"event": "questionEnd", "correct": true, "pointsThisRound": 800, "pointsTotal": 69420, "streak": 2, "position": 3, "behind": "Big Chungus"}
```
Game end:
```json
{"event": "end", "position": 3}
```