import * as React from 'react';
import { createContext, useState, ReactNode} from "react";

type Props = {
    children: ReactNode
}

export type PlayerState = {
    looped: boolean,
    drawed: boolean,
    duration: number,
    position: number,
    volume: number,
    paused: boolean
}

export type ActionType =
| {type: "looped", payload: boolean}
| {type: "drawed", payload: boolean}
| {type: "duration", payload: number}
| {type: "position", payload: number}
| {type: "volume", payload: number}
| {type: "paused", payload: boolean};

type PlayerStateContext = {
    playerState: PlayerState,
    playerStateDispatch: React.Dispatch<ActionType>
}

const defaultState = {
    looped: false,
    drawed: false,
    duration: 1800, // second
    position: 0,
    volume: 0,
    paused: true
}

export const generateFromLooped = (state: PlayerState, value: boolean) => {
    let newPlayerState = { ...state };
    newPlayerState.looped = value;
    return newPlayerState;
}

export const generateFromDrawed = (state: PlayerState, value: boolean) => {
    let newPlayerState = { ...state };
    newPlayerState.drawed = value;
    return newPlayerState;
}

export const generateFromDuration = (state: PlayerState, value: number) => {
    let newPlayerState = { ...state };
    newPlayerState.duration = value;
    return newPlayerState;
}

export const generateFromPosition = (state: PlayerState, value: number) => {
    let newPlayerState = { ...state };
    newPlayerState.position = value;
    return newPlayerState;
}

export const generateFromVolume = (state: PlayerState, value: number) => {
    let newPlayerState = { ...state };
    newPlayerState.volume = value;
    return newPlayerState;
}

export const generateFromPaused = (state: PlayerState, value: boolean) => {
    let newPlayerState = { ...state };
    newPlayerState.paused = value;
    return newPlayerState;
}

export const reducerFunction = (state: PlayerState, action: ActionType) => {
    console.debug(state)
        console.debug(action)
        switch (action.type) {
            case 'looped':
                return {...state, looped: action.payload };
            case 'drawed':
                return {...state, drawed: action.payload };
            case 'duration':
                return {...state, duration: action.payload };
            case 'position':
                return {...state, position: action.payload };
            case 'volume':
                return {...state, volume: action.payload };
            case 'paused':
                return {...state, paused: action.payload };
            default:
                return state;
        }
};

//export const PlayerContext = createContext([defaultContext, (context: PlayerContext) => {}]);
export const PlayerContext = createContext({} as PlayerStateContext);

export const ContextProvider: React.FC<Props> = (props) => {
    const [playerState, playerStateDispatch] = React.useReducer(reducerFunction, defaultState);
    return(
        <PlayerContext.Provider value={{playerState, playerStateDispatch}}>
            {props.children}
        </PlayerContext.Provider>
    )
}
