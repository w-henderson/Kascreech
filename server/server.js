const express = require('express');
const cors = require('cors');
const app = express();
app.use(cors());
app.use(express.json())
const port = 80;

app.get('/', (req, res) => {
  res.send('Hello World!')
})

app.post('/postResponse', function (req, res, next) {
  console.log(req.body);
  res.json({
    correct: true,
    timeOfFinish: new Date().getTime()
  })
})

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})