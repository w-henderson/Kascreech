var Chungus;
(function (Chungus) {
    var SERVER_IP = "http://kascreech.rack.ga";
    var gameId;
    var questions;
    var timePerQuestion;
    var timeShowingAnswers;
    var timeShowingLeaderboard;
    var gameStartTime;
    var chungus;
    var music = false;
    $.ajaxSetup({
        contentType: "application/json; charset=utf-8"
    });
    // starts game on the server and gets its information
    function chungusGameInfo() {
        $.post(SERVER_IP + "/chungusGameInfo")
            .done(function (data) {
            if (!data.bigChungus) {
                window.location.href = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
                return;
            }
            setUpGame(data);
        })
            .fail(function (err) {
            alert("elliot did large ooooof");
        });
    }
    Chungus.chungusGameInfo = chungusGameInfo;
    // sets up all the timeouts and stuff
    function setUpGame(data) {
        gameId = data.gameId;
        questions = data.questions;
        timePerQuestion = data.timePerQuestion;
        timeShowingAnswers = data.timeShowingAnswers;
        timeShowingLeaderboard = data.timeShowingLeaderboard;
        gameStartTime = data.gameStartTime;
        chungus = data.bigChungus; // whether serverless testing
        var connectedPlayers = 0;
        document.getElementById("qSpan").innerHTML = "Game Code: " + gameId;
        // update UI every second until game begins
        for (var f = 0; f < gameStartTime - new Date().getTime(); f += 1000) {
            window.setTimeout(function () {
                var timeTillGameStart = Math.round((gameStartTime - new Date().getTime()) / 1000);
                document.getElementById("connectedInfo").innerHTML =
                    "Game begins in " + timeTillGameStart + " seconds.<br>" +
                        (connectedPlayers + " players are connected.");
                if (timeTillGameStart > 5 && chungus) { // if more than 5 seconds till game begins and not testing
                    $.post(SERVER_IP + "/leaderboard", JSON.stringify({ gameId: gameId }))
                        .done(function (returnedData) {
                        connectedPlayers = returnedData.players.length;
                    });
                }
            }, f);
        }
        window.setTimeout(function () {
            console.log("started game");
            questions.forEach(function (question, index) {
                window.setTimeout(function () {
                    updateQuestion(question, index); // update to that question
                }, index * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard));
            });
        }, gameStartTime - new Date().getTime());
    }
    Chungus.setUpGame = setUpGame;
    // function to sanitize the html
    function sanitizeHTML(text) {
        var element = document.createElement('div');
        element.innerText = text;
        return element.innerHTML;
    }
    // method to update the screen to show a question
    function updateQuestion(question, index) {
        var questionEndTime = (gameStartTime + (index + 1) * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard));
        var showAnswerTime = questionEndTime - timeShowingLeaderboard - timeShowingAnswers;
        var showLeaderboardTime = questionEndTime - timeShowingLeaderboard;
        window.setTimeout(function () {
            document.getElementById("options").className = "showCorrect";
            document.getElementById("timer").innerHTML = "<i class='fa fa-clock'></i>0";
            if (!chungus)
                return;
            $.post(SERVER_IP + "/leaderboard", JSON.stringify({ gameId: gameId }))
                .done(function (returnedData) {
                var i = 0;
                document.getElementById("leaderboardTable").innerHTML = "";
                returnedData.players.forEach(function (user) {
                    if (i >= 5) {
                        return;
                    }
                    document.getElementById("leaderboardTable").innerHTML +=
                        "<tr><td>" + (i + 1) + "</td><td>" + sanitizeHTML(user.username) + "</td><td>" + user.score + "</td></tr>";
                    i += 1;
                });
            });
        }, showAnswerTime - new Date().getTime());
        window.setTimeout(function () {
            document.body.className = "leaderboard";
            document.getElementById("qSpan").innerHTML = "Leaderboard";
        }, showLeaderboardTime - new Date().getTime());
        // update questions shown
        document.getElementById("qSpan").innerHTML = question.question;
        document.getElementById("questionNumber").innerHTML = (index + 1) + "/" + (questions.length);
        document.getElementById("options").className = ""; // don't show correct answer yet
        for (var i = 0; i < 4; i++) {
            document.getElementById("opt" + (i + 1)).innerHTML = "<span>" + question.responses[i] + "</span>";
            document.getElementById("opt" + (i + 1)).className = "";
        }
        question.correct.forEach(function (data) {
            document.getElementById("opt" + (data + 1).toString()).className = "correct";
        });
        // show question
        document.body.className = "";
        for (var f = 0; f < timePerQuestion; f += 1000) {
            window.setTimeout(function () {
                document.getElementById("timer").innerHTML = "<i class='fa fa-clock'></i>" + Math.round((showAnswerTime - new Date().getTime()) / 1000);
            }, f);
        }
    }
    function toggleMusic() {
        if (!music) {
            document.getElementById("music").play();
            document.getElementById("audioButton").innerHTML = "<i class='fas fa-volume-up'></i>";
        }
        else {
            document.getElementById("music").pause();
            document.getElementById("audioButton").innerHTML = "<i class='fas fa-volume-mute'></i>";
        }
        music = !music;
    }
    Chungus.toggleMusic = toggleMusic;
})(Chungus || (Chungus = {}));
