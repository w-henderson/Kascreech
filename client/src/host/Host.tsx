import React from 'react';
import { SERVER_ADDR } from '../App';
import '../styles/Host.scss';

import Lobby from "./Lobby";
import HostQuestion from './HostQuestion';
import Leaderboard from './Leaderboard';
import LobbyLoading from '../LobbyLoading';

enum HostPhase {
  AwaitingLobby,
  Lobby,
  AwaitingQuestion,
  Question,
  Answer,
  Leaderboard
}

interface HostProps {
  kahootID: string,
  onFailure: (message: string) => void
}

interface HostState {
  gameId: string | undefined,
  gameName: string | undefined,
  questionCount: number | undefined,
  questionNumber: number,
  lobbyPlayers: string[],
  phase: HostPhase,
  currentQuestion: Question | undefined,
  leaderboard: LeaderboardEntry[]
}

class Host extends React.Component<HostProps, HostState> {
  websocket: WebSocket | undefined;

  constructor(props: HostProps) {
    super(props);
    this.state = {
      gameId: undefined,
      gameName: undefined,
      questionNumber: 0,
      questionCount: undefined,
      lobbyPlayers: [],
      phase: HostPhase.AwaitingLobby,
      currentQuestion: undefined,
      leaderboard: []
    }

    this.startGame = this.startGame.bind(this);
    this.nextQuestion = this.nextQuestion.bind(this);
    this.endQuestion = this.endQuestion.bind(this);
    this.showLeaderboard = this.showLeaderboard.bind(this);
  }

  componentDidMount() {
    this.websocket = new WebSocket(`ws://${SERVER_ADDR}`);
    this.websocket.onmessage = this.wsHandler.bind(this);
    this.websocket.onopen = () => {
      this.websocket?.send(JSON.stringify({
        command: "host",
        id: this.props.kahootID
      }));
    }
  }

  startGame() {
    this.websocket?.send(JSON.stringify({
      command: "start"
    }));
    this.setState({
      phase: HostPhase.AwaitingQuestion,
      questionNumber: this.state.questionNumber + 1
    });
  }

  nextQuestion() {
    this.websocket?.send(JSON.stringify({
      command: "question"
    }));
    this.setState({
      phase: HostPhase.AwaitingQuestion,
      questionNumber: this.state.questionNumber + 1
    })
  }

  endQuestion() {
    this.websocket?.send(JSON.stringify({
      command: "leaderboard"
    }));
    this.setState({
      phase: HostPhase.Answer
    });
  }

  showLeaderboard() {
    this.setState({
      phase: HostPhase.Leaderboard
    })
  }

  wsHandler(e: MessageEvent<any>) {
    let data = JSON.parse(e.data);
    if (this.state.phase === HostPhase.AwaitingLobby) {
      if (data.status === "success") {
        this.setState({
          gameId: data.gameId,
          gameName: data.gameName,
          questionCount: data.questionCount,
          phase: HostPhase.Lobby
        })
      } else {
        this.websocket?.close();
        this.props.onFailure(data.message);
      }
    } else if (this.state.phase === HostPhase.Lobby) {
      if (data.event === "newPlayer") {
        let lobbyPlayers = this.state.lobbyPlayers;
        lobbyPlayers.push(data.playerName);
        this.setState({ lobbyPlayers });
      }
    } else if (this.state.phase === HostPhase.AwaitingQuestion) {
      this.setState({
        currentQuestion: data,
        phase: HostPhase.Question
      });
    } else if (this.state.phase === HostPhase.Answer) {
      this.setState({
        leaderboard: data.leaderboard
      })
    }
  }

  render() {
    if (this.state.phase === HostPhase.AwaitingLobby) {
      return (
        <LobbyLoading />
      )
    }
    else if (this.state.phase === HostPhase.AwaitingQuestion) {
      return (
        <div>Loading...</div>
      )
    } else if (this.state.phase === HostPhase.Lobby) {
      return (
        <Lobby
          gameName={this.state.gameName ?? ""}
          gameId={this.state.gameId ?? ""}
          players={this.state.lobbyPlayers}
          startCallback={this.startGame} />
      )
    } else if ((this.state.phase === HostPhase.Question || this.state.phase === HostPhase.Answer) && this.state.currentQuestion) {
      return (
        <HostQuestion
          question={this.state.currentQuestion}
          questionNumber={this.state.questionNumber}
          showCorrect={this.state.phase === HostPhase.Answer}
          showAnswersCallback={this.endQuestion}
          showLeaderboardCallback={this.showLeaderboard} />
      )
    } else if (this.state.phase === HostPhase.Leaderboard) {
      return (
        <Leaderboard
          leaderboard={this.state.leaderboard}
          isLastQuestion={this.state.questionCount === this.state.questionNumber}
          nextQuestionCallback={this.nextQuestion} />
      )
    }
  }
}

export default Host;
