use minifb::{Key, Window, WindowOptions};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::collections::VecDeque;
use std::error::Error;
use std::borrow::{Borrow, BorrowMut};
use std::{thread, time};
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
const W: usize = 800;
const H: usize = 600;

struct BufferWrapper(Vec<u32>);
impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe {
            std::slice::from_raw_parts(
                self.0.as_ptr() as *const u8,
                self.0.len() * 4
            )
        }
    }
}
impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe {
            std::slice::from_raw_parts_mut(
                self.0.as_mut_ptr() as *mut u8,
                self.0.len() * 4
            )
        }
    }
}
impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        self.0.as_slice()
    }
}
impl BorrowMut<[u32]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u32] {
        self.0.as_mut_slice()
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = BufferWrapper(vec![0u32; W * H]);

    let mut window = Window::new(
        "window_title",
        W,
        H,
        WindowOptions::default(),
    )?;

    let mut data = VecDeque::new();

    let distribution: Uniform<f32> = Uniform::new(0., 1.);
    let mut rng = thread_rng();
    let mut epoch: f64 = 0.;
    let sleep_time = time::Duration::from_millis(500);

    while window.is_open() && !window.is_key_down(Key::Escape) {     
        {
            let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                buf.borrow_mut(),
                (W as u32, H as u32),
            )?
            .into_drawing_area();
            root.fill(&BLACK)?;
            {
                let a = rng.sample(distribution) as f64;
                data.push_back((epoch, a));


                let mut chart = ChartBuilder::on(&root)
                    .margin(10)
                    .set_all_label_area_size(30)
                    .build_cartesian_2d(0f64..epoch as f64, 0f64..1f64)?;

                chart
                    .configure_mesh()
                    .label_style(("sans-serif", 15).into_font().color(&GREEN))
                    .axis_style(&GREEN)
                    .bold_line_style(&GREEN.mix(0.2))
                    .light_line_style(&TRANSPARENT)
                    .draw()?;
                
                chart.plotting_area().fill(&BLACK)?;

                chart.draw_series(data.iter().zip(data.iter().skip(1)).map(
                    |(&(x0, y0), &(x1, y1))| {
                        PathElement::new(
                            vec![(x0, y0), (x1, y1)],
                            &GREEN,
                        )
                    },
                ))?;
            }
            root.present()?;
        }
        window.update_with_buffer(buf.borrow(), W, H)?;

        while let Some((e, _)) = data.front() {
            if data.len() > 10 {
                data.pop_front();
            }
            break;
        }
        thread::sleep(sleep_time);
        epoch += 1.;
    }
    Ok(())
}