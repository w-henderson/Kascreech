import React from 'react';
import { ResultType } from "./Player";
import '../styles/Result.scss';

interface ResultProps {
  result: ResultType,
  position: number
}

class Result extends React.Component<ResultProps> {
  render() {
    return (
      <div>
        <h2>{this.props.result.correct ? "Well done, you got it right!" : "Sadge, you got it wrong."}</h2>
        <strong>You're in position {this.props.position}!</strong><br />
        Points: {this.props.result.pointsThisRound}<br />
        Total Points: {this.props.result.pointsTotal}<br />
        Streak: {this.props.result.streak}<br />
        {this.props.result.behind !== null &&
          <span>You're just behind {this.props.result.behind}!</span>
        }
      </div>
    )
  }
}

export default Result;
