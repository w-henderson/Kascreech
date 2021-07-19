import React from 'react';
import './styles/Home.scss';

import loading from "./images/loading.svg";

class LobbyLoading extends React.Component {
  render() {
    return (
      <div>
        <img
          src={loading}
          className="loader"
          alt="Loading animation" />
      </div>
    )
  }
}

export default LobbyLoading;
