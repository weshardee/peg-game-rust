use React from 'react';

type Props = {
  children?: React.Element<any>,
};

const style = {
  overflow: 'hidden',
  position: 'relative',
  width: '100vw',
  height: '100vh',
  backgroundColor: '#333333',
};

const Stage = (props: Props) => (
  <div style={style}>
    {props.children}
  </div>
);

pub Stage;
