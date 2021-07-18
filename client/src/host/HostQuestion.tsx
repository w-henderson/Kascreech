import React from 'react';
import '../styles/HostQuestion.scss';

interface QuestionProps {
  question: Question,
  showCorrect: boolean,
  questionNumber: number,
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
    this.props.showAnswersCallback();
  }

  render() {
    return (
      <div>
        <div>
          <h1>({this.props.questionNumber}) {this.props.question.question} - Time Remaining {this.state.timeRemaining} sec</h1>
          {this.props.question.answers.map((answer, index) =>
            <div key={index}>
              {answer.text}
              {this.props.showCorrect && (answer.correct ? "(Correct)" : "(Incorrect)")}
            </div>
          )}
        </div>

        {(this.state.timeRemaining > 0 && !this.props.showCorrect) &&
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
        }
      </div>
    )
  }
}

export default HostQuestion;
