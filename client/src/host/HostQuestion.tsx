import React from 'react';
import '../styles/HostQuestion.scss';

import { Timer } from "@material-ui/icons";

interface QuestionProps {
  question: Question,
  showCorrect: boolean,
  questionNumber: number,
  questionCount: number,
  showAnswersCallback: () => void,
  showLeaderboardCallback: () => void
}

interface QuestionState {
  timeRemaining: number
}

class HostQuestion extends React.Component<QuestionProps, QuestionState> {
  updateInterval: NodeJS.Timeout | undefined;

  constructor(props: QuestionProps) {
    super(props);

    this.state = { timeRemaining: 0 }
    this.update = this.update.bind(this);
    this.skip = this.skip.bind(this);
  }

  componentDidMount() {
    this.setState({ timeRemaining: this.props.question.duration });
    this.updateInterval = setInterval(this.update, 1000);
  }

  update() {
    let newTimeRemaining = this.state.timeRemaining - 1;
    this.setState({ timeRemaining: newTimeRemaining });
    if (newTimeRemaining === 0) this.skip();
  }

  skip() {
    if (this.updateInterval) clearInterval(this.updateInterval);
    this.setState({ timeRemaining: 0 });
    this.props.showAnswersCallback();
  }

  render() {
    return (
      <div className="HostQuestion">
        <div>
          <h1>{this.props.question.question}</h1>
          <span className="topLeft">{this.props.questionNumber}/{this.props.questionCount}</span>
          <span className="topRight"><Timer />{this.state.timeRemaining}</span>

          {this.props.showCorrect &&
            <button
              className="bottomRight"
              onClick={this.props.showLeaderboardCallback}>Continue</button>}
          {(this.state.timeRemaining > 0 && !this.props.showCorrect) &&
            <button className="bottomRight" onClick={this.skip}>Skip</button>}
        </div>

        <div>
          {this.props.question.answers.map((answer, index) =>
            <div key={index} className={this.props.showCorrect && !answer.correct ? "incorrect" : ""}>
              {answer.text}
            </div>
          )}
        </div>
      </div>
    )
  }
}

/*{(this.state.timeRemaining > 0 && !this.props.showCorrect) &&
          <div>
            <h2>Skip Countdown</h2>
            <button onClick={this.skip}>Skip Countdown</button>
          </div>
        }

        {this.props.showCorrect &&
          <div>
            <h2>Show Leaderboard</h2>
            <button onClick={this.props.showLeaderboardCallback}>Show Leaderboard</button>
          </div>
        }*/

export default HostQuestion;
