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
    // this.props.fetchPosts();
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

    handle_message_change = (event: SyntheticInputEvent<>) => {
      const text = event.target.value;
      if (text.length > 140) { return; }
      this.setState({ message: text });
    }

    create_post_request = (e: SyntheticEvent<HTMLButtonElement>) => {
      e.preventDefault();

      this.props.createPostRequest(this.state.message);
      this.setState({ message: '' });
    }

    render_form = () => (
      <form>
        <div className="uk-flex uk-flex-middle uk-flex-row">
          <div className="uk-margin" style={{ position: 'relative', width: '80%' }}>
            <textarea
              className="uk-textarea feed-message-input"
              rows="5"
              placeholder="Enter a message"
              value={this.state.message}
              onChange={this.handle_message_change}
            />
            <button
              className="uk-button uk-button-default"
              onClick={this.create_post_request}
              type="button"
              style={{
                position: 'absolute', float: 'right', top: 0, height: '100%',
              }}
            >
              {'Send!'}
            </button>
          </div>
        </div>
      </form>
    )

    render() {
      return (
        <div className="uk-container uk-flex uk-flex-column feed-content">
          <a href="#help-modal" uk-toggle="target: #help-modal" className="uk-position-bottom-left" uk-icon="info" />
          <div id="help-modal" uk-modal="true">
            <div className="uk-modal-dialog uk-modal-body">
              <h2 className="uk-modal-title">Help</h2>
              <p>
                  In Wakkave, your messages will expire after 1 hour. While they are still alive
                  you can vote to decide which way the majority will go.
                  If you guess correctly you'll gain points, if not you'll lose points.
                  Have fun!
              </p>
              <button className="uk-modal-close-default" type="button" uk-close="true" />
            </div>
          </div>
          <div className="edge-content uk-flex uk-flex-row uk-flex-between">
            <div className="uk-text-large" uk-tooltip="title: Karma; pos: right">
              {`Karma: ${this.props.user.karma}!`}
            </div>
            <div className="uk-text-large">
              {`${this.props.user.username}`}
            </div>
            <button
              className="uk-button uk-button-default"
              onClick={this.props.logoutRequest}
              type="button"
            >
                  Logout
            </button>
          </div>
          <div className="center-content">
            {this.render_posts()}
          </div>
          <div className="edge-content">
            {this.render_form()}
          </div>
        </div>
      );
    }
}
