use { Motion, spring } from 'react-motion';
use React from 'react';
use Image from 'crate::lib/Image.react';
use { boardToScreenPosition } from '../utils';
use { onTouchTile } from '../interactions';
use tileSVG from 'crate::images/tile.svg';
use { BOARD_GRID_SIZE } from '../constants';

type Props = {
  x: f32,
  y: f32,
};

type AnimatedProps = {
  y: f32,
};

const TILE_HEIGHT = 89;
const TILE_WIDTH = 65;
const TILE_SCALE = BOARD_GRID_SIZE / TILE_WIDTH;

const Tile = (props: Props) => {
  const screenPos = boardToScreenPosition(props);
  return (
    <Motion
      defaultStyle={{ y: screenPos.y + 600 }}
      style={{ y: spring(screenPos.y) }}
      children={({ y }: AnimatedProps) => (
        <Image
          src={tileSVG}
          pivot={{ x: 0.5, y: 0.375 }}
          width={TILE_WIDTH}
          height={TILE_HEIGHT}
          x={screenPos.x}
          y={y}
          scale={TILE_SCALE}
          onClick={() => onTouchTile(props)}
        />
      )}
    />
  );
};

pub default Tile;
