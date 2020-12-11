var Client;
(function (Client) {
    var SERVER_IP = "";
    var USER_ID = uuidv4();
    var gameId;
    var answers;
    var timePerQuestion;
    var timeShowingAnswers;
    var timeShowingLeaderboard;
    var gameStartTime;
    var currentQuestion;
    var questionEndTime;
    var score = 0;
    var questionIndex = -1;
    $.ajaxSetup({
        contentType: "application/json; charset=utf-8"
    });
    function generateGame() {
        gameId = document.getElementById("id").value;
        var username = document.getElementById("username").value;
        $.post(SERVER_IP + "/generateGame", JSON.stringify({ gameId: gameId, uuid: USER_ID, username: username }))
            .done(function (data) {
            setUpGame(data, gameId);
        })
            .fail(function (err) {
            if (err.status == 404) {
                document.getElementById("id").className = "badbad";
                window.setTimeout(function () {
                    document.getElementById("id").className = "";
                });
            }
            else {
                alert("elliot did large ooooof");
            }
        });
    }
    Client.generateGame = generateGame;
    function setUpGame(data, id) {
        answers = data.answers;
        timePerQuestion = data.timePerQuestion;
        timeShowingAnswers = data.timeShowingAnswers;
        timeShowingLeaderboard = data.timeShowingLeaderboard;
        gameStartTime = data.gameStartTime;
        gameId = id;
        document.body.className = "leaderboard";
        document.getElementById("leaderboard").innerHTML = "Game will begin shortly.";
        window.setTimeout(function () {
            answers.forEach(function (question, index) {
                window.setTimeout(function () {
                    updateQuestion(question); // update to that question
                }, index * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard));
            });
        }, gameStartTime - new Date().getTime());
    }
    Client.setUpGame = setUpGame;
    function updateQuestion(question) {
        questionIndex++;
        console.log("updated question");
        document.body.className = "";
        currentQuestion = question;
        var questionStartTime = gameStartTime + questionIndex * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard);
        questionEndTime = questionStartTime + timePerQuestion;
        score = 0;
        window.setTimeout(function () {
            document.body.className = "leaderboard";
            var correctString = score > 0 ? "You got this one right!" : "You got this one wrong.";
            document.getElementById("leaderboard").innerHTML = correctString + "<br>Leaderboard position loading...";
            $.post(SERVER_IP + "/leaderboard", JSON.stringify({ gameId: gameId }))
                .done(function (data) {
                data.players.forEach(function (player, index) {
                    if (player.uuid == USER_ID) {
                        document.getElementById("leaderboard").innerHTML = correctString + ("<br>You're in place " + (index + 1) + ".");
                    }
                });
            });
        }, questionEndTime - new Date().getTime());
    }
    function makeGuess(id) {
        if (currentQuestion.includes(id)) {
            score = Math.round(1000 * (questionEndTime - new Date().getTime()) / timePerQuestion);
        }
        document.body.className = "leaderboard";
        document.getElementById("leaderboard").innerHTML = "Guess made.";
        $.post(SERVER_IP + "/makeGuess", JSON.stringify({
            gameId: gameId,
            uuid: USER_ID,
            correct: score > 0,
            score: score
        })).fail(function () {
            console.error("oof");
        });
    }
    Client.makeGuess = makeGuess;
})(Client || (Client = {}));
