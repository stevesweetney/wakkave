/* @flow */

import * as React from 'react'
import ReactDOM from 'react-dom';
import UserPost from '../components/post';
import { Vote } from '../../../build/frontend';

type State = {
    message: string,
};

type Props = {
    posts: Array<any>,
    fetchPosts: () => void,
    createPostRequest: (content: string) => void,
    voteRequest: (n: number, vote: Vote) => void,
};

export default class Feed extends React.Component<Props, State> {
    constructor(props: Props) {
        super(props);

        this.state = {
            message: "",
        }
    }

    componentDidMount() {
        this.props.fetchPosts();
    }

    render_posts = () => (
        <ul className="uk-list uk-list-divider">
            {this.props.posts.map(p => (
                <UserPost key={p.id} id={p.id}
                content={p.content} isMine={false} vote={p.vote}
                onVote={this.props.voteRequest}  />
            ))}
        </ul>
    )

    update_username = (event: SyntheticInputEvent<>) => {
        this.setState({message: event.target.value});
    }

    create_post_request = (e: SyntheticEvent<HTMLButtonElement>) => {
        e.preventDefault();
        
        this.props.createPostRequest(this.state.message);
        this.setState({ message: "" });

        return;
    }

    render_form = () => (
        <form>
            <div className="uk-flex uk-flex-middle uk-flex-column">
                <div className="uk-margin">
                    <textarea
                        className="uk-textarea uk-form-width-medium"
                        rows="5" placeholder="Enter a message"
                        value={this.state.message}
                        onChange={this.update_username}>
                    </textarea>
                </div>
                <div className="uk-margin">
                    <button className="uk-button uk-button-default"
                    onClick={this.create_post_request}>{"Send!"}</button>
                </div>
            </div>
        </form>
    )

    render() {
        return (
            <div className="uk-container">
                <p>{"Your feed will appear here"}</p>
                {this.render_posts()}
                {this.render_form()}
            </div>
        );
        
    }
}