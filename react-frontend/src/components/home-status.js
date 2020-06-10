import React from 'react';
import Switch from "react-switch";

class HomeStatus extends React.Component {
    constructor(props) {
        super(props);
        this.props.handleChange();
        this.state = { available: false };
    }

    componentDidMount() {
        this.setState({ available: this.props.available })
        console.log("HomeStatus Constructor: " + String(this.props.available))
    }

    render() {
        return (
            <div className="status-wrapper">
                <div className="dishwasher-button">
                    <Switch onChange={()=>{this.props.handleChange()}} checked={this.state.available} />
                    {/* <button onClick={()=>{this.props.handleChange()}}> haha{this.props.available}</button> */}
                </div>
            </div>
        )
    }
}


export default HomeStatus;