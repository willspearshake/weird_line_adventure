use sdl2::pixels::Color;

static WINDOW_WIDTH: usize = 800;
static WINDOW_HEIGHT: usize = 600;



pub fn clear_color_buffer(mut color_buffer: Vec<u8>,color: Color) -> Vec<u8> {

    for y in 0..WINDOW_HEIGHT {
		for x in 0..WINDOW_WIDTH {
                let i =  (4 * WINDOW_WIDTH * y) as usize + x*4;
                color_buffer[i] = color.b;
                color_buffer[i + 1] = color.g;
                color_buffer[i + 2] = color.r;
                color_buffer[i + 3] = color.a;
            }
		}
    color_buffer
}
    
// BGRA32

pub fn draw_pixel(mut color_buffer: Vec<u8>, x: usize, y: usize, color: Color) -> Vec<u8> {
	if x < WINDOW_WIDTH && y < WINDOW_HEIGHT {
        let i =  (4 * WINDOW_WIDTH * y) as usize + x*4;
        color_buffer[i] = color.b;
        color_buffer[i + 1] = color.g;
        color_buffer[i + 2] = color.r;
        color_buffer[i + 3] = color.a;
	}
    color_buffer
}

pub fn draw_grid(mut color_buffer: Vec<u8>, color: Color) -> Vec<u8> {

    for y in (0..WINDOW_HEIGHT).step_by(10) {
      for x in (0..WINDOW_WIDTH).step_by(10) {
                  let i =  (4 * WINDOW_WIDTH * y) as usize + x*4;
                  color_buffer[i] = color.b;
                  color_buffer[i + 1] = color.g;
                  color_buffer[i + 2] = color.r;
                  color_buffer[i + 3] = color.a;
              }
      }
    color_buffer
}

pub fn draw_rectangle(mut color_buffer: Vec<u8>, x: usize, y: usize, w: usize, h: usize, color: Color) -> Vec<u8> {
  for i in 0..WINDOW_HEIGHT{
    for j in 0..WINDOW_WIDTH {
        let current_x = x + i;
        let current_y = y + j;
        color_buffer = draw_pixel(color_buffer, current_x, current_y, color);
      }
    }
  color_buffer
}