// use ndarray::prelude::*;
// use ndarray::{Array2, Array, ScalarOperand};
// use libnum::{Num, FromPrimitive, pow};
//
// fn incremental_mean_and_var<T>(x: &Array2<T>, last_mean: &Array1<T>, last_variance: &Array1<T>, last_sample_count: usize)
//     where T: Num + Clone + FromPrimitive + ScalarOperand
// {
// 	let last_sample_count: T = FromPrimitive::from_usize(last_sample_count).unwrap();
// 	let last_sum = last_mean * last_sample_count;
//     let new_sum = x.sum_axis(Axis(0));
//
//     let new_sample_count = FromPrimitive::from_usize(x.len_of(Axis(0))).unwrap();
//     let updated_sample_count = last_sample_count + new_sample_count;
//
//     let updated_mean = (last_sum + new_sum) / updated_sample_count;
//
//     // We need a function to compute the variance!
//     let new_unnormalized_variance = x.var_axis(Axis(0)) * new_sample_count;
//     if last_sample_count == FromPrimitive::from_usize(0).unwrap() {
//         let updated_unnormalized_variance = new_unnormalized_variance;
//     } else {
//         let last_over_new_count = last_sample_count / new_sample_count;
//         let last_unnormalized_variance = last_variance * last_sample_count;
//         let updated_unnormalized_variance = (
//             last_unnormalized_variance +
//             new_unnormalized_variance +
//             last_over_new_count /
// 			updated_sample_count *
//             pow(last_sum / last_over_new_count - new_sum, 2)
//         );
//         let updated_variance = updated_unnormalized_variance / updated_sample_count;
//     }
// }
