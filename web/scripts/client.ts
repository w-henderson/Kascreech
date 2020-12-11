namespace Client {
  const SERVER_IP = "";
  const USER_ID = uuidv4();

  var gameId: string;
  var answers: Answers[];
  var timePerQuestion: number;
  var timeShowingAnswers: number;
  var timeShowingLeaderboard: number;
  var gameStartTime: number;

  var currentQuestion: Answers;
  var questionEndTime: number;
  var score = 0;
  var questionIndex = -1;

  $.ajaxSetup({
    contentType: "application/json; charset=utf-8"
  });

  export function generateGame(): void {
    gameId = (<HTMLInputElement>document.getElementById("id")).value;
    let username: string = (<HTMLInputElement>document.getElementById("username")).value;
    $.post(SERVER_IP + "/generateGame", JSON.stringify({ gameId, uuid: USER_ID, username }))
      .done(function (data: GenerateGame) {
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

  export function setUpGame(data: GenerateGame, id: string) {
    answers = data.answers;
    timePerQuestion = data.timePerQuestion;
    timeShowingAnswers = data.timeShowingAnswers;
    timeShowingLeaderboard = data.timeShowingLeaderboard;
    gameStartTime = data.gameStartTime;
    gameId = id;

    document.body.className = "leaderboard";
    document.getElementById("leaderboard").innerHTML = "Game will begin shortly.";

    window.setTimeout(function () { // timeout to start game at right time
      answers.forEach((question, index) => { // for each question
        window.setTimeout(function () { // set a timeout to happen at the start of the question
          updateQuestion(question); // update to that question
        }, index * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard));
      });
    }, gameStartTime - new Date().getTime());
  }

  function updateQuestion(question: Answers) {
    questionIndex++;
    console.log("updated question");
    document.body.className = "";
    currentQuestion = question;
    let questionStartTime = gameStartTime + questionIndex * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard);
    questionEndTime = questionStartTime + timePerQuestion;
    score = 0;

    window.setTimeout(function () {
      document.body.className = "leaderboard";
      let correctString = score > 0 ? "You got this one right!" : "You got this one wrong."
      document.getElementById("leaderboard").innerHTML = correctString + "<br>Leaderboard position loading...";
      $.post(SERVER_IP + "/leaderboard", JSON.stringify({ gameId }))
        .done(function (data: Leaderboard) {
          data.players.forEach(function (player, index) {
            if (player.uuid == USER_ID) {
              document.getElementById("leaderboard").innerHTML = correctString + `<br>You're in place ${index + 1}.`;
            }
          });
        });
    }, questionEndTime - new Date().getTime());
  }

  export function makeGuess(id: number) {
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
}