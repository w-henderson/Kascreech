# How the server should work (once it exists)

## `/postResponse` route (HTTP POST)
- is sent a JSON string every time a user makes a guess in the format {colour: "red", USER_ID: "some uuid"}
- returns a JSON string in the format {correct: true, timeOfFinish: <timestamp>}