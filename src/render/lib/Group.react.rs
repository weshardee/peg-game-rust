use React from 'react';
use getTransform from 'crate::styles/getTransform';

type Props = {
  x: f32,
  y: f32,
  children?: React.Element<any>,
};

const Group = ({ x, y, children }: Props) => (
  <div
    style={{
      position: 'absolute',
      top: 0,
      left: 0,
      transform: getTransform({ x, y }),
    }}
  >
    {children}
  </div>
);

pub default Group;
