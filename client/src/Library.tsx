import React from 'react';
import './styles/Library.scss';

import Card from './library/Card';
import FeaturedCard from './library/FeaturedCard';

interface LibraryProps {
  close: () => void,
  updateImportId: (s: string) => void,
  startGame: (e: any) => void,
  importId: string,
  importError: boolean,
}

interface LibraryState {
  featured: QuizEntry[],
  quizzes: QuizEntry[],
}

class Library extends React.Component<LibraryProps, LibraryState> {
  constructor(props: LibraryProps) {
    super(props);

    this.state = {
      featured: [
        {
          id: "1",
          name: "La France",
          description: "Culture génerale française",
          author: "GarAlb",
          image: "https://media.kahoot.it/02add214-3f87-4472-bb78-81c14f9e03a4_opt",
          questions: 10,
          plays: 69,
          kahoot: true,
        },
        {
          id: "2",
          name: "Learn About Passover with an Extremely Long Title!",
          description: "Play along to learn about the Jewish holiday of Passover. #Passover #Jewish #JewishHolidays",
          author: "KahootStudio",
          image: "https://media.kahoot.it/fad9423e-996e-4cba-838f-861e9ffd3adc",
          questions: 10,
          plays: 69,
          kahoot: true,
        },
        {
          id: "3",
          name: "The Music of Tangled",
          description: "How well do you know the beloved music from Tangled? Don\u0027t be afraid to sing these timeless songs aloud! © Disney. All rights reserved. #disney #tangled #disneymusic",
          author: "Disney_Official",
          image: "https://media.kahoot.it/c6601554-888d-4647-bc2d-3545c9de1e1d",
          questions: 10,
          plays: 69,
          kahoot: true,
        }
      ],
      quizzes: [
        {
          id: "1",
          name: "La France",
          description: "Culture génerale française",
          author: "GarAlb",
          image: "https://media.kahoot.it/02add214-3f87-4472-bb78-81c14f9e03a4_opt",
          questions: 10,
          plays: 69,
          kahoot: true,
        },
        {
          id: "2",
          name: "Learn About Passover!",
          description: "Play along to learn about the Jewish holiday of Passover. #Passover #Jewish #JewishHolidays",
          author: "KahootStudio",
          image: "https://media.kahoot.it/fad9423e-996e-4cba-838f-861e9ffd3adc",
          questions: 10,
          plays: 69,
          kahoot: true,
        },
        {
          id: "3",
          name: "The Music of Tangled with a really long title that will need to be wrapped",
          description: "How well do you know the beloved music from Tangled? Don\u0027t be afraid to sing these timeless songs aloud! © Disney. All rights reserved. #disney #tangled #disneymusic",
          author: "Disney_Official",
          image: "https://media.kahoot.it/c6601554-888d-4647-bc2d-3545c9de1e1d",
          questions: 10,
          plays: 69,
          kahoot: true,
        }
      ],
    }
  }

  render() {
    return (
      <div>
        <div className="header">
          <div>Browse Kascreeches</div>
          <input type="search" placeholder="Name or author" />
        </div>

        <div className="featured">
          <div>Featured</div>

          <div>
            {this.state.featured.map(quiz => <FeaturedCard quiz={quiz} key={quiz.id} />)}
          </div>
        </div>

        <div className="all">
          <div>Browse All</div>

          <div>
            {this.state.quizzes.map(quiz => <Card quiz={quiz} key={quiz.id} />)}
          </div>
        </div>
      </div>
    )
  }
}

export default Library;

/*
<div>
  <span onClick={this.props.close}>Play</span>
  <span className="active">Host</span>
</div>
<form>
  <input
    placeholder="Kahoot ID"
    value={this.props.importId}
    className={this.props.importError ? "error" : undefined}
    onChange={(e) => this.props.updateImportId(e.target.value)} />
  <input
    type="submit"
    value="Create"
    onClick={(e) => this.props.startGame(e)} />
</form>
*/