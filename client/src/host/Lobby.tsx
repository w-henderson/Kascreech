import React from 'react';
import '../styles/Lobby.scss';

interface LobbyProps {
  gameName: string,
  gameId: string,
  players: string[],
  startCallback: () => void
}

class Lobby extends React.Component<LobbyProps> {
  render() {
    return (
      <div>
        <h2>{this.props.gameName} - Lobby</h2>
        Game ID: {this.props.gameId}

        <h2>Connected Players:</h2>
        {this.props.players.length > 0 ? this.props.players.map((player, index) => (
          <span key={index}>{player}<br /></span>
        )) : "None"}

        <h2>Start Game</h2>
        <button onClick={this.props.startCallback}>Start Game</button>
      </div>
    )
  }
}

export default Lobby;
