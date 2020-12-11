namespace Chungus {
  const SERVER_IP: string = "";

  var gameId: string;
  var questions: Question[];
  var timePerQuestion: number;
  var timeShowingAnswers: number;
  var timeShowingLeaderboard: number;
  var gameStartTime: number;
  var chungus: boolean;

  var music: boolean = false;

  $.ajaxSetup({
    contentType: "application/json; charset=utf-8"
  });

  // starts game on the server and gets its information
  export function chungusGameInfo(): void {
    $.post(SERVER_IP + "/chungusGameInfo")
      .done(function (data: ChungusGameInfo) {
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

  // sets up all the timeouts and stuff
  export function setUpGame(data: ChungusGameInfo) {
    gameId = data.gameId;
    questions = data.questions;
    timePerQuestion = data.timePerQuestion;
    timeShowingAnswers = data.timeShowingAnswers;
    timeShowingLeaderboard = data.timeShowingLeaderboard;
    gameStartTime = data.gameStartTime;
    chungus = data.bigChungus; // whether serverless testing

    let connectedPlayers = 0;

    document.getElementById("qSpan").innerHTML = `Game Code: ${gameId}`;

    // update UI every second until game begins
    for (let f = 0; f < gameStartTime - new Date().getTime(); f += 1000) {
      window.setTimeout(function () {
        let timeTillGameStart = Math.round((gameStartTime - new Date().getTime()) / 1000);
        document.getElementById("connectedInfo").innerHTML =
          `Game begins in ${timeTillGameStart} seconds.<br>` +
          `${connectedPlayers} players are connected.`;
        if (timeTillGameStart > 5 && chungus) { // if more than 5 seconds till game begins and not testing
          $.post(SERVER_IP + "/leaderboard", JSON.stringify({ gameId }))
            .done(function (returnedData: Leaderboard) {
              connectedPlayers = returnedData.players.length;
            });
        }
      }, f);
    }

    window.setTimeout(function () { // timeout to start game at right time
      console.log("started game");
      questions.forEach((question, index) => { // for each question
        window.setTimeout(function () { // set a timeout to happen at the start of the question
          updateQuestion(question, index); // update to that question
        }, index * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard));
      });
    }, gameStartTime - new Date().getTime());
  }

  // function to sanitize the html
  function sanitizeHTML(text) {
    let element = document.createElement('div');
    element.innerText = text;
    return element.innerHTML;
  }

  // method to update the screen to show a question
  function updateQuestion(question: Question, index) {
    let questionEndTime = (gameStartTime + (index + 1) * (timePerQuestion + timeShowingAnswers + timeShowingLeaderboard));
    let showAnswerTime = questionEndTime - timeShowingLeaderboard - timeShowingAnswers;
    let showLeaderboardTime = questionEndTime - timeShowingLeaderboard;

    window.setTimeout(function () { // timeout to when right answer will be shown
      document.getElementById("options").className = "showCorrect";
      document.getElementById("timer").innerHTML = "<i class='fa fa-clock'></i>0";
      if (!chungus) return;
      $.post(SERVER_IP + "/leaderboard", JSON.stringify({ gameId }))
        .done(function (returnedData: Leaderboard) {
          let i = 0
          document.getElementById("leaderboardTable").innerHTML = "";
          returnedData.players.forEach(function (user) {
            if (i >= 5) {
              return;
            }
            document.getElementById("leaderboardTable").innerHTML +=
              `<tr><td>${i + 1}</td><td>${sanitizeHTML(user.username)}</td><td>${user.score}</td></tr>`;
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
    for (let i = 0; i < 4; i++) {
      document.getElementById("opt" + (i + 1)).innerHTML = `<span>${question.responses[i]}</span>`;
      document.getElementById("opt" + (i + 1)).className = "";
    }
    question.correct.forEach(function (data) {
      document.getElementById("opt" + (data + 1).toString()).className = "correct";
    });

    // show question
    document.body.className = "";

    for (let f = 0; f < timePerQuestion; f += 1000) {
      window.setTimeout(function () {
        document.getElementById("timer").innerHTML = "<i class='fa fa-clock'></i>" + Math.round((showAnswerTime - new Date().getTime()) / 1000);
      }, f);
    }
  }

  export function toggleMusic() {
    if (!music) {
      (<HTMLAudioElement>document.getElementById("music")).play();
      document.getElementById("audioButton").innerHTML = "<i class='fas fa-volume-up'></i>";
    } else {
      (<HTMLAudioElement>document.getElementById("music")).pause();
      document.getElementById("audioButton").innerHTML = "<i class='fas fa-volume-mute'></i>";
    }
    music = !music;
  }
}