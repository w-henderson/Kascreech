import React from 'react';
import Host from './host/Host';
import Player from './player/Player';
import './styles/App.scss';

export const SERVER_ADDR = "localhost";

enum AppPhase {
  Initial,
  Host,
  Player
}

interface AppState {
  phase: AppPhase,
  hostID: string,
  joinID: string,
  joinName: string
}

class App extends React.Component<{}, AppState> {
  websocket: WebSocket | undefined;

  constructor(props: {}) {
    super(props);
    this.reset = this.reset.bind(this);

    this.state = {
      phase: AppPhase.Initial,
      hostID: "",
      joinID: "",
      joinName: ""
    }
  }

  reset() {
    this.setState({
      phase: AppPhase.Initial,
      hostID: "",
      joinID: "",
      joinName: ""
    })
  }

  render() {
    if (this.state.phase === AppPhase.Initial) {
      return (
        <div>
          <div>
            <h2>Host Game</h2>
            <input placeholder="Kahoot ID" value={this.state.hostID} onChange={e => this.setState({ hostID: e.target.value })} />
            <button onClick={() => this.setState({ phase: AppPhase.Host })}>Host</button>
          </div>

          <div>
            <h2>Join Game</h2>
            <input placeholder="Game ID" value={this.state.joinID} onChange={e => this.setState({ joinID: e.target.value })} />
            <input placeholder="Name" value={this.state.joinName} onChange={e => this.setState({ joinName: e.target.value })} />
            <button>Join</button>
          </div>
        </div>
      )
    } else if (this.state.phase === AppPhase.Host) {
      return (
        <Host kahootID={this.state.hostID} onFailure={this.reset} />
      )
    } else if (this.state.phase === AppPhase.Player) {
      return (
        <Player />
      )
    }
  }
}

export default App;
