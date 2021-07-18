import React from 'react';
import './styles/Home.scss';

import loading from "./images/loading.svg";

class LobbyLoading extends React.Component {
  render() {
    return (
      <div className="Home">
        <aside className="rectangle" />
        <aside className="circle" />
        <div>
          <img src={loading} className="loader" />
        </div>
      </div>
    )
  }
}

export default LobbyLoading;
