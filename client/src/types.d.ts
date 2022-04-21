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

interface DatabaseGame {
  id: string,
  name: string,
  description: string,
  author: string,
  image?: string,
  questions: DatabaseQuestion[],
  plays: number,
  kahoot: boolean,
  featured: boolean
}

interface DatabaseQuestion {
  question: string,
  time: number,
  choices: DatabaseAnswer[]
}

interface DatabaseAnswer {
  answer: string,
  correct: boolean
}