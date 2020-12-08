const SERVER_IP = "http://10.82.130.103:8080";

var gameId;
var questions;
var timePerQuestion;
var timeShowingAnswers;
var gameStartTime;
var gameId;

// starts game on the server and gets its information
function chungusGameInfo() {
  $.post(SERVER_IP + "/chungusGameInfo")
    .done(function (data) {
      if (!data.bigChungus) {
        window.location = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        return;
      }

      gameId = data.gameId;
      questions = data.questions;
      timePerQuestion = data.timePerQuestion;
      timeShowingAnswers = data.timeShowingAnswers;
      gameStartTime = data.gameStartTime;

      console.log({ gameId, questions, timePerQuestion, timeShowingAnswers, gameStartTime });
    })
    .fail(function (err) {
      alert("elliot did large ooooof");
    });
}