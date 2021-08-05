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
      <div className="Leaderboard">
        <div className="header">
          <h1>{this.props.isLastQuestion ? "That's all folks!" : "Scoreboard"}</h1>
        </div>

        {!this.props.isLastQuestion &&
          <button onClick={this.props.nextQuestionCallback}>Next</button>
        }

        <div className="scores">
          {this.props.leaderboard.slice(0, 5).map((value, index) =>
            <div key={index}>
              <span>{value.userName}</span>
              {value.streak >= 3 && <span className="streak">🔥 {value.streak}</span>}
              <span>{value.points}</span>
            </div>
          )}
        </div>
      </div>
    )
  }
}

export default Leaderboard;
