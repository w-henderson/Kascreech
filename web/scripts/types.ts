type Answers = number[];

interface Question {
    question: string,
    responses: string[],
    correct: number[]
}

interface GenerateGame {
    answers: Answers[],
    timePerQuestion: number,
    timeShowingAnswers: number,
    timeShowingLeaderboard: number,
    gameStartTime: number
}

interface Leaderboard {
    players: {
        uuid: string,
        username: string,
        score: number
    }[]
}

interface ChungusGameInfo {
    bigChungus: boolean,
    gameId: string,
    questions: Question[],
    timePerQuestion: number,
    timeShowingAnswers: number,
    timeShowingLeaderboard: number,
    gameStartTime: number
}