use crate::acceleration::simd::Simd;
#[cfg(neon_simd)]
use crate::util::partial_ord::{partial_max, partial_min};

pub(crate) trait SimdReduceOps: Simd {
    #[cfg(neon_simd)]
    unsafe fn simd_sum_uniform(mut ptr: *const Self, mut count: usize, stride: usize) -> Self {
        let mut acc = Self::simd_from_constant(Self::zero());

        while count >= Self::LANES {
            let vec = Self::simd_vec_from_stride(ptr, stride);
            acc = Self::simd_add(acc, vec);

            count -= Self::LANES;
            ptr = ptr.add(stride * Self::LANES);
        }

        let mut output = Self::simd_horizontal_sum(acc);

        for _ in 0..count {
            output += *ptr;
            ptr = ptr.add(stride);
        }

        output
    }

    #[cfg(neon_simd)]
    unsafe fn simd_sum_contiguous(mut ptr: *const Self, mut count: usize) -> Self {
        let mut acc = Self::simd_from_constant(Self::zero());

        while count >= 4 * Self::LANES {
            let a = Self::simd_load(ptr.add(0 * Self::LANES));
            let b = Self::simd_load(ptr.add(1 * Self::LANES));
            let c = Self::simd_load(ptr.add(2 * Self::LANES));
            let d = Self::simd_load(ptr.add(3 * Self::LANES));

            let ab = Self::simd_add(a, b);
            let cd = Self::simd_add(c, d);
            let abcd = Self::simd_add(ab, cd);

            count -= 4 * Self::LANES;
            ptr = ptr.add(4 * Self::LANES);

            acc = Self::simd_add(acc, abcd);
        }

        let mut output = Self::simd_horizontal_sum(acc);
        for _ in 0..count {
            output += *ptr;
            ptr = ptr.add(1);
        }

        output
    }

    #[cfg(neon_simd)]
    unsafe fn simd_product_uniform(mut ptr: *const Self, mut count: usize, stride: usize) -> Self {
        let mut acc = Self::simd_from_constant(Self::one());

        while count >= Self::LANES {
            let vec = Self::simd_vec_from_stride(ptr, stride);
            acc = Self::simd_mul(acc, vec);

            count -= Self::LANES;
            ptr = ptr.add(stride * Self::LANES);
        }

        let mut output = Self::simd_horizontal_mul(acc);

        for _ in 0..count {
            output *= *ptr;
            ptr = ptr.add(stride);
        }

        output
    }

    #[cfg(neon_simd)]
    unsafe fn simd_product_contiguous(mut ptr: *const Self, mut count: usize) -> Self {
        let mut output = Self::one();

        while count >= 4 * Self::LANES {
            let a = Self::simd_load(ptr.add(0 * Self::LANES));
            let b = Self::simd_load(ptr.add(1 * Self::LANES));
            let c = Self::simd_load(ptr.add(2 * Self::LANES));
            let d = Self::simd_load(ptr.add(3 * Self::LANES));

            let ab = Self::simd_mul(a, b);
            let cd = Self::simd_mul(c, d);
            let abcd = Self::simd_mul(ab, cd);

            count -= 4 * Self::LANES;
            ptr = ptr.add(4 * Self::LANES);

            output *= Self::simd_horizontal_mul(abcd);
        }

        for _ in 0..count {
            output *= *ptr;
            ptr = ptr.add(1);
        }

        output
    }

    #[cfg(neon_simd)]
    unsafe fn simd_min_uniform(mut ptr: *const Self, mut count: usize, stride: usize) -> Self {
        let mut acc = Self::simd_from_constant(Self::max_value());

        while count >= Self::LANES {
            let vec = Self::simd_vec_from_stride(ptr, stride);
            acc = Self::simd_min(acc, vec);

            count -= Self::LANES;
            ptr = ptr.add(stride * Self::LANES);
        }

        let mut output = Self::simd_horizontal_min(acc);
        for _ in 0..count {
            output = partial_min(*ptr, output);
            ptr = ptr.add(stride);
        }

        output
    }

    #[cfg(neon_simd)]
    unsafe fn simd_min_contiguous(mut ptr: *const Self, mut count: usize) -> Self {
        let mut acc = Self::simd_from_constant(Self::max_value());

        while count >= 4 * Self::LANES {
            let a = Self::simd_load(ptr.add(0 * Self::LANES));
            let b = Self::simd_load(ptr.add(1 * Self::LANES));
            let c = Self::simd_load(ptr.add(2 * Self::LANES));
            let d = Self::simd_load(ptr.add(3 * Self::LANES));

            let ab = Self::simd_min(a, b);
            let cd = Self::simd_min(c, d);
            let abcd = Self::simd_min(ab, cd);

            count -= 4 * Self::LANES;
            ptr = ptr.add(4 * Self::LANES);

            acc = Self::simd_min(acc, abcd);
        }

        let mut output = Self::simd_horizontal_min(acc);
        for _ in 0..count {
            output = partial_min(*ptr, output);
            ptr = ptr.add(1);
        }

        output
    }

    #[cfg(neon_simd)]
    unsafe fn simd_max_uniform(mut ptr: *const Self, mut count: usize, stride: usize) -> Self {
        let mut acc = Self::simd_from_constant(Self::min_value());

        while count >= Self::LANES {
            let vec = Self::simd_vec_from_stride(ptr, stride);
            acc = Self::simd_max(acc, vec);

            count -= Self::LANES;
            ptr = ptr.add(stride * Self::LANES);
        }

        let mut output = Self::simd_horizontal_max(acc);
        for _ in 0..count {
            output = partial_max(*ptr, output);
            ptr = ptr.add(stride);
        }

        output
    }

    #[cfg(neon_simd)]
    unsafe fn simd_max_contiguous(mut ptr: *const Self, mut count: usize) -> Self {
        let mut acc = Self::simd_from_constant(Self::min_value());

        while count >= 4 * Self::LANES {
            let a = Self::simd_load(ptr.add(0 * Self::LANES));
            let b = Self::simd_load(ptr.add(1 * Self::LANES));
            let c = Self::simd_load(ptr.add(2 * Self::LANES));
            let d = Self::simd_load(ptr.add(3 * Self::LANES));

            let ab = Self::simd_max(a, b);
            let cd = Self::simd_max(c, d);
            let abcd = Self::simd_max(ab, cd);

            count -= 4 * Self::LANES;
            ptr = ptr.add(4 * Self::LANES);

            acc = Self::simd_max(acc, abcd);
        }

        let mut output = Self::simd_horizontal_max(acc);
        for _ in 0..count {
            output = partial_max(*ptr, output);
            ptr = ptr.add(1);
        }

        output
    }
}

impl<T: Simd> SimdReduceOps for T {}
