use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cone {
    radius: f64,
    length: f64,
    charge: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConeTrunk {
    left_radius: f64,
    right_radius: f64,
    length: f64,
    charge: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hemisphere {
    radius: f64,
    charge: f64,
}

const EPSILON_0: f64 = 8.854187817e-12;

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the cone.
pub fn cone_field_on(cone: Cone, x: f64) -> f64 {
    let Cone {
        radius,
        length,
        charge,
    } = cone;
    let factor = 3.0 / 2.0;

    factor * charge / (std::f64::consts::PI * EPSILON_0 * radius.powi(3))
        * (length
            - f64::ln(radius + f64::sqrt((x + length).powi(2) + radius.powi(2)) / (x + length)))
}

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the cone.
pub fn cone_trunk_field_on(cone_trunk: ConeTrunk, x: f64) -> f64 {
    todo!()
}

/// Calculates the field value at `x`.
/// `x` is assumed to be on the axis of symmetry of the hemisphere.
/// `parts` is the amount of precision the user desires to have over the value of the field.
pub fn hemisphere_field_on(hemisphere: Hemisphere, x: f64, parts: i64) -> f64 {
    let Hemisphere { radius, charge } = hemisphere;
    let factor = 3.0 * charge / (8.0 * std::f64::consts::PI * EPSILON_0);
    let d_x = radius / parts as f64;
    let R2 = radius.powi(2);
    let d2 = x.powi(2);

    let sum: f64 = (1..=parts)
        .map(|i| {
            let x_i = radius / parts as f64 * i as f64;
            let r3 = (R2 - x_i.powi(2)).powf(3.0 / 2.0);
            let square_term = d2 / (d2 + R2 - x_i.powi(2)).powf(0.5);
            d_x * (1.0 / r3) * (1.0 - square_term)
        })
        .sum();

    sum * factor
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
/// `parts` is the amount of precision the user desires to have over the value of the field.
#[wasm_bindgen]
pub fn js_hemisphere_field_on(
    figure: JsValue,
    x: JsValue,
    parts: JsValue,
) -> Result<JsValue, JsValue> {
    let hemisphere = serde_wasm_bindgen::from_value(figure)?;
    let x = serde_wasm_bindgen::from_value(x)?;
    let parts = serde_wasm_bindgen::from_value(parts)?;

    let field = hemisphere_field_on(hemisphere, x, parts);
    Ok(serde_wasm_bindgen::to_value(&field)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cone_field_works() {
        const CORRECT_FIELD: f64 = 466966208714.77106;
        let cone = Cone {
            radius: 0.3,
            length: 0.5,
            charge: 1.0,
        };

        let point = 2.5;

        let output = cone_field_on(cone, point);
        let difference = (output - CORRECT_FIELD).abs();
        // Has at least 5 decimal points of precision...
        println!("Output: {}", output);
        println!("Correct: {}", CORRECT_FIELD);
        println!("Difference: {}", difference);
        assert!(difference <= 1e-5);
    }

    #[test]
    fn cone_trunk_works() {
        //TODO Implement test
        const CORRECT_FIELD: f64 = 0.0;
        let cone = ConeTrunk {
            length: 0.5,
            charge: 1.0,
            left_radius: 0.5,
            right_radius: 0.3,
        };

        let point = 2.5;

        let output = cone_trunk_field_on(cone, point);
        let difference = (output - CORRECT_FIELD).abs();
        // Has at least 5 decimal points of precision...
        println!("Output: {}", output);
        println!("Correct: {}", CORRECT_FIELD);
        println!("Difference: {}", difference);
        assert!(difference <= 1e-5);
    }

    #[test]
    fn hemisphere_field_works() {
        //TODO Implement test
        const CORRECT_FIELD: f64 = 0.0;
        let hemisphere = Hemisphere {
            radius: 0.5,
            charge: 1.0,
        };

        let point = 2.5;

        let output = hemisphere_field_on(hemisphere, point, 500);
        let difference = (output - CORRECT_FIELD).abs();
        // Has at least 5 decimal points of precision...
        println!("Output: {}", output);
        println!("Correct: {}", CORRECT_FIELD);
        println!("Difference: {}", difference);
        assert!(difference <= 1e-5);
    }
}
