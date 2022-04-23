import React from 'react';
import '../styles/Library.scss';

import Card from './Card';
import FeaturedCard from './FeaturedCard';
import LobbyLoading from '../LobbyLoading';

interface LibraryProps {
  close: () => void,
  import: () => void,
  updateImportId: (s: string) => void,
  startGame: (id: string) => void,
  importId: string,
  importError: boolean,
}

interface LibraryState {
  loaded: boolean,
  featured: DatabaseGame[],
  quizzes: DatabaseGame[],
  search: string,
  searchLoaded: boolean
}

class Library extends React.Component<LibraryProps, LibraryState> {
  constructor(props: LibraryProps) {
    super(props);

    this.state = {
      loaded: process.env.NODE_ENV === "development", // in development mode, don't connect to the API
      featured: [],
      quizzes: [],
      search: "",
      searchLoaded: false
    }

    this.search = this.search.bind(this);
  }

  componentDidMount() {
    if (!this.state.loaded) {
      Promise.all([
        fetch("/api/v1/recent")
          .then(res => res.json())
          .then((data: DatabaseGame[]) => {
            let quizzes = this.state.quizzes.concat(data);
            this.setState({ quizzes });
          }),
        fetch("/api/v1/featured")
          .then(res => res.json())
          .then((data: DatabaseGame[]) => this.setState({ featured: data }))
      ]).then(() => {
        this.setState({ loaded: true });
      })
    }
  }

  search(e: any) {
    let query: string = e.target.value;
    this.setState({
      search: query,
      searchLoaded: false
    });

    if (query.length > 0) {
      fetch("/api/v1/search", {
        method: "POST",
        body: JSON.stringify({ query })
      })
        .then(res => res.json())
        .then((data: DatabaseGame[]) => {
          this.setState({
            quizzes: data,
            searchLoaded: true
          });
        })
    } else {
      fetch("/api/v1/recent")
        .then(res => res.json())
        .then((data: DatabaseGame[]) => {
          this.setState({ quizzes: data });
        });
    }
  }

  render() {
    let searching = this.state.search.length !== 0;
    let searchLoaded = this.state.searchLoaded;

    if (this.state.loaded) {
      return (
        <div className="Library">
          <div className="header">
            <div>Browse Kascreeches</div>
            <input
              type="search"
              placeholder="Name or author"
              value={this.state.search}
              onChange={this.search} />
            <div onClick={this.props.import}>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="32" height="32"><path fill="none" d="M0 0h24v24H0z" /><path d="M12 12.586l4.243 4.242-1.415 1.415L13 16.415V22h-2v-5.587l-1.828 1.83-1.415-1.415L12 12.586zM12 2a7.001 7.001 0 0 1 6.954 6.194 5.5 5.5 0 0 1-.953 10.784v-2.014a3.5 3.5 0 1 0-1.112-6.91 5 5 0 1 0-9.777 0 3.5 3.5 0 0 0-1.292 6.88l.18.03v2.014a5.5 5.5 0 0 1-.954-10.784A7 7 0 0 1 12 2z" /></svg>
            </div>
          </div>

          {(this.state.search.length === 0 && this.state.featured.length !== 0) &&
            <div className="featured">
              <div>Featured</div>

              <div>
                {this.state.featured.map(quiz =>
                  <FeaturedCard
                    quiz={quiz}
                    key={quiz.id}
                    play={() => this.props.startGame(quiz.id)} />)
                }
              </div>
            </div>
          }

          <div className="all">
            <div>{this.state.search.length === 0 ? "Recently Created" : `Search Results for "${this.state.search}"`}</div>

            {!(searching && !searchLoaded) &&
              <div>
                {this.state.quizzes.map(quiz =>
                  <Card
                    quiz={quiz}
                    key={quiz.id}
                    play={() => this.props.startGame(quiz.id)} />)
                }

                {this.state.quizzes.length === 0 &&
                  "No quizzes found"
                }
              </div>
            }

            {(searching && !searchLoaded) &&
              <div>
                <LobbyLoading />
              </div>
            }
          </div>
        </div>
      )
    } else {
      return <LobbyLoading />
    }
  }
}

export default Library;