# Host
Initialise game:
```json
{"status": "success", "gameId": "123456", "gameName": "Game Name", "questionCount": 2}
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