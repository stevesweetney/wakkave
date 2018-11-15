/* @flow */

import * as React from 'react';
import ReactDOM from 'react-dom';

const App = () => {
    return (
    <div className="uk-position-center uk-text-large">
       This site is powered by Webassembly.
       <br/>
       Try switching to a Wasm compatible browser. 
    </div>
    )
}

if (root !== null) {
  ReactDOM.render(<App/>, root);
}
