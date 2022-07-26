# opengl_framebuffer

**Unofficial** bindings for OpenGL framebuffers, for use with [Piston's `opengl_graphics`](https://github.com/PistonDevelopers/opengl_graphics) backend.


## Usage

```rust
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
let mut gl = GlGraphics::new(OpenGL::V3_2);

let texture_settings = opengl_graphics::TextureSettings::new();
let canvas = image::ImageBuffer::new(800, 800);
let texture = Texture::from_image(&canvas, &texture_settings);
let buffer = FrameBuffer::new(texture);

buffer.draw(args.viewport(), gl, |_, gl| {
    clear(BLACK, gl);
    // use `gl` to draw to the framebuffer here
});

gl.draw(v, |c, gl| {
    clear(BLACK, gl);

    // Draw the framebuffer to the screen
    Image::new().draw(
        buffer.texture(),
        &DrawState::default(),
        c.transform,
        gl,
    );
});
```
