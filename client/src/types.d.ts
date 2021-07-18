interface Question {
  question: string,
  duration: number,
  answers: {
    text: string,
    correct: boolean
  }[];
}

interface LeaderboardEntry {
  userName: string,
  points: number,
  streak: number
}