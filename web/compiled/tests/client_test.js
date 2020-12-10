var ClientTester;
(function (ClientTester) {
    function testSetUpGame() {
        Client.setUpGame({
            "answers": [[1], [3]],
            "timePerQuestion": 10000,
            "timeShowingAnswers": 5000,
            "timeShowingLeaderboard": 5000,
            "gameStartTime": new Date().getTime() + 3000
        }, "123456");
    }
    ClientTester.testSetUpGame = testSetUpGame;
})(ClientTester || (ClientTester = {}));