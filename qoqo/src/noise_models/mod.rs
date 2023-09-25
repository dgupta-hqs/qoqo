// Copyright © 2023 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.

//! Noise models encode different types of noise that can be present in Quantum Computing hardware.
//!
//! Collection of pyo3 wrappers around roqoqo noise models.

mod continuous_decoherence;
pub use continuous_decoherence::ContinuousDecoherenceModelWrapper;
mod imperfect_readout;
pub use imperfect_readout::ImperfectReadoutModelWrapper;
mod error_on_gate;
pub use error_on_gate::ErrorOnGateModelWrapper;
use pyo3::prelude::*;

/// A collection of noise models that represent different types of noise that can be present in Quantum Computing hardware.
///
/// .. autosummary::
///     :toctree: generated/
///
///     ContinuousDecoherenceModel
///     ImperfectReadoutModel
///     ErrorOnGateModel
#[pymodule]
pub fn noise_models(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<ContinuousDecoherenceModelWrapper>()?;
    module.add_class::<ImperfectReadoutModelWrapper>()?;
    module.add_class::<ErrorOnGateModelWrapper>()?;
    Ok(())
}