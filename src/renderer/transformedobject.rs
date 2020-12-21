/*//an object that supports rotation, scaling and positioning.

use crate::renderer::globject;
use crate::material::Material;

pub struct TransformedObject<'a> {
    pub rotation: float,
    pub scaling: (float, float, float),
    pub position: (float, float),

    globject: &'a globject::GlObject,
    material: &'a material::Material,
}

impl TransformedObject {
    pub fn from_globject(globject: &globject::GlObject, material: &material::Material) -> TransformedObject {
        TransformedObject {
            rotation: 0.0,
            scaling: (1.0, 1.0, 1.0),
            position: (0.0, 0.0),

            globject: globject,
            material: material,
        }
    }
}*/

