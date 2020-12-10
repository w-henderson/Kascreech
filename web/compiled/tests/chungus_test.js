var ChungusTester;
(function (ChungusTester) {
    function testSetUpGame() {
        Chungus.setUpGame({
            "bigChungus": false,
            "questions": [
                {
                    "question": "Which of the following is not a cheese?",
                    "responses": [
                        "Elliot's feet",
                        "Elliot's head",
                        "Elliot's nose",
                        "Elliot",
                    ],
                    "correct": [2]
                },
                {
                    "question": "What is 1+1?",
                    "responses": [
                        "2",
                        "Two",
                        "Window",
                        "A number",
                    ],
                    "correct": [0, 1]
                },
            ],
            "timePerQuestion": 10000,
            "timeShowingAnswers": 5000,
            "timeShowingLeaderboard": 5000,
            "gameStartTime": new Date().getTime() + 3000,
            "gameId": "123456"
        });
    }
    ChungusTester.testSetUpGame = testSetUpGame;
})(ChungusTester || (ChungusTester = {}));
