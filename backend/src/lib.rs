use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cone {
    radius: f32,
    length: f32,
    charge: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConeTrunk {
    left_radius: f32,
    right_radius: f32,
    length: f32,
    charge: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hemisphere {
    radius: f32,
    charge: f32,
}

const EPSILON_0: f32 = 8.854187817e-12;

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the cone.
pub fn cone_field_on(cone: Cone, x: f32) -> f32 {
    todo!()
}

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the cone.
pub fn cone_trunk_field_on(cone_trunk: ConeTrunk, x: f32) -> f32 {
    todo!()
}

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the hemisphere.
pub fn hemisphere_field_on(hemisphere: Hemisphere, x: f32) -> f32 {
    todo!()
}

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the cone.
#[wasm_bindgen]
pub fn js_cone_field_on(figure: JsValue, x: JsValue) -> Result<JsValue, JsValue> {
    let cone = serde_wasm_bindgen::from_value(figure)?;
    let x = serde_wasm_bindgen::from_value(x)?;

    let field = cone_field_on(cone, x);
    Ok(serde_wasm_bindgen::to_value(&field)?)
}

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the cone trunk.
#[wasm_bindgen]
pub fn js_cone_trunk_field_on(figure: JsValue, x: JsValue) -> Result<JsValue, JsValue> {
    let trunk = serde_wasm_bindgen::from_value(figure)?;
    let x = serde_wasm_bindgen::from_value(x)?;

    let field = cone_trunk_field_on(trunk, x);
    Ok(serde_wasm_bindgen::to_value(&field)?)
}

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the hemisphere.
#[wasm_bindgen]
pub fn js_hemisphere_field_on(figure: JsValue, x: JsValue) -> Result<JsValue, JsValue> {
    let hemisphere = serde_wasm_bindgen::from_value(figure)?;
    let x = serde_wasm_bindgen::from_value(x)?;

    let field = hemisphere_field_on(hemisphere, x);
    Ok(serde_wasm_bindgen::to_value(&field)?)
}
