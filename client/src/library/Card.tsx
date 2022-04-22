import React from 'react';

interface CardProps {
  quiz: DatabaseGame,
  play: () => void
}

class Card extends React.Component<CardProps> {
  render() {
    return (
      <div className="card">
        <img src={this.props.quiz.image} alt={this.props.quiz.name} />
        <div>
          <div className="info">
            <h3>{this.props.quiz.name}</h3>
            <p>{this.props.quiz.author}</p>
          </div>

          <div className="play" onClick={this.props.play}>
            Play Now!
          </div>
        </div>
      </div>
    )
  }
}

export default Card;
