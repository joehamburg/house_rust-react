import React from 'react';
import {connect} from 'react-redux';
import * as actionCreators from '../actions/index.js';
import HomeStatus from '../components/home-status.js';


class HomeStatusCon extends React.Component {
    render() {
        return (
            <HomeStatus handleChange={this.props.getHomeStatus} available={this.props.available}></HomeStatus>
        )
    }
}


const mapStateToProps=(state) => {
    return state
}

export default connect(mapStateToProps, actionCreators)(HomeStatusCon);