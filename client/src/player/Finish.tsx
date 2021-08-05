import React from 'react';
import '../styles/Finish.scss';

import getSuffix from "../suffix";

interface FinishProps {
  position: number
}

class Finish extends React.Component<FinishProps> {
  render() {
    let comment: string = "Nice work!";
    if (this.props.position == 1) comment = "Congratulations on your victory!";
    if (this.props.position == 2) comment = "Nicely done, just one away from first!";
    if (this.props.position == 3) comment = "On the podium, nicely done!";
    if (this.props.position >= 8) comment = "Better luck next time!";

    return (
      <div className="Finish">
        <h1>{this.props.position}{getSuffix(this.props.position)}</h1>
        {comment}
      </div>
    )
  }
}

export default Finish;
