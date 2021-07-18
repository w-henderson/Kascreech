import React from 'react';
import { SERVER_ADDR } from '../App';
import '../styles/Player.scss';

import LobbyLoading from '../LobbyLoading';
import PlayerQuestion from './PlayerQuestion';
import Result from "./Result";

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
  onFailure: (message: string) => void
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
    this.websocket = new WebSocket(`ws://${SERVER_ADDR}`);
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
      if (data.status === "success") {
        this.setState({ phase: PlayerPhase.Lobby });
      } else {
        this.props.onFailure(data.message);
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
        <div>
          <h2>You're in the lobby!</h2>
          You'll be playing as soon as the host starts the game.
        </div>
      )
    } else if (this.state.phase === PlayerPhase.Question) {
      return (
        <PlayerQuestion
          answerCount={this.state.answerCount}
          guessCallback={this.guess} />
      )
    } else if (this.state.phase === PlayerPhase.GuessedQuestion) {
      return (
        <div>Guess made!</div>
      )
    } else if (this.state.phase === PlayerPhase.Result && this.state.result) {
      return (
        <Result
          result={this.state.result}
          position={this.state.position} />
      )
    } else if (this.state.phase === PlayerPhase.End) {
      return (
        <div>
          <h2>You came in at number {this.state.position}!</h2>
          Nice work / better luck next time / touch grass / you need to revise more etc
        </div>
      )
    }
  }
}

export default Player;
