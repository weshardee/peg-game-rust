use React from 'react';
use getTransform from 'crate::styles/getTransform';

pub type Spritesheet = {
  uri: string,
  width: f32,
  height: f32,
};

type DefaultProps = {
  rotation: f32,
  x: f32,
  y: f32,
  pivot: { x: f32, y: f32 },
  srcOffset: { x: f32, y: f32 },
  frame: f32,
  onClick: () => void,
  alpha: f32,
  scale: f32,
};

type Props = DefaultProps & {
  src: string,
  width: f32,
  height: f32,
};

class Image extends React.Component<DefaultProps, Props, void> {
  static defaultProps: DefaultProps = {
    rotation: 0,
    x: 0,
    y: 0,
    pivot: { x: 0.5, y: 0.5 },
    onClick: () => { },
    frame: 0,
    alpha: 1,
    scale: 1,
    srcOffset: { x: 0, y: 0 },
  };

  props: Props;

  render(): React.Element<any> {
    const { props } = this;
    const { x, y, rotation, scale, alpha, width, height, srcOffset } = props;
    const style = {
      // sprite sheet image and offset
      backgroundImage: `url(${props.src})`,
      backgroundPosition: `-${srcOffset.x}px -${srcOffset.y}px`,
      // sizing
      width,
      height,
      transform: getTransform({ x, y, rotation, scale }),
      transformOrigin: `${props.pivot.x * 100}% ${props.pivot.y * 100}%`,
      opacity: alpha,

      // position props
      position: 'absolute',

      // TODO move this into the transform
      left: (-props.width) * props.pivot.x,
      top: (-props.height) * props.pivot.y,
    };
    return <div style={style} onClick={props.onClick} />;
  }
}

pub default Image;
