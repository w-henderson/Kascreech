import React, { FormEvent } from 'react';
import { AppPhase, KascreechError } from './App';
import './styles/Home.scss';

import logo from "./images/logo.png";

interface HomeProps {
  hostID: string,
  joinID: string,
  joinName: string,
  error: KascreechError | undefined,
  setState: (newState: any) => void
}

enum Tab {
  Play,
  Host
}

interface HomeState {
  tab: Tab
}

class Home extends React.Component<HomeProps, HomeState> {
  constructor(props: HomeProps) {
    super(props);

    if (this.props.error === KascreechError.UreqError) {
      this.state = { tab: Tab.Host }
    } else {
      this.state = { tab: Tab.Play }
    }

    this.switchTab = this.switchTab.bind(this);
    this.startGame = this.startGame.bind(this);
  }

  switchTab(tab: Tab) {
    this.setState({ tab });
    this.props.setState({ error: undefined });
  }

  startGame(e: FormEvent, mode: AppPhase) {
    e.preventDefault();
    this.props.setState({ phase: mode, error: undefined });
  }

  render() {
    if (this.state.tab === Tab.Play) {
      return (
        <div className="Home">
          <img src={logo} alt="Kascreech logo" />

          <div className="form">
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
                className={this.props.error === KascreechError.NameAlreadyExists ? "error" : undefined}
                onChange={(e) => this.props.setState({ joinName: e.target.value, error: undefined })} />
              <input
                type="submit"
                value="Enter"
                onClick={(e) => this.startGame(e, AppPhase.Player)} />
            </form>
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
    } else {
      return (
        <div className="Home">
          <img src={logo} alt="Kascreech logo" />

          <div className="form">
            <div>
              <span onClick={() => this.switchTab(Tab.Play)}>Play</span>
              <span className="active">Host</span>
            </div>
            <form>
              <input
                placeholder="Kahoot ID"
                value={this.props.hostID}
                className={this.props.error === KascreechError.UreqError ? "error" : undefined}
                onChange={(e) => this.props.setState({ hostID: e.target.value, error: undefined })} />
              <input
                type="submit"
                value="Create"
                onClick={(e) => this.startGame(e, AppPhase.Host)} />
            </form>
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
}

export default Home;
