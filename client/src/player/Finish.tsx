import React from 'react';
import Confetti, { ConfettiConfig } from 'react-dom-confetti';
import '../styles/Finish.scss';

import getSuffix from "../suffix";

interface FinishProps {
  position: number
}

interface FinishState {
  confetti: boolean
}

class Finish extends React.Component<FinishProps, FinishState> {
  constructor(props: FinishProps) {
    super(props);
    this.state = { confetti: false };
  }

  componentDidMount() {
    this.setState({ confetti: true });
  }

  render() {
    let comment: string = "Nice work!";
    if (this.props.position === 1) comment = "Congratulations on your victory!";
    if (this.props.position === 2) comment = "Nicely done, just one away from first!";
    if (this.props.position === 3) comment = "On the podium, nicely done!";
    if (this.props.position >= 8) comment = "Better luck next time!";

    const config: ConfettiConfig = {
      angle: 0,
      spread: 360,
      startVelocity: 25,
      elementCount: 100,
      dragFriction: 0.1,
      duration: 4000,
      stagger: 3,
      width: "10px",
      height: "10px",
      colors: ["#a864fd", "#29cdff", "#78ff44", "#ff718d", "#fdff6a"]
    };

    return (
      <div className="Finish">
        <h1>{this.props.position}{getSuffix(this.props.position)}</h1>
        <Confetti active={this.state.confetti} config={config} />
        {comment}
      </div>
    )
  }
}

export default Finish;
