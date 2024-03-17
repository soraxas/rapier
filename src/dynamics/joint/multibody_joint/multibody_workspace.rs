use crate::dynamics::Velocity;
use crate::math::*;
use na::DVector;

/// A temporary workspace for various updates of the multibody.
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct MultibodyWorkspace {
    pub accs: Vec<Velocity>,
    pub ndofs_vec: DVector<Real>,
}

impl MultibodyWorkspace {
    /// Create an empty workspace.
    pub fn new() -> Self {
        MultibodyWorkspace {
            accs: Vec::new(),
            ndofs_vec: DVector::zeros(0),
        }
    }

    /// Resize the workspace so it is enough for `nlinks` links.
    pub fn resize(&mut self, nlinks: usize, ndofs: usize) {
        self.accs.resize(nlinks, Velocity::zero());
        self.ndofs_vec = DVector::zeros(ndofs)
    }
}
