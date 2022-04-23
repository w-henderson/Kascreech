import React from 'react';
import LobbyLoading from '../LobbyLoading';
import '../styles/Importer.scss';

interface ImporterProps {
  back: () => void,
  imported: (id: string) => void,
}

interface ImporterState {
  kahootId: string,
  importing: boolean
}

class Importer extends React.Component<ImporterProps, ImporterState> {
  constructor(props: ImporterProps) {
    super(props);

    this.state = {
      kahootId: "",
      importing: false
    };

    this.importGame = this.importGame.bind(this);
    this.uploadGame = this.uploadGame.bind(this);
  }

  importGame() {
    let uuidRegex = /[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/;
    let id = (this.state.kahootId.match(uuidRegex) ?? [null])[0];

    // TODO: better error handling
    if (id === null) {
      alert("No valid Kahoot ID found in the URL.");
      return;
    }

    this.setState({ importing: true }, () => {
      fetch("/api/v1/import", {
        method: "POST",
        body: JSON.stringify({
          id: this.state.kahootId
        })
      })
        .then(res => res.json())
        .then(data => this.props.imported(data.gameId));
    });
  }

  uploadGame() {
    alert("Coming Soon");
  }

  render() {
    return (
      <div className="Importer">
        <div>
          <h1>Import/Upload Game</h1>
        </div>

        {!this.state.importing &&
          <div>
            <input
              placeholder="Kahoot URL"
              value={this.state.kahootId}
              autoComplete="off"
              onChange={(e) => this.setState({ kahootId: e.target.value })} />
            <input
              type="button"
              value="Import from Link"
              onClick={this.importGame} />
            <input
              type="button"
              value="Upload File from PC"
              onClick={this.uploadGame} />
            <input
              type="button"
              value="Back to Library"
              onClick={this.props.back} />
          </div>
        }

        {this.state.importing &&
          <LobbyLoading />
        }
      </div>
    )
  }
}

export default Importer;
