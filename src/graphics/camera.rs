use crate::input;
use crate::math::{Mat4, Vec2, Vec3};
use crate::window;
use crate::Context;

/// A camera that can be used to transform the player's view of the scene.
///
/// To apply the transformation, call the `as_matrix` method and pass the
/// resulting `Mat4` to [`graphics::set_transform_matrix`](../fn.set_transform_matrix.html).
/// To disable it, call [`graphics::reset_transform_matrix`](../fn.set_transform_matrix.html).
///
/// The camera's matrix is cached internally as an optimization. After adjusting parameters
/// on the camera, you can call the `update` method to recalculate the matrix.
///
/// # Examples
///
/// The [`camera`](https://github.com/17cupsofcoffee/tetra/blob/main/examples/camera.rs)
/// example demonstrates how a camera can be used to transform a simple
/// scene.
#[derive(Debug, Clone)]
pub struct Camera {
    /// The position of the camera.
    pub position: Vec2<f32>,

    /// The rotation of the camera, in radians.
    pub rotation: f32,

    /// The zoom level of the camera.
    pub zoom: f32,

    /// The width of the camera's viewport.
    pub viewport_width: f32,

    /// The height of the camera's viewport.
    pub viewport_height: f32,

    matrix: Mat4<f32>,
}

impl Camera {
    /// Creates a new camera with the given viewport size.
    pub fn new(viewport_width: f32, viewport_height: f32) -> Camera {
        Camera {
            position: Vec2::zero(),
            rotation: 0.0,
            zoom: 1.0,
            viewport_width,
            viewport_height,

            matrix: Mat4::translation_2d(Vec2::new(viewport_width / 2.0, viewport_height / 2.0)),
        }
    }

    /// Creates a new camera, with the viewport size set to match the size of the window.
    ///
    /// This is a useful shortcut if your game renders at a 1:1 ratio with the game window.
    /// If you're rendering to a differently sized target (e.g. a `Canvas` or a
    /// `ScreenScaler`), then you should use call `new` with the target size
    /// instead.
    ///
    /// Note that if the window is resized, the camera's viewport size will *not* automatically
    /// update. If you need to keep the window size and the viewport size in sync, then call
    /// `set_viewport_size` in your `State`'s `event` method when `Event::Resized` is fired.
    pub fn with_window_size(ctx: &Context) -> Camera {
        let (width, height) = window::get_size(ctx);
        Camera::new(width as f32, height as f32)
    }

    /// Sets the size of the camera's viewport.
    pub fn set_viewport_size(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
    }

    /// Recalculates the transformation matrix, based on the data currently contained
    /// within the camera.
    pub fn update(&mut self) {
        self.matrix = Mat4::translation_2d(-self.position);
        self.matrix.rotate_z(self.rotation);
        self.matrix.scale_3d(Vec3::new(self.zoom, self.zoom, 1.0));
        self.matrix.translate_2d(Vec2::new(
            self.viewport_width / 2.0,
            self.viewport_height / 2.0,
        ));
    }

    /// Returns the current transformation matrix.
    ///
    /// Pass this to `graphics::set_transform_matrix` to apply the transformation to
    /// your scene. To disable the transformation, call `graphics::reset_transform_matrix`.
    ///
    /// The matrix is cached internally, so calling this method multiple times will not
    /// cause it to be recalculated from scratch.
    pub fn as_matrix(&self) -> Mat4<f32> {
        self.matrix
    }

    /// Projects a point from world co-ordinates to camera co-ordinates.
    pub fn project(&self, point: Vec2<f32>) -> Vec2<f32> {
        self.as_matrix()
            .inverted()
            .mul_point(Vec3::from_point_2d(point))
            .xy()
    }

    /// Projects a point from camera co-ordinates to world co-ordinates.
    pub fn unproject(&self, point: Vec2<f32>) -> Vec2<f32> {
        self.as_matrix().mul_point(Vec3::from_point_2d(point)).xy()
    }

    /// Returns the mouse's position in camera co-ordinates.
    ///
    /// This is a shortcut for calling `project(input::get_mouse_position(ctx))`.
    /// As such, it does not take into account any other transformations
    /// being made to the view (e.g. screen scaling).
    pub fn mouse_position(&self, ctx: &Context) -> Vec2<f32> {
        self.project(input::get_mouse_position(ctx))
    }

    /// Returns the X co-ordinate of the mouse's position in camera co-ordinates.
    ///
    /// This is a shortcut for calling `project(input::get_mouse_position(ctx)).x`.
    /// As such, it does not take into account any other transformations
    /// being made to the view (e.g. screen scaling).
    pub fn mouse_x(&self, ctx: &Context) -> f32 {
        self.mouse_position(ctx).x
    }

    /// Returns the Y co-ordinate of the mouse's position in camera co-ordinates.
    ///
    /// This is a shortcut for calling `project(input::get_mouse_position(ctx)).y`.
    /// As such, it does not take into account any other transformations
    /// being made to the view (e.g. screen scaling).
    pub fn mouse_y(&self, ctx: &Context) -> f32 {
        self.mouse_position(ctx).y
    }
}
