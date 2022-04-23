import React, { FormEvent } from 'react';
import { AppPhase, KascreechError } from './App';
import './styles/Home.scss';
import './styles/Library.scss';

import logo from "./images/logo.png";
import Library from './library/Library';
import Importer from './library/Importer';

interface HomeProps {
  hostID: string,
  joinID: string,
  joinName: string,
  error: KascreechError | undefined,
  setState: (newState: any) => void
}

enum Tab {
  Play,
  Host,
  Import
}

interface HomeState {
  tab: Tab
}

class Home extends React.Component<HomeProps, HomeState> {
  constructor(props: HomeProps) {
    super(props);

    if (this.props.error === KascreechError.KahootGameNotFound) {
      this.state = { tab: Tab.Host }
    } else {
      this.state = { tab: Tab.Play }
    }

    this.switchTab = this.switchTab.bind(this);
    this.startGame = this.startGame.bind(this);
    this.hostGame = this.hostGame.bind(this);
    this.importGame = this.importGame.bind(this);
  }

  switchTab(tab: Tab) {
    if (tab === Tab.Host && (window.innerWidth < 1100 || window.innerHeight < 780)) {
      alert("The window is too small to host a Kahoot game. Please use a computer or rotate your tablet to landscape mode.");
      return;
    }

    this.setState({ tab });
    this.props.setState({ error: undefined });
  }

  startGame(e: FormEvent, mode: AppPhase) {
    e.preventDefault();
    this.props.setState({ phase: mode, error: undefined });
  }

  hostGame(id: string) {
    this.props.setState({
      phase: AppPhase.Host,
      hostID: id,
      error: undefined
    });
  }

  importGame(id: string) {
    this.setState({ tab: Tab.Host });
  }

  render() {
    let inner = <></>;

    if (this.state.tab === Tab.Play) {
      inner = (
        <div>
          <div>
            <span className="active">Play</span>
            <span onClick={() => this.switchTab(Tab.Host)}>Host</span>
          </div>
          <form>
            <input
              placeholder="Game ID"
              value={this.props.joinID}
              type="tel"
              autoComplete="off"
              className={this.props.error === KascreechError.GameNotFound ? "error" : undefined}
              onChange={(e) => this.props.setState({ joinID: e.target.value, error: undefined })} />
            <input
              placeholder="User Name"
              value={this.props.joinName}
              className={this.props.error === KascreechError.UsernameAlreadyExists ? "error" : undefined}
              onChange={(e) => this.props.setState({ joinName: e.target.value, error: undefined })} />
            <input
              type="submit"
              value="Enter"
              onClick={(e) => this.startGame(e, AppPhase.Player)} />
          </form>
        </div>
      );
    } else if (this.state.tab === Tab.Host) {
      inner = (
        <Library
          close={() => this.switchTab(Tab.Play)}
          import={() => this.switchTab(Tab.Import)}
          updateImportId={(id) => this.props.setState({ hostID: id, error: undefined })}
          startGame={this.hostGame}
          importId={this.props.hostID}
          importError={this.props.error === KascreechError.KahootGameNotFound} />
      );
    } else if (this.state.tab === Tab.Import) {
      inner = (
        <Importer
          back={() => this.switchTab(Tab.Host)}
          imported={this.importGame} />
      )
    }

    let className = "regular";
    if (this.state.tab === Tab.Host) className = "library";
    if (this.state.tab === Tab.Import) className = "import";

    return (
      <div className="Home">
        <img
          src={logo}
          alt="Kascreech logo"
          className="logo"
          onClick={() => this.setState({ tab: Tab.Play })} />

        <div className={`form ${className}`}>
          {inner}
        </div>

        <footer>
          <span>
            <a href="/terms.html" target="_blank" rel="noreferrer">Terms</a>|
            <a href="/privacy.html" target="_blank" rel="noreferrer">Privacy</a>|
            <a href="https://github.com/w-henderson/Kascreech" target="_blank" rel="noreferrer">GitHub</a>
          </span>
        </footer>
      </div>
    )
  }
}

export default Home;
