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

interface QuizEntry {
  id: string,
  name: string,
  author: string,
  description: string,
  questions: number,
  plays: number,
  image?: string,
  kahoot: boolean
}