use super::mix;
use bulletproofs::r1cs::ConstraintSystem;
use util::{SpacesuitError, Value};

/// Enforces that the outputs are either a merge of the inputs :`D = A + B && C = 0`,
/// or the outputs are equal to the inputs `C = A && D = B`.
/// Works for `k` inputs and `k` outputs.
pub fn fill_cs<CS: ConstraintSystem>(
    cs: &mut CS,
    inputs: Vec<Value>,
    outputs: Vec<Value>,
) -> Result<(), SpacesuitError> {
    mix::fill_cs(cs, inputs, outputs)
}