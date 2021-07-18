import React from 'react';
import '../styles/PlayerQuestion.scss';

interface QuestionProps {
  answerCount: number,
  guessCallback: (index: number) => void
}

class PlayerQuestion extends React.Component<QuestionProps> {
  render() {
    return (
      <div>
        <h2>What's the answer?</h2>
        {[...Array(this.props.answerCount)].map((_, index) =>
          <div key={index} onClick={() => this.props.guessCallback(index)}>Answer {index + 1}</div>
        )}
      </div>
    )
  }
}

export default PlayerQuestion;
