/* @flow */

import * as React from 'react'
import ReactDOM from 'react-dom'; 
import {
  BrowserRouter as Router,
  Route,
  Redirect
} from 'react-router-dom';
import Cookies from 'js-cookie';
import Sockette from 'sockette';
import Login from './components/login';
import Feed from './components/feed';

export const SESSION_TOKEN: string = "SessionToken";

import { ProtocolInterface, WsMessage, Vote } from '../../build/frontend'

const root = document.getElementById('root')

type State = {
  ws: any,
  is_authenticated: boolean,
  is_loading: boolean,
  is_connected: boolean,
  posts: Array<any>,
  user: User,
};

type User = {
    id: number,
    username: string,
    karma: number,
    streak: number,
};

class App extends React.Component<{protocolService: ProtocolInterface}, State> { 

  constructor(props) {
    super(props)
    var ws = new Sockette("ws://127.0.0.1:8088", {
        timeout: 5e3,
        maxAttempts: 10,
        onopen: this.handle_on_open,
        onmessage: this.handle_message,
        onreconnect: e => console.log('Reconnecting...', e),
        onmaximum: e => console.log('Stop Attempting!', e),
        onclose: e => console.log('Closed!', e),
        onerror: e => console.log('Error:', e)
      });
    this.state = {
        ws: ws,
        is_authenticated: false,
        is_loading: true,
        is_connected: false,
        posts: [],
        user: {
          id: -1,
          username: "",
          karma: -1,
          streak: -1,
        }
    };
  }

  handle_on_open = e => {
    e.target.binaryType = 'arraybuffer';
    this.setState({ is_loading: false, is_connected: true });
    let token = Cookies.get(SESSION_TOKEN);
    console.log(token);

    if (token) {
      let token_data = this.props.protocolService.write_login_token(token);
      console.log(token_data);
      this.state.ws.send(token_data)
    } else {
        Cookies.remove(SESSION_TOKEN);
        console.log("No token found");
    }
  }

  handle_message = e => {
    let data = new Uint8Array(e.data);
    let { protocolService } = this.props;
    let message_type = protocolService.response_type(data);
    console.log("recieving message from ws: ", message_type);

    switch (message_type) {
        case WsMessage.Login:
            let login_res = protocolService.read_login(data);
            
            if (login_res) {
                Cookies.set(SESSION_TOKEN, login_res.token);
                this.setState({ user: login_res.user, is_authenticated: true })
            } else {
                if (!this.state.is_authenticated) {
                    Cookies.remove(SESSION_TOKEN);
                    alert("An error occured when attempting to login");
                } 
            }
            break;
        case WsMessage.Logout:
            switch (protocolService.read_logout(data)) {
                case false:
                    alert("An error occured when attempting to logout");
                default:
                    this.setState({ is_authenticated: false });
            }
            break;
        case WsMessage.FetchPosts:
            let fetch_res = protocolService.read_fetch_posts(data);
                
            if (fetch_res) {
                console.debug("Posts: ", fetch_res.posts);
                Cookies.set(SESSION_TOKEN, fetch_res.token);
                this.setState({ posts: fetch_res.posts })
            } else { 
                alert("An error occured when attempting to fetching posts");
            }
            break;
        case WsMessage.CreatePost:
            let post_res = protocolService.read_create_post(data);
                    
            if (post_res) {
                Cookies.set(SESSION_TOKEN, post_res.token);
                console.log("Adding a new post", post_res.post)
                this.setState(prevState => ({
                    posts: [...prevState.posts, post_res.post]
                }))
            } else { 
                alert("An error occured when attempting to create a post");
            }
            break;
        case WsMessage.UserVote:
            let vote_res = protocolService.read_user_vote(data);
                        
            if (vote_res) {
                Cookies.set(SESSION_TOKEN, vote_res);
            } else { 
                alert("An error occured when attempting to vote on a post");
            }
            break;
        case WsMessage.InvalidPosts:
        case WsMessage.NewPost:
        case WsMessage.UpdateUsers:
        case WsMessage.Error:
            return;
    }
  }

  handle_login_creds = (name: string, password: string) => {
    let creds_data = this.props.protocolService.write_login_creds(name, password);

    if (creds_data) {
      this.state.ws.send(creds_data);
    }
  }

  handle_register = (name: string, password: string) => {
    let creds_data = this.props.protocolService.write_registration(name, password);
    console.log(creds_data);

    if (creds_data) {
      this.state.ws.send(creds_data);
    }
  }

  create_post_request = (message: string) => {
    let token = Cookies.get(SESSION_TOKEN);
    if (token) {
        console.log("Sending message to server: ", message);
        let data = this.props.protocolService.write_create_post(token, message);
        if (data) {
            console.log("message converted to binary: ", data);
            this.state.ws.send(data);
        }
    }
  }

  vote_request = (id: number, vote: Vote) => {
    let token = Cookies.get(SESSION_TOKEN);
    if (token) {
        console.log("Sending vote to server: ", vote);
        let data = this.props.protocolService.write_user_vote(token, id, vote);
        if (data) {
            this.state.ws.send(data);
        }
    }
  }

  fetch_posts = () => {
    let token = Cookies.get(SESSION_TOKEN);
    if (token) {
        let token_data = this.props.protocolService.write_fetch_posts(token);
        this.state.ws.send(token_data);
    }
  }

  render() {
    let { is_loading, is_authenticated } = this.state;
    return (
      is_loading 
      ? (<div className="uk-position-center" uk-spinner=""></div>)
      : (
        <Router>
          <div>
            <Route exact  path="/" 
            render={(props) => <Login {...props} 
                loginRequest={this.handle_login_creds}
                registerRequest={this.handle_register}
                isAuth={is_authenticated} />}/>
            <Route path="/error" component={Error}/>
            <PrivateRoute path='/feed' 
            component={Feed} 
            isAuth={is_authenticated}
            posts={this.state.posts}
            fetchPosts={this.fetch_posts}
            createPostRequest={this.create_post_request}
            voteRequest={this.vote_request}
             />
          </div>
      </Router>
      )
    );
  }
}

const Error = () => (
  <div class="uk-position-center">
      {"Error loading application."}
  </div>
)

const PrivateRoute = ({ component: Component, isAuth, fetchPosts, posts, createPostRequest, voteRequest, ...rest }) => (
  <Route {...rest} render={(props) => (
    isAuth === true
      ? <Component {...props} 
        fetchPosts={fetchPosts} posts={posts} 
        createPostRequest={createPostRequest}
        voteRequest={voteRequest} />
      : <Redirect to='/' />
  )} />
)


if (root !== null) {
    ReactDOM.render(<App protocolService={ProtocolInterface.new()} /> , root)
}