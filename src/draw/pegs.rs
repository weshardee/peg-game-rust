use crate::board::board_iterator;
use crate::constants::*;
use crate::types::*;
use kit::*;

const LEAN_THRESHOLD: f32 = 0.5;
// const DUCK_THRESHOLD: f32 = 40.0;
const GROUNDED_THRESHOLD: f32 = 1.0;

pub fn draw(ctx: &mut Ctx, state: &State) {
  // TODO death animation
  // TODO move animation
  // TODO buzz animation (when an invalid move is chosen)
  // TODO excited animation (indicates a peg is selected)

  for pos in board_iterator() {
    let peg_i = state.board.get(pos);
    match peg_i {
      None => {}
      Some(peg_i) => {
        let pos = crate::utils::board_to_screen_position(pos);
        let shadow_img_id = state.assets.shadow.unwrap().id;
        let scale = 1.0;
        let pivot = Pivot::Center;
        kit::draw_image(ctx, shadow_img_id, pos, scale, pivot);
        let peg_type = state.pegs.peg_type[peg_i];
        let peg_state = state.pegs.state[peg_i];
        let peg_animation_frame = state.pegs.animation[peg_i];
        let peg_z = state.pegs.z[peg_i];
        let peg_z_vel = state.pegs.z_vel[peg_i];

        let sheet = match peg_type {
          PegType::Beige => &state.sprites.peg_beige,
          PegType::Blue => &state.sprites.peg_blue,
          PegType::Green => &state.sprites.peg_green,
          PegType::Pink => &state.sprites.peg_pink,
          PegType::Yellow => &state.sprites.peg_yellow,
        };
        let lean = match peg_state {
          PegState::Buzz => {
            (peg_animation_frame as f32 / BUZZ_STATE_DURATION as f32 * TAU * 2.0).sin()
          }
          _ => 0.0,
        };
        let sprite = sprite(sheet, lean, peg_z, peg_z_vel);
        let pos = vec2(pos.x(), pos.y() + peg_z);
        kit::draw_sprite(ctx, sprite, pos, scale);
      }
    }
  }
}

// fn buzz_sprite(sheet: PegSheet, frame_count: u32, z: f32) -> Sprite {
//   let percent = frame_count as f32 / BUZZ_STATE_DURATION as f32;
//   let value = TAU * percent;
// };

fn sprite(sheet: &PegSheet, lean: f32, z: f32, z_vel: f32) -> Sprite {
  let z_abs = z.abs(); // distance from ground
  let grounded = near_zero(z) && near_zero(z_vel);
  if grounded {
    if lean.abs() > LEAN_THRESHOLD {
      if lean < 0.0 {
        return sheet.lean.flip_x();
      } else {
        return sheet.lean;
      }
    } else {
      return sheet.front;
    }
  } else {
    sheet.jump
  }
}

// const LEAN_THRESHOLD = 0.5;
// const DUCK_THRESHOLD = 40;
// const GROUNDED_THRESHOLD = 5;

//fn sprite_state(state: &State, i: usize) -> PegState {
// TODO
// if (peg_is_grounded(state, i)) {
//   if (Math.abs(self.props.lean) > LEAN_THRESHOLD) {PegState::Lean} else {
//PegState::Front
//   }
// } else {
//   let absZ = Math.abs(peg_z(state, i));
//   if (absZ < DUCK_THRESHOLD) { PegState::Duck } else {
// PegState::Jump
// }
//}

//   render() {
//     // provide a buffer during the fall to pause in the duck animation
//     let modified_z = Math.max(0, Math.abs(props.z) - DUCK_THRESHOLD);
//     let ground_nearness_factor = lerpf(1.0, 0.0, modified_z / 600.0);
//     let spriteState = self.getSpriteState(this);
//     let { src, pivot, size } = sprites[props.type][spriteState];
//     let flipX = spriteState == 'lean' && props.lean < 0;
//     return (
//       <Group x={props.x} y={props.y}>
//         <Image
//           src={shadowSVG}
//           alpha={props.alive * groundNearnessFactor}
//           /* TODO handle retina better*/
//           scale={props.alive * groundNearnessFactor}
//           width={55}
//           height={34}
//         />
//         <div
//           style={{
//             transform: `scaleX(${flipX ? 1 : -1})`,
//           }}
//         >
//           <Image
//             alpha={props.alive}
//             src={src}
//             onClick={self._onTouch}
//             pivot={pivot}
//             scale={props.alive}
//             width={size.w}
//             height={size.h}
//             y={-modifiedZ} /* reflect to sim a bounce*/
//           />
//         </div>
//       </Group>
//     );
//   }
