import * as React from 'react';
import { createContext, useState, ReactNode} from "react";

type Props = {
    children: ReactNode
}

type PlayerState = {
    looped: boolean,
    drawed: boolean,
    duration: number,
    position: number,
    paused: boolean
}

type PlayerStateContext = {
    playerState: PlayerState,
    setPlayerState: React.Dispatch<React.SetStateAction<PlayerState>>
}

const defaultState = {
    looped: false,
    drawed: false,
    duration: 1800, // second
    position: 0,
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

export const generateFromPaused = (state: PlayerState, value: boolean) => {
    let newPlayerState = { ...state };
    newPlayerState.paused = value;
    return newPlayerState;
}

//export const PlayerContext = createContext([defaultContext, (context: PlayerContext) => {}]);
export const PlayerContext = createContext({} as PlayerStateContext);

export const ContextProvider: React.FC<Props> = (props) => {
    const [playerState, setPlayerState] = useState(defaultState);
    return(
        <PlayerContext.Provider value={{playerState, setPlayerState}}>
            {props.children}
        </PlayerContext.Provider>
    )
}
