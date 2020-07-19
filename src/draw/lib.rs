mod lib;

// use React from 'react';
// use Stage from 'crate::lib/Stage.react';
// use Tiles from 'crate::Tiles.react';
// use PegsContainer from 'crate::PegsContainer.react';
// use Group from 'crate::lib/Group.react';
// use ResetBtn from 'crate::ResetBtn.react';
// use StatusMsg from 'crate::StatusMsg.react';

type State = {
  width: f32,
  height: f32,
};

class App extends React.Component {
  state: State = getWindowDimensions();

    self.setState(getWindowDimensions());
  };

pub fn draw(&mut ctx, state: &State) {
  draw_tiles(ctx, state);

  //   return (
  //     <Stage>
  //       <Group x={self.state.width / 2} y={self.state.height / 3}>
  //         <Tiles />
  //         <PegsContainer />
  //       </Group>
  //       <Group y={14} x={self.state.width - 20}>
  //         <StatusMsg />
  //       </Group>
  //       <ResetBtn x={25} y={25} />
  //     </Stage>
  //   );
  // }
}

pub App;