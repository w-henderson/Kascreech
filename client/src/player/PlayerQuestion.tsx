import React from 'react';
import '../styles/PlayerQuestion.scss';

interface QuestionProps {
  answerCount: number,
  guessCallback: (index: number) => void
}

class PlayerQuestion extends React.Component<QuestionProps> {
  render() {
    return (
      <div className="PlayerQuestion">
        {[...Array(this.props.answerCount)].map((_, index) =>
          <div key={index} onClick={() => this.props.guessCallback(index)}></div>
        )}
      </div>
    )
  }
}

export default PlayerQuestion;
