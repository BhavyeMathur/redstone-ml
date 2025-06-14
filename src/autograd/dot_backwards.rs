use crate::gradient_function::{GradientFuncTrait, GradientFunction};
use crate::{call_next_backward, FloatDataType, NdArray, Tensor};
use std::cell::RefCell;
use std::rc::Rc;


pub(crate) struct DotBackwards<T: FloatDataType> {
    next_functions: [GradientFunction<T>; 2],

    lhs: Rc<NdArray<'static, T>>,
    rhs: Rc<NdArray<'static, T>>,
}


impl<T: FloatDataType> GradientFuncTrait<T> for DotBackwards<T> {
    fn backward(&mut self, grad: &NdArray<T>) {
        call_next_backward!(self.rhs.as_ref() * grad,
                            self.next_functions[0]);
        
        call_next_backward!(self.lhs.as_ref() * grad,
                            self.next_functions[1]);
    }
}

impl<T: FloatDataType> DotBackwards<T> {
    pub(crate) fn new(lhs: &Tensor<T>, rhs: &Tensor<T>) -> GradientFunction<T> {
        Rc::new(RefCell::new(Self {
            next_functions: [lhs.grad_fn(), rhs.grad_fn()],
            lhs: lhs.get_ndarray(),
            rhs: rhs.get_ndarray(),
        }))
    }
}
