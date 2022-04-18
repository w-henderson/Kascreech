import React from 'react';

interface FeaturedCardProps {
  quiz: QuizEntry
}

class FeaturedCard extends React.Component<FeaturedCardProps> {
  render() {
    return (
      <div className="featuredCard">
        <img src={this.props.quiz.image} alt={this.props.quiz.name} />
        <div>
          <div className="info">
            <h3>{this.props.quiz.name}</h3>
            <p>{this.props.quiz.author}</p>
          </div>

          <div className="stats">
            <div>
              <span>{this.props.quiz.questions}</span>
              <span>rounds</span>
            </div>

            <div>
              <span>{this.props.quiz.plays}</span>
              <span>plays</span>
            </div>
          </div>

          <div className="play">
            &gt;
          </div>
        </div>
      </div>
    )
  }
}

export default FeaturedCard;
