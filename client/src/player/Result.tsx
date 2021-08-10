import React from 'react';
import { ResultType } from "./Player";
import '../styles/Result.scss';

import tick from "../images/tick.svg";
import cross from "../images/cross.svg";

import getSuffix from "../suffix";

interface ResultProps {
  result: ResultType,
  position: number
}

class Result extends React.Component<ResultProps> {
  render() {
    if (this.props.result.correct) {
      return (
        <div className="Result">
          <h1>That's right!</h1>
          <img src={tick} alt="Tick" key="tick" />

          {this.props.result.streak > 0 &&
            <strong>Answer streak 🔥 {this.props.result.streak}</strong>}
          <div>+ {this.props.result.pointsThisRound}</div>

          <span>You're {this.props.position}{getSuffix(this.props.position)} with {this.props.result.pointsTotal} points!</span>
        </div>
      )
    } else {
      return (
        <div className="Result">
          <h1>Not quite!</h1>
          <img src={cross} alt="Cross" key="cross" />

          {this.props.result.streak > 0 &&
            <strong>Answer streak lost</strong>}
          <div>Better luck next time</div>

          <span>You're {this.props.position}{getSuffix(this.props.position)} with {this.props.result.pointsTotal} points!</span>
        </div>
      )
    }
  }
}

export default Result;
