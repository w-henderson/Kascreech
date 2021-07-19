import React from 'react';
import './styles/App.scss';

import Home from './Home';
import Host from './host/Host';
import Player from './player/Player';

export const SERVER_ADDR = "localhost";

export enum AppPhase {
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

  reset(message: string) {
    alert(message);
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
        <div className="App">
          <aside className="rectangle" />
          <aside className="circle" />
          <Home
            joinID={this.state.joinID}
            joinName={this.state.joinName}
            hostID={this.state.hostID}
            setState={(newState: any) => this.setState(newState)} />
        </div>
      )
    } else if (this.state.phase === AppPhase.Host) {
      return (
        <div className="App">
          <aside className="rectangle" />
          <aside className="circle" />
          <Host
            kahootID={this.state.hostID}
            onFailure={this.reset} />
        </div>
      )
    } else if (this.state.phase === AppPhase.Player) {
      return (
        <div className="App">
          <aside className="rectangle" />
          <aside className="circle" />
          <Player
            gameId={this.state.joinID}
            userName={this.state.joinName}
            onFailure={this.reset} />
        </div>
      )
    }
  }
}

export default App;
