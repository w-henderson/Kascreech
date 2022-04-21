import React from 'react';
import './styles/Library.scss';

import Card from './library/Card';
import FeaturedCard from './library/FeaturedCard';
import LobbyLoading from './LobbyLoading';

interface LibraryProps {
  close: () => void,
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
  searchLoaded: boolean,
  offset: number
}

class Library extends React.Component<LibraryProps, LibraryState> {
  constructor(props: LibraryProps) {
    super(props);

    this.state = {
      loaded: false,
      featured: [],
      quizzes: [],
      search: "",
      searchLoaded: false,
      offset: 0
    }

    this.search = this.search.bind(this);
  }

  componentDidMount() {
    if (!this.state.loaded) {
      Promise.all([
        fetch("/api/v1/list", {
          method: "POST",
          body: JSON.stringify({
            offset: this.state.offset,
          })
        })
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
      fetch("/api/v1/list", {
        method: "POST",
        body: JSON.stringify({
          offset: this.state.offset,
        })
      })
        .then(res => res.json())
        .then((data: DatabaseGame[]) => {
          this.setState({ quizzes: data, offset: 0 });
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
          </div>

          {this.state.search.length === 0 &&
            <div className="featured">
              <div>Featured</div>

              <div>
                {this.state.featured.map(quiz => <FeaturedCard quiz={quiz} key={quiz.id} />)}
              </div>
            </div>
          }

          <div className="all">
            <div>{this.state.search.length === 0 ? "Browse All" : `Search Results for "${this.state.search}"`}</div>

            {!(searching && !searchLoaded) &&
              <div>
                {this.state.quizzes.map(quiz => <Card quiz={quiz} key={quiz.id} />)}

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