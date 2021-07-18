import React from 'react';
import { AppPhase } from './App';
import './styles/Home.scss';

import logo from "./images/logo.png";

interface HomeProps {
  hostID: string,
  joinID: string,
  joinName: string,
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
    this.state = { tab: Tab.Play }
  }

  render() {
    if (this.state.tab === Tab.Play) {
      return (
        <div className="Home">
          <aside className="rectangle" />
          <aside className="circle" />
          <div>
            <img src={logo} alt="Kascreech logo" />
            <div>
              <div>
                <span className="active">Play</span>
                <span onClick={() => this.setState({ tab: Tab.Host })}>Host</span>
              </div>
              <form>
                <input
                  placeholder="Game ID"
                  value={this.props.joinID}
                  type="tel"
                  autoComplete="off"
                  onChange={(e) => this.props.setState({ joinID: e.target.value })} />
                <input
                  placeholder="User Name"
                  value={this.props.joinName}
                  onChange={(e) => this.props.setState({ joinName: e.target.value })} />
                <input
                  type="button"
                  value="Enter"
                  onClick={() => this.props.setState({ phase: AppPhase.Player })} />
              </form>
            </div>
          </div>
        </div>
      )
    } else {
      return (
        <div className="Home">
          <aside className="rectangle" />
          <aside className="circle" />
          <div>
            <img src={logo} alt="Kascreech logo" />
            <div>
              <div>
                <span onClick={() => this.setState({ tab: Tab.Play })}>Play</span>
                <span className="active">Host</span>
              </div>
              <form>
                <input
                  placeholder="Kahoot ID"
                  value={this.props.hostID}
                  onChange={(e) => this.props.setState({ hostID: e.target.value })} />
                <input
                  type="button"
                  value="Create"
                  onClick={() => this.props.setState({ phase: AppPhase.Host })} />
              </form>
            </div>
          </div>
        </div>
      )
    }
  }
}

/*
  <div>
    <h2>Host Game</h2>
    <input placeholder="Kahoot ID" value={this.props.hostID} onChange={e => this.props.setState({ hostID: e.target.value })} />
    <button onClick={() => this.props.setState({ phase: AppPhase.Host })}>Host</button>
  </div>

  <div>
    <h2>Join Game</h2>
    <input placeholder="Game ID" value={this.props.joinID} onChange={e => this.props.setState({ joinID: e.target.value })} />
    <input placeholder="Name" value={this.props.joinName} onChange={e => this.props.setState({ joinName: e.target.value })} />
    <button onClick={() => this.props.setState({ phase: AppPhase.Player })}>Join</button>
  </div>
*/

export default Home;
