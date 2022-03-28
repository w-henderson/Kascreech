import React from 'react';
import './styles/App.scss';

import Home from './Home';
import Host from './host/Host';
import Player from './player/Player';

export const SERVER_ADDR = process.env.REACT_APP_WS_ADDR;

export enum AppPhase {
  Initial,
  Host,
  Player
}

export enum KascreechError {
  FailedRead,
  GameNotFound,
  KahootGameNotFound,
  UsernameAlreadyExists,
  InvalidCommand,
  UnknownError,
}

interface AppState {
  phase: AppPhase,
  error: KascreechError | undefined,
  hostID: string,
  joinID: string,
  joinName: string
}

class App extends React.Component<{}, AppState> {
  websocket: WebSocket | undefined;

  constructor(props: {}) {
    super(props);
    this.error = this.error.bind(this);

    this.state = {
      phase: AppPhase.Initial,
      error: undefined,
      hostID: "",
      joinID: "",
      joinName: ""
    }
  }

  error(errorType: string, message: string) {
    let error = stringToError(errorType);

    if (error === KascreechError.GameNotFound
      || error === KascreechError.KahootGameNotFound
      || error === KascreechError.UsernameAlreadyExists) {
      this.setState({
        phase: AppPhase.Initial,
        error,
        hostID: "",
        joinID: "",
        joinName: ""
      })
    } else {
      alert(`${errorType}: ${message}`);

      this.setState({
        phase: AppPhase.Initial,
        hostID: "",
        joinID: "",
        joinName: ""
      })
    }
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
            error={this.state.error}
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
            onFailure={this.error} />
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
            onFailure={this.error} />
        </div>
      )
    }
  }
}

function stringToError(str: string): KascreechError {
  switch (str) {
    case "FailedRead": return KascreechError.FailedRead;
    case "GameNotFound": return KascreechError.GameNotFound;
    case "KahootGameNotFound": return KascreechError.KahootGameNotFound;
    case "UsernameAlreadyExists": return KascreechError.UsernameAlreadyExists;
    case "InvalidCommand": return KascreechError.InvalidCommand;
    default: return KascreechError.UnknownError;
  }
}

export default App;
