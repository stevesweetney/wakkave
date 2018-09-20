/* @flow */

import * as React from 'react';
import ReactDOM from 'react-dom';
import {
  BrowserRouter as Router,
  Route,
  Redirect,
} from 'react-router-dom';
import Cookies from 'js-cookie';
import Sockette from 'sockette';
import Login from './components/login';
import Feed from './components/feed';

import { ProtocolInterface, WsMessage, Vote } from '../../build/frontend';

const SESSION_TOKEN: string = 'SessionToken';

const root = document.getElementById('root');

type User = {
    id: number,
    username: string,
    karma: number,
    streak: number,
};

type State = {
  ws: any,
  is_authenticated: boolean,
  is_loading: boolean,
  is_connected: boolean,
  posts: Array<any>,
  user: User,
};

class App extends React.Component<{protocolService: ProtocolInterface}, State> {
  constructor(props) {
    super(props);
    const ws = new Sockette('ws://127.0.0.1:8088', {
      timeout: 5e3,
      maxAttempts: 10,
      onopen: this.handle_on_open,
      onmessage: this.handle_message,
      onreconnect: e => console.log('Reconnecting...', e),
      onmaximum: e => console.log('Stop Attempting!', e),
      onclose: e => console.log('Closed!', e),
      onerror: e => console.log('Error:', e),
    });
    this.state = {
      ws,
      is_authenticated: false,
      is_loading: true,
      is_connected: false,
      posts: [],
      user: {
        id: -1,
        username: '',
        karma: -1,
        streak: -1,
      },
    };
  }

  handle_on_open = (e) => {
    e.target.binaryType = 'arraybuffer';
    this.setState({ is_loading: false, is_connected: true });
    const token = Cookies.get(SESSION_TOKEN);
    console.log(token);

    if (token) {
      const token_data = this.props.protocolService.write_login_token(token);
      console.log(token_data);
      this.state.ws.send(token_data);
    } else {
      Cookies.remove(SESSION_TOKEN);
      console.log('No token found');
    }
  }

  handle_message = (e) => {
    const data = new Uint8Array(e.data);
    const { protocolService } = this.props;
    const message_type = protocolService.response_type(data);
    console.log('recieving message from ws: ', message_type);

    switch (message_type) {
      case WsMessage.Login: {
        const login_res = protocolService.read_login(data);

        if (login_res) {
          Cookies.set(SESSION_TOKEN, login_res.token);
          this.setState({ user: login_res.user, is_authenticated: true });
        } else if (!this.state.is_authenticated) {
          Cookies.remove(SESSION_TOKEN);
          UIkit.notification(
            'An error occured when attempting to login',
            'warning',
          );
        }
        break; }
      case WsMessage.Logout:
        switch (protocolService.read_logout(data)) {
          case false:
            UIkit.notification(
              'An error occured when attempting to logout',
              'warning',
            );
            break;
          default:
            this.setState({ is_authenticated: false });
        }
        break;
      case WsMessage.FetchPosts: {
        const fetch_res = protocolService.read_fetch_posts(data);

        if (fetch_res) {
          console.debug('Posts: ', fetch_res.posts);
          Cookies.set(SESSION_TOKEN, fetch_res.token);
          this.setState({ posts: fetch_res.posts });
        } else {
          UIkit.notification(
            'An error occured when attempting to fetching posts',
            'warning',
          );
        }
        break; }
      case WsMessage.CreatePost: {
        const post_res = protocolService.read_create_post(data);

        if (post_res) {
          Cookies.set(SESSION_TOKEN, post_res.token);
          console.log('Adding a new post', post_res.post);
          this.setState(prevState => ({
            posts: [...prevState.posts, post_res.post],
          }));
        } else {
          UIkit.notification(
            'An error occured when attempting to create a post',
            'warning',
          );
        }
        break; }
      case WsMessage.UserVote: {
        const vote_res = protocolService.read_user_vote(data);

        if (vote_res) {
          Cookies.set(SESSION_TOKEN, vote_res);
        } else {
          UIkit.notification('An error occured when attempting to vote on a post');
        }
        break; }
      case WsMessage.InvalidPosts: {
        const invalid_post_ids = protocolService.read_invalid_posts(data);

        if (invalid_post_ids) {
          console.log("Recieved invalid ID's");
          this.setState((prevState) => {
            const { posts } = prevState;
            const updated_posts = [];
            for (let i = 0; i < posts.length; i += 1) {
              const p = posts[i];
              if (invalid_post_ids.indexOf(p.id) < 0) {
                updated_posts.push(p);
              }
            }
            return { posts: updated_posts };
          });
        }
        break; }
      case WsMessage.NewPost: {
        const new_post = protocolService.read_new_post(data);
        console.log('Recieving a new post...');
        if (new_post) {
          console.log('Recieved a new post: ', new_post);

          this.setState(prevState => ({
            posts: [...prevState.posts, new_post],
          }));
        }
        break; }
      case WsMessage.UpdateUsers:
      case WsMessage.Error:
      default:
    }
  }

  handle_login_creds = (name: string, password: string) => {
    const creds_data = this.props.protocolService.write_login_creds(name, password);

    if (creds_data) {
      this.state.ws.send(creds_data);
    }
  }

  handle_register = (name: string, password: string) => {
    const creds_data = this.props.protocolService.write_registration(name, password);
    console.log(creds_data);

    if (creds_data) {
      this.state.ws.send(creds_data);
    }
  }

  handle_logout = () => {
    const token = Cookies.get(SESSION_TOKEN);
    if (token) {
      console.log('Logout request to server');
      const data = this.props.protocolService.write_logout_token(token);
      if (data) {
        console.log('logout message converted to binary: ', data);
        this.state.ws.send(data);
      }
    }
  }

  create_post_request = (message: string) => {
    const token = Cookies.get(SESSION_TOKEN);
    if (token) {
      console.log('Sending message to server: ', message);
      const data = this.props.protocolService.write_create_post(token, message);
      if (data) {
        console.log('message converted to binary: ', data);
        this.state.ws.send(data);
      }
    }
  }

  vote_request = (id: number, vote: Vote) => {
    const token = Cookies.get(SESSION_TOKEN);
    if (token) {
      console.log('Sending vote to server: ', vote);
      const data = this.props.protocolService.write_user_vote(token, id, vote);
      if (data) {
        this.state.ws.send(data);
      }
    }
  }

  fetch_posts = () => {
    const token = Cookies.get(SESSION_TOKEN);
    if (token) {
      const token_data = this.props.protocolService.write_fetch_posts(token);
      this.state.ws.send(token_data);
    }
  }

  render() {
    const { is_loading, is_authenticated, user } = this.state;
    return (
      is_loading
        ? (<div className="uk-position-center" uk-spinner="" />)
        : (
          <Router>
            <div>
              <Route
                exact
                path="/"
                render={props => (
                  <Login
                    {...props}
                    loginRequest={this.handle_login_creds}
                    registerRequest={this.handle_register}
                    isAuth={is_authenticated}
                  />
                )}
              />
              <Route path="/error" component={Error} />
              <PrivateRoute
                path="/feed"
                component={Feed}
                isAuth={is_authenticated}
                posts={this.state.posts}
                fetchPosts={this.fetch_posts}
                createPostRequest={this.create_post_request}
                voteRequest={this.vote_request}
                logoutRequest={this.handle_logout}
                user={user}
              />
            </div>
          </Router>
        )
    );
  }
}

const Error = () => (
  <div className="uk-position-center">
    {'Error loading application.'}
  </div>
);

const PrivateRoute = ({
  component: Component,
  isAuth, fetchPosts, posts, createPostRequest,
  voteRequest, logoutRequest, user, ...rest
}) => (
  <Route
    {...rest}
    render={props => (
      isAuth === true
        ? (
          <Component
            {...props}
            fetchPosts={fetchPosts}
            posts={posts}
            createPostRequest={createPostRequest}
            voteRequest={voteRequest}
            logoutRequest={logoutRequest}
            user={user}
          />
        )
        : <Redirect to="/" />
    )}
  />
);


if (root !== null) {
  ReactDOM.render(<App protocolService={ProtocolInterface.new()} />, root);
}
