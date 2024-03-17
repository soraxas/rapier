use crate::math::*;
use crate::utils::SimdRealCopy;
use na::{DVectorView, DVectorViewMut};
use std::ops::{AddAssign, Sub};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
//#[repr(align(64))]
pub struct SolverVel<N: SimdRealCopy> {
    // The linear velocity of a solver body.
    pub linear: N::Vector,
    // The angular velocity, multiplied by the inverse sqrt angular inertia, of a solver body.
    pub angular: N::AngVector,
}

impl SolverVel<Real> {
    pub fn as_slice(&self) -> &[Real; SPATIAL_DIM] {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [Real; SPATIAL_DIM] {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_vector_slice(&self) -> DVectorView<Real> {
        DVectorView::from_slice(&self.as_slice()[..], SPATIAL_DIM)
    }

    pub fn as_vector_slice_mut(&mut self) -> DVectorViewMut<Real> {
        DVectorViewMut::from_slice(&mut self.as_mut_slice()[..], SPATIAL_DIM)
    }
}

impl<N: SimdRealCopy> SolverVel<N> {
    pub fn zero() -> Self {
        Self::default()
    }
}

impl<N: SimdRealCopy> AddAssign for SolverVel<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.linear += rhs.linear;
        self.angular += rhs.angular;
    }
}

impl<N: SimdRealCopy> Sub for SolverVel<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        SolverVel {
            linear: self.linear - rhs.linear,
            angular: self.angular - rhs.angular,
        }
    }
}
