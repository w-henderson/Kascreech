import React from 'react';
import '../styles/Leaderboard.scss';

interface LeaderboardProps {
  leaderboard: LeaderboardEntry[],
  isLastQuestion: boolean,
  nextQuestionCallback: () => void
}

class Leaderboard extends React.Component<LeaderboardProps> {
  render() {
    return (
      <div>
        <h1>Leaderboard</h1>
        {this.props.leaderboard.map((value, index) =>
          <div key={index}>
            {index + 1}: {value.userName} ({value.points}, streak of {value.streak})
          </div>
        )}

        {!this.props.isLastQuestion &&
          <div>
            <h2>Next Question</h2>
            <button onClick={this.props.nextQuestionCallback}>Next Question</button>
          </div>
        }

        {this.props.isLastQuestion && <h2>That's all folks!</h2>}
      </div>
    )
  }
}

export default Leaderboard;
