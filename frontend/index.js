import React, { Component } from "react";
import ReactDOM from 'react-dom';
// components
import OnOffSwitch from './components/switch/switch'

class App extends Component {
  render() {
    return (
    <div className="App">
      <h1>Dishwasher</h1>
      <OnOffSwitch/>
    </div>
    );
  }
}

ReactDOM.render(<App/>, document.getElementById('app'));
