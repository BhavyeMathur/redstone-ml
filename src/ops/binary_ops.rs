use std::ptr::addr_of;
use crate::collapse_contiguous::collapse_to_uniform_stride;
use crate::flat_index_generator::FlatIndexGenerator;
use paste::paste;
use std::ops::{BitAnd, BitOr, Rem, Shl, Shr};


#[macro_export]
macro_rules! define_binary_op_trait {
    ($trait_name:ident, $required_trait:ident, $name:ident, $operator:tt; $($default_dtypes:ty),*) => {
        define_binary_op_trait!($trait_name, $required_trait, $name, $operator);
        impl_default_trait_for_dtypes!($trait_name, $($default_dtypes),*);
    };

    ($trait_name:ident, $required_trait:ident, $name:ident, $operator:tt) => {
        paste! {
            pub(crate) trait $trait_name: $required_trait<Output=Self> + Sized + Copy {
                unsafe fn [<$name _stride_0_1>](lhs: *const Self,
                                                rhs: *const Self,
                                                dst: *mut Self, count: usize) {
                    Self::[<$name _stride_n_n>](lhs, 0, rhs, 1, dst, count)
                }

                unsafe fn [<$name _stride_1_0>](lhs: *const Self,
                                                rhs: *const Self, dst: *mut Self, count: usize) {
                    Self::[<$name _stride_n_n>](lhs, 1, rhs, 0, dst, count)
                }

                unsafe fn [<$name _stride_n_0>](lhs: *const Self, lhs_stride: usize,
                                                rhs: *const Self, dst: *mut Self, count: usize) {
                    Self::[<$name _stride_n_n>](lhs, lhs_stride, rhs, 0, dst, count)
                }

                unsafe fn [<$name _stride_0_n>](lhs: *const Self,
                                                rhs: *const Self, rhs_stride: usize,
                                                dst: *mut Self, count: usize) {
                    Self::[<$name _stride_n_n>](lhs, 0, rhs, rhs_stride, dst, count)
                }

                unsafe fn [<$name _stride_1_1>](lhs: *const Self, rhs: *const Self, dst: *mut Self, count: usize) {
                    Self::[<$name _stride_n_n>](lhs, 1, rhs, 1, dst, count)
                }

                unsafe fn [<$name _stride_n_1>](lhs: *const Self, lhs_stride: usize,
                                                rhs: *const Self, dst: *mut Self, count: usize) {
                    Self::[<$name _stride_n_n>](lhs, lhs_stride, rhs, 1, dst, count)
                }

                unsafe fn [<$name _stride_1_n>](lhs: *const Self,
                                                rhs: *const Self, rhs_stride: usize,
                                                dst: *mut Self, count: usize) {
                    Self::[<$name _stride_n_n>](lhs, 1, rhs, rhs_stride, dst, count)
                }

                #[inline(never)]
                unsafe fn [<$name _stride_n_n>](mut lhs: *const Self, lhs_stride: usize,
                                                mut rhs: *const Self, rhs_stride: usize,
                                                mut dst: *mut Self, mut count: usize) {
                    while count != 0 {
                        *dst = *lhs $operator *rhs;

                        count -= 1;
                        lhs = lhs.add(lhs_stride);
                        rhs = rhs.add(rhs_stride);
                        dst = dst.add(1);
                    }
                }

                unsafe fn [<$name _nonunif_0>](lhs: *const Self, lhs_shape: &[usize], lhs_stride: &[usize],
                                               rhs: *const Self,
                                               dst: *mut Self, count: usize) {
                    Self::[<$name _nonunif_n>](lhs, lhs_shape, lhs_stride, rhs, 0, dst, count)
                }

                unsafe fn [<$name _0_nonunif>](lhs: *const Self,
                                               rhs: *const Self, rhs_shape: &[usize], rhs_stride: &[usize],
                                               dst: *mut Self, count: usize) {
                    Self::[<$name _n_nonunif>](lhs, 0, rhs, rhs_shape, rhs_stride, dst, count)
                }

                unsafe fn [<$name _nonunif_1>](lhs: *const Self, lhs_shape: &[usize], lhs_stride: &[usize],
                                               rhs: *const Self,
                                               dst: *mut Self, count: usize) {
                    Self::[<$name _nonunif_n>](lhs, lhs_shape, lhs_stride, rhs, 1, dst, count)
                }

                unsafe fn [<$name _1_nonunif>](lhs: *const Self,
                                               rhs: *const Self, rhs_shape: &[usize], rhs_stride: &[usize],
                                               dst: *mut Self, count: usize) {
                    Self::[<$name _n_nonunif>](lhs, 1, rhs, rhs_shape, rhs_stride, dst, count)
                }

                unsafe fn [<$name _nonunif_n>](lhs: *const Self, lhs_shape: &[usize], lhs_stride: &[usize],
                                               mut rhs: *const Self, rhs_stride: usize,
                                               mut dst: *mut Self, mut count: usize) {
                    let mut lhs_indices = FlatIndexGenerator::from(lhs_shape, lhs_stride);

                    while count != 0 {
                        let lhs_index = lhs_indices.next().unwrap_unchecked();
                        *dst = *lhs.add(lhs_index) $operator *rhs;

                        count -= 1;
                        dst = dst.add(1);
                        rhs = rhs.add(rhs_stride);
                    }
                }

                unsafe fn [<$name _n_nonunif>](mut lhs: *const Self, lhs_stride: usize,
                                               rhs: *const Self, rhs_shape: &[usize], rhs_stride: &[usize],
                                               mut dst: *mut Self, mut count: usize) {
                    let mut rhs_indices = FlatIndexGenerator::from(rhs_shape, rhs_stride);

                    while count != 0 {
                        let rhs_index = rhs_indices.next().unwrap_unchecked();
                        *dst = *lhs $operator *rhs.add(rhs_index);

                        count -= 1;
                        dst = dst.add(1);
                        lhs = lhs.add(lhs_stride);
                    }
                }

                unsafe fn [<$name _unspecialized>](lhs: *const Self, lhs_shape: &[usize], lhs_stride: &[usize],
                                                   rhs: *const Self, rhs_shape: &[usize], rhs_stride: &[usize],
                                                   mut dst: *mut Self) {
                    let lhs_indices = FlatIndexGenerator::from(lhs_shape, lhs_stride);
                    let rhs_indices = FlatIndexGenerator::from(rhs_shape, rhs_stride);

                    for (lhs_index, rhs_index) in lhs_indices.zip(rhs_indices) {
                        *dst = *lhs.add(lhs_index) $operator *rhs.add(rhs_index);
                        dst = dst.add(1);
                    }
                }

                unsafe fn [<$name _scalar>](lhs: *const Self, lhs_shape: &[usize], lhs_stride: &[usize],
                                            rhs: Self, dst: *mut Self) {
                    // special case for scalar operands
                    if lhs_stride.is_empty() {
                        *dst = *lhs $operator rhs;
                        return;
                    }

                    let rhs = addr_of!(rhs);

                    let (lhs_shape, lhs_stride) = collapse_to_uniform_stride(lhs_shape, &lhs_stride);
                    let lhs_dims = lhs_shape.len();
                    let lhs_inner_stride = lhs_stride[lhs_dims - 1];

                    if lhs_dims == 1 {
                        if lhs_inner_stride == 1 {
                            return Self::[<$name _stride_1_0>](lhs, rhs, dst, lhs_shape[0]);
                        }
                        else {
                            return Self::[<$name _stride_n_0>](lhs, lhs_inner_stride, rhs, dst, lhs_shape[0]);
                        }
                    }

                    let count = lhs_shape.iter().product();
                    return Self::[<$name _nonunif_0>](lhs, &lhs_shape, &lhs_stride, rhs, dst, count);
                }

                unsafe fn $name(lhs: *const Self, lhs_stride: &[usize],
                                rhs: *const Self, rhs_stride: &[usize],
                                dst: *mut Self, shape: &[usize]) {
                    // special case for scalar operands
                    if lhs_stride.is_empty() && rhs_stride.is_empty() {
                        *dst = *lhs $operator *rhs;
                        return;
                    }

                    let (lhs_shape, lhs_stride) = collapse_to_uniform_stride(shape, &lhs_stride);
                    let (rhs_shape, rhs_stride) = collapse_to_uniform_stride(shape, &rhs_stride);

                    let lhs_dims = lhs_shape.len();
                    let rhs_dims = rhs_shape.len();

                    let lhs_inner_stride = lhs_stride[lhs_dims - 1];
                    let rhs_inner_stride = rhs_stride[rhs_dims - 1];

                    if lhs_dims == 1 && rhs_dims == 1 { // both operands have a uniform stride

                        // one operand is a scalar
                        if rhs_inner_stride == 0 {
                            if lhs_inner_stride == 1 {
                                return Self::[<$name _stride_1_0>](lhs, rhs, dst, lhs_shape[0]);
                            }
                            else {
                                return Self::[<$name _stride_n_0>](lhs, lhs_inner_stride, rhs, dst, lhs_shape[0]);
                            }

                        } else if lhs_inner_stride == 0 {
                            if rhs_inner_stride == 1 {
                                return Self::[<$name _stride_0_1>](lhs, rhs, dst, rhs_shape[0]);
                            }
                            else {
                                return Self::[<$name _stride_0_n>](lhs, rhs, rhs_inner_stride, dst, rhs_shape[0]);
                            }
                        }

                        // both operands are contiguous
                        if lhs_inner_stride == 1 && rhs_inner_stride == 1 {
                            return Self::[<$name _stride_1_1>](lhs, rhs, dst, lhs_shape[0]);
                        }

                        if lhs_inner_stride == 1 {
                            return Self::[<$name _stride_1_n>](lhs, rhs, rhs_inner_stride, dst, lhs_shape[0]);
                        }
                        else if rhs_inner_stride == 1 {
                            return Self::[<$name _stride_n_1>](lhs, lhs_inner_stride, rhs, dst, rhs_shape[0]);
                        }

                        // neither element is contiguous
                        return Self::[<$name _stride_n_n>](lhs, lhs_inner_stride, rhs, rhs_inner_stride, dst, lhs_shape[0]);
                    }

                    // only 1 operand has a uniform stride
                    if rhs_dims == 1 && rhs_inner_stride == 0 {
                        return Self::[<$name _nonunif_0>](lhs, &lhs_shape, &lhs_stride,
                                                          rhs, dst, rhs_shape[0]);
                    } else if lhs_dims == 1 && lhs_inner_stride == 0 {
                        return Self::[<$name _0_nonunif>](lhs,
                                                          rhs, &rhs_shape, &rhs_stride,
                                                          dst, lhs_shape[0]);
                    }

                    if rhs_dims == 1 && rhs_inner_stride == 1 {
                        return Self::[<$name _nonunif_1>](lhs, &lhs_shape, &lhs_stride,
                                                          rhs, dst, rhs_shape[0]);
                    } else if lhs_dims == 1 && lhs_inner_stride == 1 {
                        return Self::[<$name _1_nonunif>](lhs,
                                                          rhs, &rhs_shape, &rhs_stride,
                                                          dst, lhs_shape[0]);
                    }

                    if rhs_dims == 1 {
                        return Self::[<$name _nonunif_n>](lhs, &lhs_shape, &lhs_stride,
                                                          rhs, rhs_inner_stride,
                                                          dst, rhs_shape[0]);
                    } else if lhs_dims == 1 {
                        return Self::[<$name _n_nonunif>](lhs, lhs_inner_stride,
                                                          rhs, &rhs_shape, &rhs_stride,
                                                          dst, lhs_shape[0]);
                    }

                    // unspecialized loop
                    Self::[<$name _unspecialized>](lhs, &lhs_shape, &lhs_stride,
                                                   rhs, &rhs_shape, &rhs_stride,
                                                   dst);
                }
            }
        }
    }
}

define_binary_op_trait!(BinaryOpRem, Rem, rem, %;
                        i8, i16, i32, i64, i128, isize,
                        u8, u16, u32, u64, u128, usize,
                        f32, f64);

define_binary_op_trait!(BinaryOpBitAnd, BitAnd, bitand, &;
                        i8, i16, i32, i64, i128, isize,
                        u8, u16, u32, u64, u128, usize);

define_binary_op_trait!(BinaryOpBitOr, BitOr, bitor, |;
                        i8, i16, i32, i64, i128, isize,
                        u8, u16, u32, u64, u128, usize);

define_binary_op_trait!(BinaryOpShl, Shl, shl, <<;
                        i8, i16, i32, i64, i128, isize,
                        u8, u16, u32, u64, u128, usize);

define_binary_op_trait!(BinaryOpShr, Shr, shr, >>;
                        i8, i16, i32, i64, i128, isize,
                        u8, u16, u32, u64, u128, usize);
