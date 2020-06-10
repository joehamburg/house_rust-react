let defaultState = {
    available: false
}

const mainReducer = (state=defaultState, action) => {
    if (action.type === "GET_HOME_STATUS") {
        return {
            ...state,
            available: action.available
        }
    } else {
        return {
            ...state
        }
    }
}


export default mainReducer;