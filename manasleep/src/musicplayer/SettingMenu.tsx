import * as React from 'react';
import { invoke } from "@tauri-apps/api";
import { useContext } from "react";
import { styled } from '@mui/material/styles';
import AllInclusiveIcon from '@mui/icons-material/AllInclusive';
import Box from '@mui/material/Box';
import Checkbox from '@mui/material/Checkbox';
import Drawer from '@mui/material/Drawer';
import FormControlLabel from '@mui/material/FormControlLabel';
import Grid from '@mui/material/Grid';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import Slider from '@mui/material/Slider';
import MuiInput from '@mui/material/Input';
import VolumeUp from '@mui/icons-material/VolumeUp';
import {PlayerContext, generateFromDrawed, generateFromLooped, generateFromPaused, generateFromDuration, generateFromPosition} from './PlayerContext';

export const PlayerDrawer = () => {
    const {playerState, playerStateDispatch} = React.useContext(PlayerContext);

    const handleIntervalChange = (event: Event, newValue: number | number[]) => {
        if (typeof newValue === 'number') {
            invoke("cmd_set_interval", { interval: newValue });
            playerStateDispatch({ type: 'interval', payload: newValue });
        }
    };

    const handleDurationChange = (event: Event, newValue: number | number[]) => {
        if (typeof newValue === 'number') {
            invoke("cmd_set_duration", { duration: newValue * 60 });
            invoke("cmd_set_position", { position: 0 });
            playerStateDispatch({ type: 'duration', payload: newValue * 60 });
            playerStateDispatch({ type: 'position', payload: 0 });
        }
    };

    const handleLoopedChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        let newLooped = event.target.checked;
        invoke("cmd_set_looping", { looping: newLooped });
        playerStateDispatch({ type: 'looped', payload: newLooped });
    };

    return (
    <Drawer
      anchor='top'
      PaperProps={{
        sx: {
          color: "rgba(255, 255, 255, 1)",
          backgroundColor: "rgba(100, 100, 100, 0.8)",
          pt: 2,
          px: 2,
          pb: 2,
        }
      }}
      open={playerState.drawed}
      onClose={() => {playerStateDispatch({ type: 'drawed', payload: false })}}
      sx={{backgroundColor: 'rgba(0, 0, 0, 0.1)', alignItems: 'center' }}
      >
        <Typography gutterBottom>
        読み上げ間隔(秒)
        </Typography>
        <Box sx={{ width: 250, display: 'flex' }}>
          <Slider
            size="small"
            aria-label="duration"
            defaultValue={ (playerState.interval ) }
            valueLabelDisplay="auto"
            onChange={handleIntervalChange}
            min={3}
            max={180}
          />
        </Box>

        <Typography gutterBottom>
        再生時間(分)
        </Typography>
        <Box sx={{ width: 250, display: 'flex' }}>
          <Slider
            size="small"
            disabled={playerState.looped ? true : false}
            aria-label="duration"
            defaultValue={ (playerState.duration / 60) }
            valueLabelDisplay="auto"
            onChange={handleDurationChange}
            step={5}
            marks
            min={5}
            max={180}
          />
        </Box>

        <Box sx={{ width: 250, display: 'flex' }}>
        <FormControlLabel
          value="end"
          control={<Checkbox
              checked={playerState.looped}
              onChange={handleLoopedChange}
              inputProps={{ 'aria-label': 'controlled' }}
              />}
          label="ループ再生"
          labelPlacement="end"
        />
        </Box>
    </Drawer>
    )
}
