const SERVER_IP = "http://192.168.137.211:8080";
const USER_ID = uuidv4();

function showCorrectness(correct) {
  document.getElementById("correctnessIndicator").className = correct ? "show" : "show wrong";
  window.setTimeout(function () {
    document.getElementById("correctnessIndicator").className = document.getElementById("correctnessIndicator").className.replace("show", "");
    window.setTimeout(function () {
      document.getElementById("correctnessIndicator").className = "";
    }, 1000);
  }, 2000);
}

function sendResponse(colour) {
  console.log(JSON.stringify({ colour, uuid: USER_ID }));
  $.post(SERVER_IP + "/postResponse", JSON.stringify({ colour, uuid: USER_ID }), function (data) {
    data = JSON.parse(data);
    let now = new Date().getTime();
    let then = data.timestamp;
    window.setTimeout(function () {
      showCorrectness(data.correct);
    }, then - now);
  }).fail(function (err) {
    document.getElementById("oof").currentTime = 0;
    document.getElementById("oof").play();
  });
}