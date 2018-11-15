/* @flow */

import * as React from 'react';
import { Vote } from '../../../build/frontend';

type State = {
    vote: Vote
}

type Props = {
    content: string,
    vote: string,
    id: number,
    onVote: (n: number, vote: Vote) => void,
    isMine: boolean,
}

export default class UserPost extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    let v;

    if (this.props.vote === 'Up') {
      v = Vote.Up;
    } else if (this.props.vote === 'Down') {
      v = Vote.Down;
    } else {
      v = Vote.None;
    }

    this.state = {
      vote: v,
    };
  }

    handle_vote = (vote: Vote) => {
      if (this.state.vote !== vote) {
        this.props.onVote(this.props.id, vote);
        this.setState({ vote });
      }
    }

    handle_downvote = () => {
      this.handle_vote(Vote.Down);
    }

    handle_upvote = () => {
      this.handle_vote(Vote.Up);
    }

    get_style = (vote: Vote) => {
      if (this.state.vote === vote) {
        return ({ color: '#f45b69' });
      }
      return ({ color: '#999' });
    }

    get_style_up = () => this.get_style(Vote.Up)

    get_style_down = () => this.get_style(Vote.Down)

    render() {
      const bgColor = this.props.isMine ? '#1e87f0' : '#e3e3e3';
      const color = this.props.isMine ? '#fff' : '#361f27';
      const messageStyle = {
        color,
        padding: '5px 5px 5px 5px',
        backgroundColor: bgColor,
      };
      return (
        <li className="uk-flex uk-flex-left uk-flex-stretch uk-flex-row">
          <div className="uk-flex-inline uk-flex-column">
            <span
              uk-icon="icon: arrow-up"
              onClick={this.handle_upvote}
              style={this.get_style_up()}
              role="button"
            />
            <span
              uk-icon="icon: arrow-down"
              onClick={this.handle_downvote}
              style={this.get_style_down()}
              role="button"
            />
          </div>
          <span className="uk-border-rounded" style={messageStyle}>{this.props.content}</span>
        </li>
      );
    }
}
