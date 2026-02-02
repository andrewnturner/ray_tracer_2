/// Coordinates of a point on the final rendered image.
/// 2D space with extent [0, image_width) x [0, image_height].
/// Coordinates start from the bottom left.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RasterSpace;

/// Normalised coordinates on an abstract screen.
/// 2D space with extent [-aspect_ratio, -1) x [aspect_ratio, 1).
/// Coordinates start from bottom left.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ScreenSpace;

/// Objective coordinates for all objects in a scene.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WorldSpace;

/// Local coordinates for a camera.
/// The camera's lens points down the positive z-axis.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CameraSpace;

/// Local coordinates for an element of a scene.
/// The element is centred at the origin.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObjectSpace;

/// Local coordinates for the parent of an object in a scene.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ParentSpace;

/// Coordinates for a point on a texture
/// 2D space.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TextureSpace;

/// Utility space for intermediate values when composing transforms.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IntermediateSpace;
