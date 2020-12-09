const SERVER_IP = "http://kascreech.rack.ga";
const USER_ID = uuidv4();

var gameId;
var answers;
var timePerQuestion;
var timeShowingAnswers;
var timeShowingLeaderboard;
var gameStartTime;

var currentQuestion;
var questionStartTime;
var score = 0;

$.ajaxSetup({
  contentType: "application/json; charset=utf-8"
});

function generateGame() {
  gameId = document.getElementById("id").value;
  $.post(SERVER_IP + "/generateGame", JSON.stringify({ gameId }))
    .done(function (data) {
      setUpGame(data, gameId);
    })
    .fail(function (err) {
      if (err.status == 404) {
        document.getElementById("id").className = "badbad";
        window.setTimeout(function () {
          document.getElementById("id").className = "";
        });
      } else {
        alert("elliot did large ooooof");
      }
    });
}

function setUpGame(data, id) {
  answers = data.answers;
  timePerQuestion = data.timePerQuestion;
  timeShowingAnswers = data.timeShowingAnswers;
  timeShowingLeaderboard = data.timeShowingLeaderboard;
  gameStartTime = data.gameStartTime;
  gameId = id;

  document.body.className = "leaderboard";
  document.getElementById("leaderboard").innerHTML = "Game will begin shortly.";

  window.setTimeout(function () { // timeout to start game at right time
    console.log("started game");
    answers.forEach((question, index) => { // for each question
      window.setTimeout(function () { // set a timeout to happen at the start of the question
        updateQuestion(question, index); // update to that question
      }, index * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard));
    });
  }, gameStartTime - new Date().getTime());
}

function updateQuestion(question, index) {
  document.body.className = "";
  currentQuestion = question;
  questionStartTime = new Date().getTime();
  questionEndTime = questionStartTime + index * timePerQuestion;
  score = 0;

  window.setTimeout(function () {
    document.body.className = "leaderboard";
    correctString = score > 0 ? "You got this one right!" : "You got this one wrong."
    document.getElementById("leaderboard").innerHTML = correctString + "<br>Leaderboard position loading...";
    $.post(SERVER_IP + "/leaderboard", JSON.stringify({ gameId }))
      .done(function (data) {
        data.forEach(function (player, index) {
          if (player.uuid == USER_ID) {
            document.getElementById("leaderboard").innerHTML = correctString + `<br>You're in place ${index + 1}.`;
          }
        });
      });
  }, timeShowingAnswers - new Date().getTime());
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