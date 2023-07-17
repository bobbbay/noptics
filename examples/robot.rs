use futures::StreamExt;
use noptics::Lens;
// use tarsier::Data;
use tokio::pin;

use noptics::Data;

#[derive(Default, Copy, Clone)]
struct GyroRotDeg(usize);

#[derive(Default)]
struct InputState {
    gyro_rot_deg: GyroRotDeg,
}

// START auto-generated

#[derive(Clone)]
struct InputState1 {
    gyro_rot_deg: Lens<Self, GyroRotDeg>,
}

impl Default for InputState1 {
    #[inline]
    fn default() -> Self {
        Self {
            gyro_rot_deg: Lens::new(Default::default()),
        }
    }
}

impl Data for GyroRotDeg {}

// END auto-generated

#[tokio::main]
async fn main() {
    let input_state = InputState1::default();

    let my_lens = input_state.gyro_rot_deg;
    let my_lens_2 = my_lens.clone();

    tokio::spawn(async move {
        let s = my_lens.stream();
        pin!(s);

        while let Some(value) = s.next().await {
            let value = value.0;
            println!("{value}");
        }
    });

    tokio::spawn(async move {
        let mut i = 0;

        loop {
            if i > 10 {
                return;
            }
            my_lens_2.set(GyroRotDeg(i));
            i += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    });

    loop {}
}
