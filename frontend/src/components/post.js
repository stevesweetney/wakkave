/* @flow */

import * as React from 'react';
import ReactDOM from 'react-dom';
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

        var v;

        if (this.props.vote === "Up") {
            v = Vote.Up
        } else if (this.props.vote == "Down") {
            v = Vote.Down
        } else {
            v = Vote.None
        }

        this.state = {
            vote: v,
        }
    }

    handle_vote = (vote: Vote) => {
        if (this.state.vote !== vote) {
            this.props.onVote(this.props.id, vote);
            this.setState({ vote });
            console.debug("Debugging vote", vote);
        }   
    }

    handle_downvote = () => {
        this.handle_vote(Vote.Down)
    }

    handle_upvote = () => {
        this.handle_vote(Vote.Up)
    }

    get_style = (vote: Vote) => {
        if (this.state.vote === vote) { 
            return ({ color: "orange" }) 
        } else { 
            return ({ color: "#999"}) 
        }
    }

    get_style_up = () => {
        return this.get_style(Vote.Up)
    }

    get_style_down = () => {
        return this.get_style(Vote.Down)
    }

    render() {  
        return (
            <li>
                <div className="uk-flex-inline uk-flex-column">
                    <span uk-icon="icon: arrow-up"
                        onClick={this.handle_upvote}
                        style={this.get_style_up()}></span>
                    <span uk-icon="icon: arrow-down"
                        onClick={this.handle_downvote}
                        style={this.get_style_down()}></span>
                </div>
                {this.props.content}
            </li>
        )
    }
        
}