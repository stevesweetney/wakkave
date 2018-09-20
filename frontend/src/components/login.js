/* @flow */

import * as React from 'react';

type State = {
    username: string,
    password: string,
    reg_username: string,
    reg_password: string,
    reg_password_confirm: string,
};

type Props = {
    loginRequest: (name: string, pass: string) => void,
    registerRequest: (name: string, pass: string) => void,
    history: any,
    isAuth: boolean,
}

export default class Login extends React.Component<Props, State> {
    state = {
      username: '',
      password: '',
      reg_username: '',
      reg_password: '',
      reg_password_confirm: '',
    };

    componentDidUpdate() {
      if (this.props.isAuth) {
        this.props.history.push('/feed');
      }
    }

    update_username = (event: SyntheticInputEvent<>) => {
      this.setState({ username: event.target.value });
    }

    update_password = (event: SyntheticInputEvent<>) => {
      this.setState({ password: event.target.value });
    }

    update_reg_username = (event: SyntheticInputEvent<>) => {
      this.setState({ reg_username: event.target.value });
    }

    update_reg_password = (event: SyntheticInputEvent<>) => {
      this.setState({ reg_password: event.target.value });
    }

    update_reg_password_confirm = (event: SyntheticInputEvent<>) => {
      this.setState({ reg_password_confirm: event.target.value });
    }

    login_request = (e: SyntheticEvent<HTMLButtonElement>) => {
      e.preventDefault();
      this.props.loginRequest(this.state.username, this.state.password);
    }

    register_request = (e: SyntheticEvent<HTMLButtonElement>) => {
      e.preventDefault();
      const { reg_username, reg_password, reg_password_confirm } = this.state;

      if (reg_password !== reg_password_confirm) {
        UIkit.notification('Passwords must match!', 'warning');
        return;
      }

      this.props.registerRequest(this.state.reg_username, this.state.reg_password);
    }

    render_tabs() {
      return (
        <ul
          className="uk-subnav"
          uk-switcher="connect: .login-content; animation: uk-animation-slide-left-small"
        >
          <li className="uk-active"><a href="#">Login</a></li>
          <li><a href="#">Register</a></li>
        </ul>
      );
    }

    render_forms() {
      return (
        <ul className="uk-switcher login-content">
          <li>
            <form onSubmit={function () { return false; }}>
              <div className="uk-margin">
                <input
                  className="uk-input"
                  type="text"
                  name="username"
                  placeholder="Username"
                  value={this.state.username}
                  onChange={this.update_username}
                />
              </div>
              <div className="uk-margin">
                <input
                  className="uk-input"
                  type="text"
                  name="password"
                  placeholder="Password"
                  value={this.state.password}
                  onChange={this.update_password}
                />
              </div>
              <div className="uk-margin">
                <button
                  className="uk-button uk-button-default"
                  type="submit"
                  onClick={this.login_request}
                >
                  {'Login'}
                </button>
              </div>
            </form>
          </li>
          <li>
            <form onSubmit={function () { return false; }}>
              <div className="uk-margin">
                <input
                  className="uk-input"
                  type="text"
                  name="username"
                  placeholder="Username"
                  value={this.state.reg_username}
                  onChange={this.update_reg_username}
                />
              </div>
              <div className="uk-margin">
                <input
                  className="uk-input"
                  type="text"
                  name="password"
                  placeholder="Password"
                  value={this.state.reg_password}
                  onChange={this.update_reg_password}
                />
              </div>
              <div className="uk-margin">
                <input
                  className="uk-input"
                  type="text"
                  name="password"
                  placeholder="Confirm password"
                  value={this.state.reg_password_confirm}
                  onChange={this.update_reg_password_confirm}
                />
              </div>
              <div className="uk-margin">
                <button
                  className="uk-button uk-button-default"
                  type="submit"
                  onClick={this.register_request}
                >
                  {'Register Now'}
                </button>
              </div>
            </form>
          </li>
        </ul>
      );
    }

    render() {
      return (
        <div className="uk-position-center">
          {this.render_tabs()}
          {this.render_forms()}
        </div>
      );
    }
}
