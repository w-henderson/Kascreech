import React from 'react';

import loading from "./images/loading.svg";

class LobbyLoading extends React.Component {
  render() {
    return (
      <div className="LobbyLoading">
        <img
          src={loading}
          className="loader"
          alt="Loading animation" />
      </div>
    )
  }
}

export default LobbyLoading;
