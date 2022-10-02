import * as React from 'react';
import { invoke } from "@tauri-apps/api";
import { listen } from '@tauri-apps/api/event'
import { styled, useTheme } from '@mui/material/styles';
import AllInclusiveIcon from '@mui/icons-material/AllInclusive';
import Box from '@mui/material/Box';
import EditIcon from '@mui/icons-material/Edit'
import Typography from '@mui/material/Typography';
import Slider from '@mui/material/Slider';
import Icon from '@mui/material/Icon';
import Input from '@mui/material/Input';
import IconButton from '@mui/material/IconButton';
import Stack from '@mui/material/Stack';
import ToggleButton from '@mui/material/ToggleButton';
import PauseRounded from '@mui/icons-material/PauseRounded';
import PlayArrowRounded from '@mui/icons-material/PlayArrowRounded';
import FastForwardRounded from '@mui/icons-material/FastForwardRounded';
import FastRewindRounded from '@mui/icons-material/FastRewindRounded';
import Fab from '@mui/material/Fab'
import VolumeUpRounded from '@mui/icons-material/VolumeUpRounded';
import VolumeDownRounded from '@mui/icons-material/VolumeDownRounded';

import {PlayerState, PlayerContext, generateFromDrawed, generateFromLooped, generateFromPaused, generateFromDuration, generateFromPosition, generateFromVolume} from './PlayerContext';

import jacket from '../assets/jacket.png'

const Widget = styled('div')(({ theme }) => ({
  padding: 16,
  borderRadius: 16,
  width: 343,
  maxWidth: '100%',
  margin: 'auto',
  position: 'relative',
  zIndex: 1,
  backgroundColor:
    theme.palette.mode === 'dark' ? 'rgba(0,0,0,0.6)' : 'rgba(255,255,255,0.4)',
  backdropFilter: 'blur(40px)',
}));

const CoverImage = styled('div')({
  width: 100,
  height: 100,
  objectFit: 'cover',
  overflow: 'hidden',
  flexShrink: 0,
  borderRadius: 8,
  backgroundColor: 'rgba(0,0,0,0.08)',
  '& > img': {
    width: '100%',
  },
});

const TinyText = styled(Typography)({
  fontSize: '0.75rem',
  opacity: 0.38,
  fontWeight: 500,
  letterSpacing: 0.2,
});

export default function MusicPlayerSlider() {
  const theme = useTheme();
  const {playerState, playerStateDispatch} = React.useContext(PlayerContext);

  const mainIconColor = '#fff' ;
  const lightIconColor =  'rgba(0,0,0,0.4)';

  function formatDuration(value: number) {
    const minute = Math.floor(value / 60);
    const secondLeft = value - minute * 60;
    return `${minute}:${secondLeft < 10 ? `0${secondLeft}` : secondLeft}`;
  }

  const handleVolumeChange = (event: Event, newValue: number | number[]) => {
      if (typeof newValue === 'number') {
          invoke("cmd_set_volume", { volume: newValue });
          playerStateDispatch({ type: 'volume', payload: newValue })
      }
  };

  const handleLoopedChange = () => {
      let newLooped = !playerState.looped;
      invoke("cmd_set_looping", { looping: newLooped });
      playerStateDispatch({ type: 'looped', payload: newLooped })
  };

  const handlePausedChange = () => {
      let newPaused = !playerState.paused;
      invoke("cmd_set_playing", { playing: !newPaused });
      playerStateDispatch({ type: 'paused', payload: newPaused })
  };

  const handleBackPushed = () => {
      invoke("cmd_set_position", { position: 0 });
      playerStateDispatch({ type: 'position', payload: 0 })
  };

  React.useEffect(() => {
        const _unlisten_stopped = listen('player-state-stopped', (event) => {
            console.log(event)
            playerStateDispatch({ type: 'paused', payload: event.payload as boolean })
        });

        const _unlisten_position = listen('player-state-position', (event) => {
            console.debug(event)
            if (typeof event.payload === "number") {
                playerStateDispatch({ type: 'position', payload: event.payload as number })
            }
        });

  }, []);

  return (
    <Box sx={{ width: '100%', overflow: 'hidden' }}>
      <Widget>
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
          <CoverImage>
            <img alt="MANA san" src={jacket} />
          </CoverImage>
          <Box sx={{ ml: 1.5, minWidth: 0 }}>
            <Typography variant="caption" color="text.secondary" fontWeight={500}>
              COEIROINK:MANA
            </Typography>
            <Typography noWrap>
              <b>■■■■■■■■■■■■■■■</b>
            </Typography>
          </Box>
        </Box>
        <Slider
          disabled
          aria-label="time-indicator"
          size="small"
          value={playerState.position}
          min={0}
          step={1}
          max={playerState.duration}
          color="primary"
          sx={{ '& .MuiSlider-thumb': { width: 8, height: 8, }, }}
        />
        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            mt: -2,
          }}
        >
          <TinyText>{formatDuration(playerState.position)}</TinyText>
          {playerState.looped ? (
            <TinyText>inf</TinyText>
          ) : (
            <TinyText>-{formatDuration(playerState.duration - playerState.position)}</TinyText>
          )}
        </Box>

        <Box
          sx={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            mt: -1,
          }}
        >
          <IconButton aria-label="previous song">
            <FastRewindRounded
                fontSize="large"
                htmlColor={mainIconColor}
                onClick={handleBackPushed}
                />
          </IconButton>
          <IconButton
            aria-label={playerState.paused ? 'play' : 'pause'}
            onClick={handlePausedChange}
          >
            {playerState.paused ? (
              <PlayArrowRounded sx={{ fontSize: '3rem' }} htmlColor={mainIconColor}
              />
            ) : (
              <PauseRounded sx={{ fontSize: '3rem' }} htmlColor={mainIconColor} />
            )}
          </IconButton>
          <IconButton aria-label="loop"
              color={playerState.looped ? 'secondary' : 'default'  }
              onClick={handleLoopedChange}
            >
            <AllInclusiveIcon />
          </IconButton>
        </Box>

        <Stack spacing={2} direction="row" sx={{ mb: 1, px: 1 }} alignItems="center">
          <VolumeDownRounded htmlColor={lightIconColor} />
          <Slider
            aria-label="Volume"
            defaultValue={30}
            onChange={handleVolumeChange}
            sx={{
              '& .MuiSlider-track': {
                border: 'none',
              },
              '& .MuiSlider-thumb': {
                '&:hover, &.Mui-focusVisible, &.Mui-active': {
                  boxShadow: 'none',
                },
              },
            }}
          />
          <VolumeUpRounded htmlColor={lightIconColor} />
        </Stack>

        <Fab
          color="primary"
          size="small"
          aria-label="edit"
          onClick={ () => {playerStateDispatch({ type: 'drawed', payload: true })} }
          sx={{ 
              position: 'absolute',
              top: 16,
              right: 16,
          }}>
          <EditIcon />
        </Fab>

      </Widget>
    </Box>
  );
}
