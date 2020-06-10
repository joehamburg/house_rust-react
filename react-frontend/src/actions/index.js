import axios from 'axios';

export function getHomeStatus() {
    return(dispatch) => {
        return axios.get("http://localhost:8000/homestatus/1").then((response)=> {
            console.log("getHomeStatus: " + String(response.data.available))
            dispatch(displayHomeStatus(response.data.available)) 
        })
    }
}

export function displayHomeStatus(available) {
    return { 
        type:"GET_HOME_STATUS",
        available: available
    }
}