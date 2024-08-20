use num_traits::Float;

use common_error::DaftResult;

use crate::{array::DataArray, datatypes::DaftNumericType};

impl<T> DataArray<T>
where
    T: DaftNumericType,
    T::Native: Float,
{
    pub fn cbrt(&self) -> DaftResult<Self> {
        self.apply(|v| v.cbrt())
    }
}