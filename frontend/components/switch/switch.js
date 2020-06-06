import React, { Component } from "react";
import Switch from "react-switch";
import axios from "axios"

import { REST_ENDPOINT } from "../../constants";

/*--------------------------------------------------------------------------------------------------------------------*/

export default class OnOffSwitch extends Component {
    constructor(props) {
        super(props);
        this.id = props.id;
        this.state = { checked: false };
        this.handleChange = this.handleChange.bind(this);
    }

    componentDidMount() {
        axios.get(`http://${REST_ENDPOINT}/homestatus/${this.id}`).then((response) =>{
            this.setState({
                checked: response.data.available
            })
        })
    }

    handleChange(checked) {
        this.setState({ checked });
        let content = { 
            id: parseInt(this.id), 
            description: 'dishwasher',
            available: checked
        }
        axios.put(`http://${REST_ENDPOINT}/homestatus/${this.id}`, content).then((response) =>{
            console.log(response);
        })
    }

    render() {
        return (
        <label>
            <Switch onChange={this.handleChange} checked={this.state.checked} />
        </label>
        );
    }
}