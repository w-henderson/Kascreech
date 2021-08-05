import React from 'react';
import '../styles/GuessMade.scss';

import loading from "../images/loading.svg";

class GuessMade extends React.Component {
  render() {
    return (
      <div className="GuessMade">
        <h1>Guess Made!</h1>
        <img
          src={loading}
          className="loader"
          alt="Loading animation" />
      </div>
    )
  }
}

export default GuessMade;
