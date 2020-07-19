type Props = {
  x?: f32,
  y?: f32,
  rotation?: f32,
  scale?: f32,
  pivot?: { x: f32, y: f32 },
  size?: { w: f32, h: f32 },
};

const DEFAULT_PIVOT = { x: 0.5, y: 0.5 };

pub fn get_transform(props: Props): String {
  let pivot = props.pivot ? props.pivot : DEFAULT_PIVOT;
  let width = props.size ? props.size.w : 0;
  let height = props.size ? props.size.h : 0;
  let transforms: Array<string> = [];
  if (props.x) {
    transforms.push(`translateX(${props.x - pivot.x * width}px)`);
  }
  if (props.y) {
    transforms.push(`translateY(${props.y - pivot.y * height}px)`);
  }
  if (props.rotation) {
    transforms.push(`rotate(${props.rotation}turn)`);
  }
  if (props.scale != null && props.scale !== 1) {
    transforms.push(`scale(${props.scale})`);
  }
  return transforms.join(' ');
}