const SERVER_IP = "http://localhost";
const USER_ID = uuidv4();

function showCorrectness(correct) {
  document.getElementById("correctnessIndicator").className = correct ? "show" : "show wrong";
  window.setTimeout(function () {
    document.getElementById("correctnessIndicator").className = document.getElementById("correctnessIndicator").className.replace("show", "");
    window.setTimeout(function () {
      document.getElementById("correctnessIndicator").className = "";
    }, 1000)
  }, 2000)
}

function sendResponse(colour) {
  window.fetch(SERVER_IP + "/postResponse", {
    method: "post",
    body: JSON.stringify({ colour, USER_ID })
  }).then(json).then(function (data) {
    let now = new Date().getTime();
    let then = data.timestamp;
    window.setTimeout(function () {
      showCorrectness(data.correct);
    }, then - now);
  });
}