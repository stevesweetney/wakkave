/* @flow */

import * as React from 'react';
import { List, AutoSizer } from 'react-virtualized';
import UserPost from './post';
import { Vote } from '../../../build/frontend';

type State = {
    message: string,
};

type User = {
  id: number,
  username: string,
  karma: number,
  streak: number,
};

type Props = {
    posts: Array<any>,
    fetchPosts: () => void,
    createPostRequest: (content: string) => void,
    voteRequest: (n: number, vote: Vote) => void,
    logoutRequest: () => void,
    user: User,
};

export default class Feed extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      message: '',
    };
  }

  componentDidMount() {
    //this.props.fetchPosts();
  }

  render_row = ({ index, key, style }) => {
    const p = this.props.posts[index];
    return (
      <div style={style} key={key}>
        <UserPost
          key={p.id}
          id={p.id}
          content={p.content}
          isMine={this.props.user.id === p.userId}
          vote={p.vote}
          onVote={this.props.voteRequest}
        />
      </div>
    );
  }


  render_posts = () => (
    <AutoSizer>
      {({ width, height }) => (
        <List
          width={width}
          height={height}
          rowHeight={50}
          rowRenderer={this.render_row}
          rowCount={this.props.posts.length}
        />
      )}
    </AutoSizer>
  )

    update_username = (event: SyntheticInputEvent<>) => {
      this.setState({ message: event.target.value });
    }

    create_post_request = (e: SyntheticEvent<HTMLButtonElement>) => {
      e.preventDefault();

      this.props.createPostRequest(this.state.message);
      this.setState({ message: '' });
    }

    render_form = () => (
      <form>
        <div className="uk-flex uk-flex-middle uk-flex-column">
          <div className="uk-margin">
            <textarea
              className="uk-textarea uk-form-width-medium"
              rows="5"
              placeholder="Enter a message"
              value={this.state.message}
              onChange={this.update_username}
            />
          </div>
          <div className="uk-margin">
            <button
              className="uk-button uk-button-default"
              onClick={this.create_post_request}
              type="button"
            >
              {'Send!'}
            </button>
          </div>
        </div>
      </form>
    )

    render() {
      return (
        <div className="uk-container">
          <button
            className="uk-button uk-button-default uk-position-top-right"
            onClick={this.props.logoutRequest}
            type="button"
          >
                Logout
          </button>
          <p>
            {`Karma: ${this.props.user.karma}!`}
          </p>
          <p>
            {`Welcome ${this.props.user.username}!`}
          </p>
          <div className="uk-flex uk-flex-column" style={{ height: '400px' }}>
            <div className="uk-flex-1">
              {this.render_posts()}
            </div>
            {this.render_form()}
          </div>
        </div>
      );
    }
}
