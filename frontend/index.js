import React from 'react';
import ReactDOM from 'react-dom';

class App extends React.Component {
  render() {
    return <h1>Test hot reload ok we can develop now jesus that took forever</h1>
  }
}

ReactDOM.render(<App/>, document.getElementById('app'));
