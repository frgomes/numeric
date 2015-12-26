use tensor::Tensor;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg, BitAnd, BitOr, BitXor};

// T <op> &T

macro_rules! add_impl {
    ($trait_name:ident, $func_name:ident, $func_name_with_mul:ident) => (
        // T <op> T
        impl<T: Copy + $trait_name<Output=T>> $trait_name for Tensor<T> {
            type Output = Tensor<T>;
            fn $func_name(mut self, rhs: Self::Output) -> Self::Output {
                if rhs.is_scalar() {
                    {
                        let n = self.size();
                        let mut data = self.slice_mut();
                        let v: T = rhs.scalar_value() as T;
                        for i in 0..n {
                            data[i] = data[i].$func_name(v);
                        }
                    }
                    self
                } else if self.is_scalar() {
                    let mut t = Tensor::empty(&rhs.shape);
                    {
                        let n = t.size();
                        let mut data = t.slice_mut();
                        let v = self.scalar_value();
                        for i in 0..n {
                            data[i] = v.$func_name(rhs.data[i]);
                        }
                    }
                    t
                } else {
                    {
                        let n = self.size();
                        assert_eq!(self.shape, rhs.shape);
                        let mut data = self.slice_mut();
                        for i in 0..n {
                            data[i] = data[i].$func_name(rhs.data[i]);
                        }
                    }
                    self
                }
            }
        }
        // T <op> &T
        impl<'a, T: Copy + $trait_name<Output=T>> $trait_name<&'a Tensor<T>> for Tensor<T> {
            type Output = Tensor<T>;
            fn $func_name(mut self, rhs: &Self::Output) -> Self::Output {
                if rhs.is_scalar() {
                    {
                        let n = self.size();
                        let mut data = self.slice_mut();
                        let v = rhs[0];
                        for i in 0..n {
                            data[i] = data[i].$func_name(v);
                        }
                    }
                    self
                } else if self.is_scalar() {
                    let mut t: Tensor<T> = Tensor::empty(&rhs.shape);
                    {
                        let n = t.size();
                        let mut data = t.slice_mut();
                        let v = self[0];
                        for i in 0..n {
                            data[i] = v.$func_name(rhs.data[i]);
                        }
                    }
                    t
                } else {
                    {
                        let n = self.size();
                        assert_eq!(self.shape, rhs.shape);
                        let mut data = self.slice_mut();
                        for i in 0..n {
                            data[i] = data[i].$func_name(rhs.data[i]);
                        }
                    }
                    self
                }
            }
        }

        // T <op> &T  (with out)
        impl<T: Copy + $trait_name<Output=T>> Tensor<T> {
            pub fn $func_name_with_mul(&self, rhs: &Tensor<T>, out: &mut Tensor<T>) -> () {
                if rhs.is_scalar() {
                    assert!(out.shape() == self.shape());
                    let n = out.size();
                    let mut data = out.slice_mut();
                    let v = rhs[0];
                    for i in 0..n {
                        data[i] = data[i].$func_name(v);
                    }
                } else if self.is_scalar() {
                    assert!(out.shape() == rhs.shape());
                    let mut data = out.slice_mut();
                    let v = self[0];
                    for i in 0..rhs.size() {
                        data[i] = v.$func_name(rhs.data[i]);
                    }
                } else {
                    assert_eq!(self.shape, rhs.shape);
                    let n = out.size();
                    let mut data = out.slice_mut();
                    for i in 0..n {
                        data[i] = self.data[i].$func_name(rhs.data[i]);
                    }
                }
            }
        }

        // &T <op> &T
        impl<'a, T: Copy + $trait_name<Output=T>> $trait_name<&'a Tensor<T>> for &'a Tensor<T> {
            type Output = Tensor<T>;
            fn $func_name(self, rhs: &Self::Output) -> Self::Output {
                //println!("$fname &T + &T");
                if rhs.is_scalar() {
                    let mut t = self.clone();
                    {
                        let n = t.size();
                        let mut data = t.slice_mut();
                        let v = rhs[0];
                        for i in 0..n {
                            data[i] = data[i].$func_name(v);
                        }
                    }
                    t
                } else if self.is_scalar() {
                    let mut t = Tensor::empty(&rhs.shape);
                    {
                        let n = t.size();
                        let mut data = t.slice_mut();
                        let v = self[0];
                        for i in 0..n {
                            data[i] = v.$func_name(rhs.data[i]);
                        }
                    }
                    t
                } else {
                    assert_eq!(self.shape, rhs.shape);
                    let mut t = self.clone();
                    {
                        let n = t.size();
                        let mut data = t.slice_mut();
                        for i in 0..n {
                            data[i] = data[i].$func_name(rhs.data[i]);
                        }
                    }
                    t
                }
            }
        }

        // T <op> S
        impl<T: Copy + $trait_name<Output=T>> $trait_name<T> for Tensor<T> {
            type Output = Tensor<T>;
            fn $func_name(mut self, rhs: T) -> Self::Output {
                {
                    let n = self.size();
                    let mut data = self.slice_mut();
                    for i in 0..n {
                        data[i] = data[i].$func_name(rhs);
                    }
                }
                self
            }
        }
    )
}

// Any operation supported on T should be supported on Tensor<T>, as long as T supports Copy
add_impl!(Add, add, add_with_out);
add_impl!(Sub, sub, sub_with_out);
add_impl!(Mul, mul, mul_with_out);
add_impl!(Div, div, div_with_out);
add_impl!(Rem, rem, rem_with_out);

add_impl!(BitAnd, bitand, bitand_with_out);
add_impl!(BitOr, bitor, bitor_with_out);
add_impl!(BitXor, bitxor, bitxor_with_out);


// -T
impl<T: Copy + Neg<Output=T>> Neg for Tensor<T> {
    type Output = Tensor<T>;
    fn neg(mut self) -> Self::Output {
        {
            let n = self.size();
            let mut data = self.slice_mut();
            for i in 0..n {
                data[i] = -data[i];
            }
        }
        self
    }
}

// -&T
impl<'a, T: Copy + Neg<Output=T>> Neg for &'a Tensor<T> {
    type Output = Tensor<T>;
    fn neg(self) -> Self::Output {
        let mut t = Tensor::empty(&self.shape);
        {
            let mut data = t.slice_mut();
            for i in 0..self.size() {
                data[i] = -self.data[i];
            }
        }
        t
    }
}