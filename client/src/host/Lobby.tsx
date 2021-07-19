import React from 'react';
import '../styles/Lobby.scss';

import logo from "../images/logo.png";

import { PersonRounded } from "@material-ui/icons";

interface LobbyProps {
  gameName: string,
  gameId: string,
  players: string[],
  startCallback: () => void
}

class Lobby extends React.Component<LobbyProps> {
  render() {
    return (
      <div className="Lobby">
        <header>
          <div>Join in with the fun by typing in <b>this code</b>!</div>
          <div>{this.props.gameId}</div>
        </header>

        <div className="top">
          <div>
            <PersonRounded />
            <span>{this.props.players.length}</span>
          </div>
          <div>
            <img src={logo} alt="Kascreech logo" />
          </div>
          <div>
            <button onClick={this.props.startCallback}>Start</button>
          </div>
        </div>

        {this.props.players.length > 0 &&
          <div className="names">
            {this.props.players.map((value, index) =>
              <div key={index}>{value}</div>
            )}
          </div>
        }

        {this.props.players.length === 0 &&
          <div className="empty">
            <div>
              Waiting for players...
            </div>
          </div>
        }
      </div>
    )
  }
}

export default Lobby;
