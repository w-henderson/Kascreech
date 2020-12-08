const SERVER_IP = "http://10.82.130.103:8080";
const USER_ID = uuidv4();

$.ajaxSetup({
  contentType: "application/json; charset=utf-8"
});

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