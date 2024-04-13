use sdl2::pixels::Color;

pub struct ColorBuffer {
  pub buffer: Vec<u8>,
  pub height: usize,
  pub width: usize
}

impl ColorBuffer {
  pub fn set_pixel_color(&mut self, x: usize, y: usize, color: Color) {
    if x < self.width && y < self.height {
          let i = 4 * self.width * y + x*4;
          self.buffer[i] = color.b;
          self.buffer[i + 1] = color.g;
          self.buffer[i + 2] = color.r;
          self.buffer[i + 3] = color.a;
    }
  }
}


pub fn clear_color_buffer(mut color_buffer: ColorBuffer, color: Color) -> ColorBuffer {

    for y in 0..color_buffer.height {
		for x in 0..color_buffer.width {
                color_buffer.set_pixel_color(x, y, color);
            }
		}
    color_buffer
}
    
// BGRA32

pub fn draw_pixel(mut color_buffer: ColorBuffer, x: usize, y: usize, color: Color) -> ColorBuffer {
	if x < color_buffer.width && y < color_buffer.height {
    color_buffer.set_pixel_color(x, y, color);
	}
    color_buffer
}

pub fn draw_grid(mut color_buffer: ColorBuffer, color: Color) -> ColorBuffer {

    for y in (0..color_buffer.height).step_by(10) {
      for x in (0..color_buffer.width).step_by(10) {
        color_buffer.set_pixel_color(x, y, color);
        }
    }
    color_buffer
}

pub fn draw_rectangle(mut color_buffer: ColorBuffer, x: usize, y: usize, w: usize, h: usize, color: Color) -> ColorBuffer {
  for i in 0..h{
    for j in 0..w {
        let current_x = x + i;
        let current_y = y + j;
        color_buffer.set_pixel_color(current_x, current_y, color);
      }
    }
  color_buffer
}
