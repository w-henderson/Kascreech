import React from 'react';
import '../styles/Player.scss';

import LobbyLoading from '../LobbyLoading';
import PlayerQuestion from './PlayerQuestion';
import PlayerLobby from './PlayerLobby';
import Result from "./Result";
import GuessMade from './GuessMade';
import Finish from './Finish';

enum PlayerPhase {
  AwaitingLobby,
  Lobby,
  Question,
  GuessedQuestion,
  Result,
  End
}

export interface ResultType {
  correct: boolean,
  pointsThisRound: number,
  pointsTotal: number,
  streak: number,
  behind: string | null
}

interface PlayerProps {
  gameId: string,
  userName: string,
  onFailure: (errorType: string, message: string) => void
}

interface PlayerState {
  phase: PlayerPhase,
  answerCount: number,
  position: number,
  result: ResultType | undefined;
}

class Player extends React.Component<PlayerProps, PlayerState> {
  websocket: WebSocket | undefined;

  constructor(props: PlayerProps) {
    super(props);
    this.guess = this.guess.bind(this);
    this.state = {
      phase: PlayerPhase.AwaitingLobby,
      answerCount: 0,
      position: 0,
      result: undefined
    }
  }

  componentDidMount() {
    let websocketAddr = (window.location.protocol === "https:" ? "wss" : "ws")
      + `://${window.location.host}`;

    this.websocket = new WebSocket(websocketAddr);
    this.websocket.onmessage = this.wsHandler.bind(this);
    this.websocket.onopen = () => {
      this.websocket?.send(JSON.stringify({
        command: "join",
        gameId: this.props.gameId,
        playerName: this.props.userName
      }));
    }
  }

  guess(index: number) {
    this.websocket?.send(JSON.stringify({
      command: "guess",
      index
    }));
    this.setState({ phase: PlayerPhase.GuessedQuestion });
  }

  wsHandler(e: MessageEvent<any>) {
    let data = JSON.parse(e.data);
    if (this.state.phase === PlayerPhase.AwaitingLobby) {
      if (data.success === true) {
        this.setState({ phase: PlayerPhase.Lobby });
      } else {
        this.props.onFailure(data.errorType, data.errorMessage);
      }
    } else if (this.state.phase === PlayerPhase.Lobby || this.state.phase === PlayerPhase.Result) {
      if (data.event === "questionStart") {
        this.setState({
          phase: PlayerPhase.Question,
          answerCount: data.numberOfAnswers
        });
      } else if (data.event === "end") {
        this.setState({
          phase: PlayerPhase.End,
          position: data.position
        });
      }
    } else if (this.state.phase === PlayerPhase.Question || this.state.phase === PlayerPhase.GuessedQuestion) {
      this.setState({
        phase: PlayerPhase.Result,
        position: data.position,
        result: {
          correct: data.correct,
          pointsThisRound: data.pointsThisRound,
          pointsTotal: data.pointsTotal,
          streak: data.streak,
          behind: data.behind
        }
      });
    }
  }

  render() {
    if (this.state.phase === PlayerPhase.AwaitingLobby) {
      return (
        <LobbyLoading />
      )
    } else if (this.state.phase === PlayerPhase.Lobby) {
      return (
        <PlayerLobby />
      )
    } else if (this.state.phase === PlayerPhase.Question) {
      return (
        <PlayerQuestion
          answerCount={this.state.answerCount}
          guessCallback={this.guess} />
      )
    } else if (this.state.phase === PlayerPhase.GuessedQuestion) {
      return (
        <GuessMade />
      )
    } else if (this.state.phase === PlayerPhase.Result && this.state.result) {
      return (
        <Result
          result={this.state.result}
          position={this.state.position} />
      )
    } else if (this.state.phase === PlayerPhase.End) {
      return (
        <Finish position={this.state.position} />
      )
    }
  }
}

export default Player;
