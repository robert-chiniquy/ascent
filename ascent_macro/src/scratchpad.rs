use std::ops::Deref;
use std::{clone, cmp::max, rc::Rc};

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum LambdaCalcExpr {
   Ref(&'static str),
   Lam(&'static str, Rc<LambdaCalcExpr>),
   App(Rc<LambdaCalcExpr>, Rc<LambdaCalcExpr>),
}

use LambdaCalcExpr::*;

impl LambdaCalcExpr {
   #[allow(dead_code)]
   fn depth(&self) -> usize {
      match self {
         LambdaCalcExpr::Ref(_) => 0,
         LambdaCalcExpr::Lam(_x, b) => 1 + b.depth(),
         LambdaCalcExpr::App(f, e) => 1 + max(f.depth(), e.depth()),
      }
   }
}
fn app(f: LambdaCalcExpr, a: LambdaCalcExpr) -> LambdaCalcExpr {
   App(Rc::new(f), Rc::new(a))
}
fn lam(x: &'static str, e: LambdaCalcExpr) -> LambdaCalcExpr {
   Lam(x, Rc::new(e))
}

fn sub(exp: &LambdaCalcExpr, var: &str, e: &LambdaCalcExpr) -> LambdaCalcExpr {
   match exp {
      Ref(x) if *x == var => e.clone(),
      Ref(_x) => exp.clone(),
      App(ef, ea) => app(sub(ef, var, e), sub(ea, var, e)),
      Lam(x, _eb) if *x == var => exp.clone(),
      Lam(x, eb) => lam(x, sub(eb, var, e)),
   }
}

#[allow(non_snake_case)]
fn U() -> LambdaCalcExpr {
   lam("x", app(Ref("x"), Ref("x")))
}
#[allow(non_snake_case)]
fn I() -> LambdaCalcExpr {
   lam("x", Ref("x"))
}

fn min<'a>(inp: impl Iterator<Item = (&'a i32,)>) -> impl Iterator<Item = i32> {
   inp.map(|tuple| tuple.0).min().cloned().into_iter()
}

#[warn(warnings)]
#[allow(unused_imports)]
#[allow(redundant_semicolons)]
#[cfg(test)]
fn _test() {
   use ascent::aggregators::*;
   use ascent::lattice::set::Set;

   pub struct AscentProgram {
      pub output: Vec<(LambdaCalcExpr,)>,
      #[allow(non_snake_case)]
      pub output_indices_0: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)>,
      pub input: Vec<(LambdaCalcExpr,)>,
      #[allow(non_snake_case)]
      pub input_indices_0: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)>,
      #[allow(non_snake_case)]
      pub input_indices_: ascent::internal::RelIndexType<()>,
      pub do_eval: Vec<(LambdaCalcExpr,)>,
      #[allow(non_snake_case)]
      pub do_eval_indices_: ascent::internal::RelIndexType<()>,
      #[allow(non_snake_case)]
      pub do_eval_indices_0: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)>,
      pub eval: Vec<(LambdaCalcExpr, LambdaCalcExpr)>,
      #[allow(non_snake_case)]
      pub eval_indices_0_1: ascent::internal::RelFullIndexType<(LambdaCalcExpr, LambdaCalcExpr)>,
      #[allow(non_snake_case)]
      pub eval_indices_0: ascent::internal::RelIndexType<(LambdaCalcExpr,)>,
      pub scc0_duration: std::time::Duration,
      pub scc1_duration: std::time::Duration,
      pub scc2_duration: std::time::Duration,
      pub scc3_duration: std::time::Duration,
   }
   impl AscentProgram {
      #[allow(unused_imports)]
      #[doc = "Runs the Ascent program to a fixed point."]
      pub fn run(&mut self) {
         macro_rules! __check_return_conditions {
            () => {};
         }
         use core::cmp::PartialEq;
         self.update_indices_priv();
         let _self = self;
         ascent::internal::comment("scc 0");
         {
            let _scc_start_time = ::std::time::Instant::now();
            #[allow(non_snake_case)]
            let input_indices_0_delta: &mut ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> =
               &mut _self.input_indices_0;
            #[allow(non_snake_case)]
            let mut input_indices_0_total: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let mut input_indices_0_new: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let input_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self.input_indices_;
            #[allow(non_snake_case)]
            let mut input_indices__total: ascent::internal::RelIndexType<()> = Default::default();
            #[allow(non_snake_case)]
            let mut input_indices__new: ascent::internal::RelIndexType<()> = Default::default();
            #[allow(unused_assignments, unused_variables)]
            {
               let mut __changed = false;
               ascent::internal::comment("input <-- ");
               let __new_row: (LambdaCalcExpr,) = (app(U(), I()),);
               let __new_row_ind = _self.input.len();
               if !input_indices_0_total.contains_key(&__new_row)
                  && !input_indices_0_delta.contains_key(&__new_row)
                  && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                     &mut input_indices_0_new,
                     &__new_row,
                     __new_row_ind,
                  )
               {
                  ::ascent::internal::RelIndexTrait::index_insert(&mut input_indices__new, (), __new_row_ind);
                  _self.input.push(__new_row);
                  __changed = true;
               }
               ascent::internal::RelIndexTrait::move_index_contents(input_indices_0_delta, &mut input_indices_0_total);
               std::mem::swap(&mut input_indices_0_new, input_indices_0_delta);
               ascent::internal::RelIndexTrait::move_index_contents(input_indices__delta, &mut input_indices__total);
               std::mem::swap(&mut input_indices__new, input_indices__delta);
               ascent::internal::RelIndexTrait::move_index_contents(input_indices_0_delta, &mut input_indices_0_total);
               std::mem::swap(&mut input_indices_0_new, input_indices_0_delta);
               ascent::internal::RelIndexTrait::move_index_contents(input_indices__delta, &mut input_indices__total);
               std::mem::swap(&mut input_indices__new, input_indices__delta);
               __check_return_conditions!();
            }
            _self.input_indices_0 = input_indices_0_total;
            _self.input_indices_ = input_indices__total;
            _self.scc0_duration += _scc_start_time.elapsed();
         }
         ascent::internal::comment("scc 1");
         {
            let _scc_start_time = ::std::time::Instant::now();
            #[allow(non_snake_case)]
            let do_eval_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self.do_eval_indices_;
            #[allow(non_snake_case)]
            let mut do_eval_indices__total: ascent::internal::RelIndexType<()> = Default::default();
            #[allow(non_snake_case)]
            let mut do_eval_indices__new: ascent::internal::RelIndexType<()> = Default::default();
            #[allow(non_snake_case)]
            let do_eval_indices_0_delta: &mut ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> =
               &mut _self.do_eval_indices_0;
            #[allow(non_snake_case)]
            let mut do_eval_indices_0_total: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let mut do_eval_indices_0_new: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let input_indices__total: &mut ascent::internal::RelIndexType<()> = &mut _self.input_indices_;
            #[allow(unused_assignments, unused_variables)]
            {
               let mut __changed = false;
               ascent::internal::comment("do_eval <-- input_indices__total");
               if let Some(__matching) = input_indices__total.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.input[__ind].clone();
                     let exp = &__row.0;
                     let __new_row: (LambdaCalcExpr,) = (exp.clone(),);
                     let __new_row_ind = _self.do_eval.len();
                     if !do_eval_indices_0_total.contains_key(&__new_row)
                        && !do_eval_indices_0_delta.contains_key(&__new_row)
                        && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                           &mut do_eval_indices_0_new,
                           &__new_row,
                           __new_row_ind,
                        )
                     {
                        ::ascent::internal::RelIndexTrait::index_insert(&mut do_eval_indices__new, (), __new_row_ind);
                        _self.do_eval.push(__new_row);
                        __changed = true;
                     }
                  }
               }
               ascent::internal::RelIndexTrait::move_index_contents(
                  do_eval_indices__delta,
                  &mut do_eval_indices__total,
               );
               std::mem::swap(&mut do_eval_indices__new, do_eval_indices__delta);
               ascent::internal::RelIndexTrait::move_index_contents(
                  do_eval_indices_0_delta,
                  &mut do_eval_indices_0_total,
               );
               std::mem::swap(&mut do_eval_indices_0_new, do_eval_indices_0_delta);
               ascent::internal::RelIndexTrait::move_index_contents(
                  do_eval_indices__delta,
                  &mut do_eval_indices__total,
               );
               std::mem::swap(&mut do_eval_indices__new, do_eval_indices__delta);
               ascent::internal::RelIndexTrait::move_index_contents(
                  do_eval_indices_0_delta,
                  &mut do_eval_indices_0_total,
               );
               std::mem::swap(&mut do_eval_indices_0_new, do_eval_indices_0_delta);
               __check_return_conditions!();
            }
            _self.do_eval_indices_ = do_eval_indices__total;
            _self.do_eval_indices_0 = do_eval_indices_0_total;
            _self.scc1_duration += _scc_start_time.elapsed();
         }
         ascent::internal::comment("scc 2");
         {
            let _scc_start_time = ::std::time::Instant::now();
            #[allow(non_snake_case)]
            let eval_indices_0_1_delta: &mut ascent::internal::RelFullIndexType<(
               LambdaCalcExpr,
               LambdaCalcExpr,
            )> = &mut _self.eval_indices_0_1;
            #[allow(non_snake_case)]
            let mut eval_indices_0_1_total: ascent::internal::RelFullIndexType<(
               LambdaCalcExpr,
               LambdaCalcExpr,
            )> = Default::default();
            #[allow(non_snake_case)]
            let mut eval_indices_0_1_new: ascent::internal::RelFullIndexType<(
               LambdaCalcExpr,
               LambdaCalcExpr,
            )> = Default::default();
            #[allow(non_snake_case)]
            let eval_indices_0_delta: &mut ascent::internal::RelIndexType<(LambdaCalcExpr,)> =
               &mut _self.eval_indices_0;
            #[allow(non_snake_case)]
            let mut eval_indices_0_total: ascent::internal::RelIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let mut eval_indices_0_new: ascent::internal::RelIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let do_eval_indices_0_delta: &mut ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> =
               &mut _self.do_eval_indices_0;
            #[allow(non_snake_case)]
            let mut do_eval_indices_0_total: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let mut do_eval_indices_0_new: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let do_eval_indices__delta: &mut ascent::internal::RelIndexType<()> = &mut _self.do_eval_indices_;
            #[allow(non_snake_case)]
            let mut do_eval_indices__total: ascent::internal::RelIndexType<()> = Default::default();
            #[allow(non_snake_case)]
            let mut do_eval_indices__new: ascent::internal::RelIndexType<()> = Default::default();
            #[allow(unused_assignments, unused_variables)]
            loop {
               let mut __changed = false;
               ascent::internal::comment("eval <-- do_eval_indices__delta, eval_indices_0_total, eval_indices_0_total");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_5 = &__row.0;
                     if let exp @ App(ef, ea) = _arg_pattern_5 {
                        if let Some(__matching) = eval_indices_0_total.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_6 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_6 {
                                 if let Some(__matching) = eval_indices_0_total.get(&((sub(fb, fx, ea)).clone(),)) {
                                    for &__ind in __matching.iter() {
                                       let __row = &_self.eval[__ind].clone();
                                       let final_res = &__row.1;
                                       let __new_row: (LambdaCalcExpr, LambdaCalcExpr) =
                                          (exp.clone(), final_res.clone());
                                       let __new_row_ind = _self.eval.len();
                                       if !eval_indices_0_1_total.contains_key(&__new_row)
                                          && !eval_indices_0_1_delta.contains_key(&__new_row)
                                          && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                             &mut eval_indices_0_1_new,
                                             &__new_row,
                                             __new_row_ind,
                                          )
                                       {
                                          ::ascent::internal::RelIndexTrait::index_insert(
                                             &mut eval_indices_0_new,
                                             (__new_row.0.clone(),),
                                             __new_row_ind,
                                          );
                                          _self.eval.push(__new_row);
                                          __changed = true;
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__total, eval_indices_0_delta, eval_indices_0_total");
               if let Some(__matching) = do_eval_indices__total.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_5 = &__row.0;
                     if let exp @ App(ef, ea) = _arg_pattern_5 {
                        if let Some(__matching) = eval_indices_0_delta.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_6 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_6 {
                                 if let Some(__matching) = eval_indices_0_total.get(&((sub(fb, fx, ea)).clone(),)) {
                                    for &__ind in __matching.iter() {
                                       let __row = &_self.eval[__ind].clone();
                                       let final_res = &__row.1;
                                       let __new_row: (LambdaCalcExpr, LambdaCalcExpr) =
                                          (exp.clone(), final_res.clone());
                                       let __new_row_ind = _self.eval.len();
                                       if !eval_indices_0_1_total.contains_key(&__new_row)
                                          && !eval_indices_0_1_delta.contains_key(&__new_row)
                                          && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                             &mut eval_indices_0_1_new,
                                             &__new_row,
                                             __new_row_ind,
                                          )
                                       {
                                          ::ascent::internal::RelIndexTrait::index_insert(
                                             &mut eval_indices_0_new,
                                             (__new_row.0.clone(),),
                                             __new_row_ind,
                                          );
                                          _self.eval.push(__new_row);
                                          __changed = true;
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__delta, eval_indices_0_delta, eval_indices_0_total");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_5 = &__row.0;
                     if let exp @ App(ef, ea) = _arg_pattern_5 {
                        if let Some(__matching) = eval_indices_0_delta.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_6 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_6 {
                                 if let Some(__matching) = eval_indices_0_total.get(&((sub(fb, fx, ea)).clone(),)) {
                                    for &__ind in __matching.iter() {
                                       let __row = &_self.eval[__ind].clone();
                                       let final_res = &__row.1;
                                       let __new_row: (LambdaCalcExpr, LambdaCalcExpr) =
                                          (exp.clone(), final_res.clone());
                                       let __new_row_ind = _self.eval.len();
                                       if !eval_indices_0_1_total.contains_key(&__new_row)
                                          && !eval_indices_0_1_delta.contains_key(&__new_row)
                                          && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                             &mut eval_indices_0_1_new,
                                             &__new_row,
                                             __new_row_ind,
                                          )
                                       {
                                          ::ascent::internal::RelIndexTrait::index_insert(
                                             &mut eval_indices_0_new,
                                             (__new_row.0.clone(),),
                                             __new_row_ind,
                                          );
                                          _self.eval.push(__new_row);
                                          __changed = true;
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__total, eval_indices_0_total, eval_indices_0_delta");
               if let Some(__matching) = do_eval_indices__total.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_5 = &__row.0;
                     if let exp @ App(ef, ea) = _arg_pattern_5 {
                        if let Some(__matching) = eval_indices_0_total.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_6 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_6 {
                                 if let Some(__matching) = eval_indices_0_delta.get(&((sub(fb, fx, ea)).clone(),)) {
                                    for &__ind in __matching.iter() {
                                       let __row = &_self.eval[__ind].clone();
                                       let final_res = &__row.1;
                                       let __new_row: (LambdaCalcExpr, LambdaCalcExpr) =
                                          (exp.clone(), final_res.clone());
                                       let __new_row_ind = _self.eval.len();
                                       if !eval_indices_0_1_total.contains_key(&__new_row)
                                          && !eval_indices_0_1_delta.contains_key(&__new_row)
                                          && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                             &mut eval_indices_0_1_new,
                                             &__new_row,
                                             __new_row_ind,
                                          )
                                       {
                                          ::ascent::internal::RelIndexTrait::index_insert(
                                             &mut eval_indices_0_new,
                                             (__new_row.0.clone(),),
                                             __new_row_ind,
                                          );
                                          _self.eval.push(__new_row);
                                          __changed = true;
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__delta, eval_indices_0_total, eval_indices_0_delta");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_5 = &__row.0;
                     if let exp @ App(ef, ea) = _arg_pattern_5 {
                        if let Some(__matching) = eval_indices_0_total.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_6 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_6 {
                                 if let Some(__matching) = eval_indices_0_delta.get(&((sub(fb, fx, ea)).clone(),)) {
                                    for &__ind in __matching.iter() {
                                       let __row = &_self.eval[__ind].clone();
                                       let final_res = &__row.1;
                                       let __new_row: (LambdaCalcExpr, LambdaCalcExpr) =
                                          (exp.clone(), final_res.clone());
                                       let __new_row_ind = _self.eval.len();
                                       if !eval_indices_0_1_total.contains_key(&__new_row)
                                          && !eval_indices_0_1_delta.contains_key(&__new_row)
                                          && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                             &mut eval_indices_0_1_new,
                                             &__new_row,
                                             __new_row_ind,
                                          )
                                       {
                                          ::ascent::internal::RelIndexTrait::index_insert(
                                             &mut eval_indices_0_new,
                                             (__new_row.0.clone(),),
                                             __new_row_ind,
                                          );
                                          _self.eval.push(__new_row);
                                          __changed = true;
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__total, eval_indices_0_delta, eval_indices_0_delta");
               if let Some(__matching) = do_eval_indices__total.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_5 = &__row.0;
                     if let exp @ App(ef, ea) = _arg_pattern_5 {
                        if let Some(__matching) = eval_indices_0_delta.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_6 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_6 {
                                 if let Some(__matching) = eval_indices_0_delta.get(&((sub(fb, fx, ea)).clone(),)) {
                                    for &__ind in __matching.iter() {
                                       let __row = &_self.eval[__ind].clone();
                                       let final_res = &__row.1;
                                       let __new_row: (LambdaCalcExpr, LambdaCalcExpr) =
                                          (exp.clone(), final_res.clone());
                                       let __new_row_ind = _self.eval.len();
                                       if !eval_indices_0_1_total.contains_key(&__new_row)
                                          && !eval_indices_0_1_delta.contains_key(&__new_row)
                                          && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                             &mut eval_indices_0_1_new,
                                             &__new_row,
                                             __new_row_ind,
                                          )
                                       {
                                          ::ascent::internal::RelIndexTrait::index_insert(
                                             &mut eval_indices_0_new,
                                             (__new_row.0.clone(),),
                                             __new_row_ind,
                                          );
                                          _self.eval.push(__new_row);
                                          __changed = true;
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__delta, eval_indices_0_delta, eval_indices_0_delta");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_5 = &__row.0;
                     if let exp @ App(ef, ea) = _arg_pattern_5 {
                        if let Some(__matching) = eval_indices_0_delta.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_6 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_6 {
                                 if let Some(__matching) = eval_indices_0_delta.get(&((sub(fb, fx, ea)).clone(),)) {
                                    for &__ind in __matching.iter() {
                                       let __row = &_self.eval[__ind].clone();
                                       let final_res = &__row.1;
                                       let __new_row: (LambdaCalcExpr, LambdaCalcExpr) =
                                          (exp.clone(), final_res.clone());
                                       let __new_row_ind = _self.eval.len();
                                       if !eval_indices_0_1_total.contains_key(&__new_row)
                                          && !eval_indices_0_1_delta.contains_key(&__new_row)
                                          && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                             &mut eval_indices_0_1_new,
                                             &__new_row,
                                             __new_row_ind,
                                          )
                                       {
                                          ::ascent::internal::RelIndexTrait::index_insert(
                                             &mut eval_indices_0_new,
                                             (__new_row.0.clone(),),
                                             __new_row_ind,
                                          );
                                          _self.eval.push(__new_row);
                                          __changed = true;
                                       }
                                    }
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__delta");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_ = &__row.0;
                     if let exp @ Ref(_) = _arg_pattern_ {
                        let __new_row: (LambdaCalcExpr, LambdaCalcExpr) = (exp.clone(), exp.clone());
                        let __new_row_ind = _self.eval.len();
                        if !eval_indices_0_1_total.contains_key(&__new_row)
                           && !eval_indices_0_1_delta.contains_key(&__new_row)
                           && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                              &mut eval_indices_0_1_new,
                              &__new_row,
                              __new_row_ind,
                           )
                        {
                           ::ascent::internal::RelIndexTrait::index_insert(
                              &mut eval_indices_0_new,
                              (__new_row.0.clone(),),
                              __new_row_ind,
                           );
                           _self.eval.push(__new_row);
                           __changed = true;
                        }
                     }
                  }
               }
               ascent::internal::comment("eval <-- do_eval_indices__delta, if let ⋯");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let exp = &__row.0;
                     if let Lam(_, _) = exp {
                        let __new_row: (LambdaCalcExpr, LambdaCalcExpr) = (exp.clone(), exp.clone());
                        let __new_row_ind = _self.eval.len();
                        if !eval_indices_0_1_total.contains_key(&__new_row)
                           && !eval_indices_0_1_delta.contains_key(&__new_row)
                           && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                              &mut eval_indices_0_1_new,
                              &__new_row,
                              __new_row_ind,
                           )
                        {
                           ::ascent::internal::RelIndexTrait::index_insert(
                              &mut eval_indices_0_new,
                              (__new_row.0.clone(),),
                              __new_row_ind,
                           );
                           _self.eval.push(__new_row);
                           __changed = true;
                        }
                     }
                  }
               }
               ascent::internal::comment("do_eval <-- do_eval_indices__delta");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_1 = &__row.0;
                     if let App(ef, _ea) = _arg_pattern_1 {
                        let __new_row: (LambdaCalcExpr,) = (ef.as_ref().clone(),);
                        let __new_row_ind = _self.do_eval.len();
                        if !do_eval_indices_0_total.contains_key(&__new_row)
                           && !do_eval_indices_0_delta.contains_key(&__new_row)
                           && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                              &mut do_eval_indices_0_new,
                              &__new_row,
                              __new_row_ind,
                           )
                        {
                           ::ascent::internal::RelIndexTrait::index_insert(
                              &mut do_eval_indices__new,
                              (),
                              __new_row_ind,
                           );
                           _self.do_eval.push(__new_row);
                           __changed = true;
                        }
                     }
                  }
               }
               ascent::internal::comment("do_eval <-- do_eval_indices__delta, eval_indices_0_total");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_3 = &__row.0;
                     if let App(ef, ea) = _arg_pattern_3 {
                        if let Some(__matching) = eval_indices_0_total.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_4 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_4 {
                                 let __new_row: (LambdaCalcExpr,) = (sub(fb, fx, ea),);
                                 let __new_row_ind = _self.do_eval.len();
                                 if !do_eval_indices_0_total.contains_key(&__new_row)
                                    && !do_eval_indices_0_delta.contains_key(&__new_row)
                                    && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                       &mut do_eval_indices_0_new,
                                       &__new_row,
                                       __new_row_ind,
                                    )
                                 {
                                    ::ascent::internal::RelIndexTrait::index_insert(
                                       &mut do_eval_indices__new,
                                       (),
                                       __new_row_ind,
                                    );
                                    _self.do_eval.push(__new_row);
                                    __changed = true;
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("do_eval <-- do_eval_indices__total, eval_indices_0_delta");
               if let Some(__matching) = do_eval_indices__total.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_3 = &__row.0;
                     if let App(ef, ea) = _arg_pattern_3 {
                        if let Some(__matching) = eval_indices_0_delta.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_4 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_4 {
                                 let __new_row: (LambdaCalcExpr,) = (sub(fb, fx, ea),);
                                 let __new_row_ind = _self.do_eval.len();
                                 if !do_eval_indices_0_total.contains_key(&__new_row)
                                    && !do_eval_indices_0_delta.contains_key(&__new_row)
                                    && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                       &mut do_eval_indices_0_new,
                                       &__new_row,
                                       __new_row_ind,
                                    )
                                 {
                                    ::ascent::internal::RelIndexTrait::index_insert(
                                       &mut do_eval_indices__new,
                                       (),
                                       __new_row_ind,
                                    );
                                    _self.do_eval.push(__new_row);
                                    __changed = true;
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::comment("do_eval <-- do_eval_indices__delta, eval_indices_0_delta");
               if let Some(__matching) = do_eval_indices__delta.get(&()) {
                  for &__ind in __matching.iter() {
                     let __row = &_self.do_eval[__ind].clone();
                     let _arg_pattern_3 = &__row.0;
                     if let App(ef, ea) = _arg_pattern_3 {
                        if let Some(__matching) = eval_indices_0_delta.get(&((ef.deref()).clone(),)) {
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let _arg_pattern_4 = &__row.1;
                              if let Lam(fx, fb) = _arg_pattern_4 {
                                 let __new_row: (LambdaCalcExpr,) = (sub(fb, fx, ea),);
                                 let __new_row_ind = _self.do_eval.len();
                                 if !do_eval_indices_0_total.contains_key(&__new_row)
                                    && !do_eval_indices_0_delta.contains_key(&__new_row)
                                    && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                       &mut do_eval_indices_0_new,
                                       &__new_row,
                                       __new_row_ind,
                                    )
                                 {
                                    ::ascent::internal::RelIndexTrait::index_insert(
                                       &mut do_eval_indices__new,
                                       (),
                                       __new_row_ind,
                                    );
                                    _self.do_eval.push(__new_row);
                                    __changed = true;
                                 }
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::RelIndexTrait::move_index_contents(
                  eval_indices_0_1_delta,
                  &mut eval_indices_0_1_total,
               );
               std::mem::swap(&mut eval_indices_0_1_new, eval_indices_0_1_delta);
               ascent::internal::RelIndexTrait::move_index_contents(eval_indices_0_delta, &mut eval_indices_0_total);
               std::mem::swap(&mut eval_indices_0_new, eval_indices_0_delta);
               ascent::internal::RelIndexTrait::move_index_contents(
                  do_eval_indices_0_delta,
                  &mut do_eval_indices_0_total,
               );
               std::mem::swap(&mut do_eval_indices_0_new, do_eval_indices_0_delta);
               ascent::internal::RelIndexTrait::move_index_contents(
                  do_eval_indices__delta,
                  &mut do_eval_indices__total,
               );
               std::mem::swap(&mut do_eval_indices__new, do_eval_indices__delta);
               if !__changed {
                  break;
               }
               __check_return_conditions!();
            }
            _self.eval_indices_0_1 = eval_indices_0_1_total;
            _self.eval_indices_0 = eval_indices_0_total;
            _self.do_eval_indices_0 = do_eval_indices_0_total;
            _self.do_eval_indices_ = do_eval_indices__total;
            _self.scc2_duration += _scc_start_time.elapsed();
         }
         ascent::internal::comment("scc 3");
         {
            let _scc_start_time = ::std::time::Instant::now();
            #[allow(non_snake_case)]
            let output_indices_0_delta: &mut ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> =
               &mut _self.output_indices_0;
            #[allow(non_snake_case)]
            let mut output_indices_0_total: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let mut output_indices_0_new: ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> = Default::default();
            #[allow(non_snake_case)]
            let eval_indices_0_total: &mut ascent::internal::RelIndexType<(LambdaCalcExpr,)> =
               &mut _self.eval_indices_0;
            #[allow(non_snake_case)]
            let input_indices_0_total: &mut ascent::internal::RelFullIndexType<(LambdaCalcExpr,)> =
               &mut _self.input_indices_0;
            #[allow(unused_assignments, unused_variables)]
            {
               let mut __changed = false;
               ascent::internal::comment("output <-- input_indices_0_total, eval_indices_0_total [SIMPLE JOIN]");
               if input_indices_0_total.len() <= eval_indices_0_total.len() {
                  for (__cl1_joined_columns, __cl1_tuple_indices) in input_indices_0_total.iter() {
                     let exp = &__cl1_joined_columns.0;
                     if let Some(__matching) = eval_indices_0_total.get(&(exp.clone(),)) {
                        for &cl1_ind in [__cl1_tuple_indices] {
                           let __row = &_self.input[cl1_ind].clone();
                           for &__ind in __matching.iter() {
                              let __row = &_self.eval[__ind].clone();
                              let res = &__row.1;
                              let __new_row: (LambdaCalcExpr,) = (res.clone(),);
                              let __new_row_ind = _self.output.len();
                              if !output_indices_0_total.contains_key(&__new_row)
                                 && !output_indices_0_delta.contains_key(&__new_row)
                                 && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                    &mut output_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                 )
                              {
                                 _self.output.push(__new_row);
                                 __changed = true;
                              }
                           }
                        }
                     }
                  }
               } else {
                  for (__cl1_joined_columns, __cl1_tuple_indices) in eval_indices_0_total.iter() {
                     let exp = &__cl1_joined_columns.0;
                     if let Some(__matching) = input_indices_0_total.get(&(exp.clone(),)) {
                        for &cl1_ind in __cl1_tuple_indices.iter() {
                           let __row = &_self.eval[cl1_ind].clone();
                           let res = &__row.1;
                           for &__ind in [__matching] {
                              let __row = &_self.input[__ind].clone();
                              let __new_row: (LambdaCalcExpr,) = (res.clone(),);
                              let __new_row_ind = _self.output.len();
                              if !output_indices_0_total.contains_key(&__new_row)
                                 && !output_indices_0_delta.contains_key(&__new_row)
                                 && ::ascent::internal::RelFullIndexTrait::insert_if_not_present(
                                    &mut output_indices_0_new,
                                    &__new_row,
                                    __new_row_ind,
                                 )
                              {
                                 _self.output.push(__new_row);
                                 __changed = true;
                              }
                           }
                        }
                     }
                  }
               }
               ascent::internal::RelIndexTrait::move_index_contents(
                  output_indices_0_delta,
                  &mut output_indices_0_total,
               );
               std::mem::swap(&mut output_indices_0_new, output_indices_0_delta);
               ascent::internal::RelIndexTrait::move_index_contents(
                  output_indices_0_delta,
                  &mut output_indices_0_total,
               );
               std::mem::swap(&mut output_indices_0_new, output_indices_0_delta);
               __check_return_conditions!();
            }
            _self.output_indices_0 = output_indices_0_total;
            _self.scc3_duration += _scc_start_time.elapsed();
         }
      }
      fn update_indices_priv(&mut self) {
         for (i, tuple) in self.output.iter().enumerate() {
            let selection_tuple = (tuple.0.clone(),);
            ascent::internal::RelIndexTrait::index_insert(&mut self.output_indices_0, selection_tuple, i);
         }
         for (i, tuple) in self.input.iter().enumerate() {
            let selection_tuple = (tuple.0.clone(),);
            ascent::internal::RelIndexTrait::index_insert(&mut self.input_indices_0, selection_tuple, i);
            let selection_tuple = ();
            ascent::internal::RelIndexTrait::index_insert(&mut self.input_indices_, selection_tuple, i);
         }
         for (i, tuple) in self.do_eval.iter().enumerate() {
            let selection_tuple = ();
            ascent::internal::RelIndexTrait::index_insert(&mut self.do_eval_indices_, selection_tuple, i);
            let selection_tuple = (tuple.0.clone(),);
            ascent::internal::RelIndexTrait::index_insert(&mut self.do_eval_indices_0, selection_tuple, i);
         }
         for (i, tuple) in self.eval.iter().enumerate() {
            let selection_tuple = (tuple.0.clone(), tuple.1.clone());
            ascent::internal::RelIndexTrait::index_insert(&mut self.eval_indices_0_1, selection_tuple, i);
            let selection_tuple = (tuple.0.clone(),);
            ascent::internal::RelIndexTrait::index_insert(&mut self.eval_indices_0, selection_tuple, i);
         }
      }
      #[deprecated = "Explicit call to update_indices not required anymore."]
      pub fn update_indices(&mut self) {
         self.update_indices_priv();
      }
      #[allow(unused_imports)]
      fn type_constaints() {
         let _type_constraints: ascent::internal::TypeConstraints<LambdaCalcExpr>;
      }
      pub fn summary() -> &'static str {
         "scc 0, is_looping: false:\n  input <-- \n  dynamic relations: input\nscc 1, is_looping: false:\n  do_eval <-- input_indices__total\n  dynamic relations: do_eval\nscc 2, is_looping: true:\n  eval <-- do_eval_indices__delta, eval_indices_0_total, eval_indices_0_total\n  eval <-- do_eval_indices__total, eval_indices_0_delta, eval_indices_0_total\n  eval <-- do_eval_indices__delta, eval_indices_0_delta, eval_indices_0_total\n  eval <-- do_eval_indices__total, eval_indices_0_total, eval_indices_0_delta\n  eval <-- do_eval_indices__delta, eval_indices_0_total, eval_indices_0_delta\n  eval <-- do_eval_indices__total, eval_indices_0_delta, eval_indices_0_delta\n  eval <-- do_eval_indices__delta, eval_indices_0_delta, eval_indices_0_delta\n  eval <-- do_eval_indices__delta\n  eval <-- do_eval_indices__delta, if let ⋯\n  do_eval <-- do_eval_indices__delta\n  do_eval <-- do_eval_indices__delta, eval_indices_0_total\n  do_eval <-- do_eval_indices__total, eval_indices_0_delta\n  do_eval <-- do_eval_indices__delta, eval_indices_0_delta\n  dynamic relations: eval, do_eval\nscc 3, is_looping: false:\n  output <-- input_indices_0_total, eval_indices_0_total [SIMPLE JOIN]\n  dynamic relations: output\n"
      }
      pub fn relation_sizes_summary(&self) -> String {
         use std::fmt::Write;
         let mut res = String::new();
         writeln!(&mut res, "{} size: {}", "output", self.output.len()).unwrap();
         writeln!(&mut res, "{} size: {}", "input", self.input.len()).unwrap();
         writeln!(&mut res, "{} size: {}", "do_eval", self.do_eval.len()).unwrap();
         writeln!(&mut res, "{} size: {}", "eval", self.eval.len()).unwrap();
         res
      }
      pub fn scc_times_summary(&self) -> String {
         use std::fmt::Write;
         let mut res = String::new();
         writeln!(&mut res, "scc {} time: {:?}", "0", self.scc0_duration).unwrap();
         writeln!(&mut res, "scc {} time: {:?}", "1", self.scc1_duration).unwrap();
         writeln!(&mut res, "scc {} time: {:?}", "2", self.scc2_duration).unwrap();
         writeln!(&mut res, "scc {} time: {:?}", "3", self.scc3_duration).unwrap();
         res
      }
   }
   impl Default for AscentProgram {
      fn default() -> Self {
         let mut _self = AscentProgram {
            output: Default::default(),
            output_indices_0: Default::default(),
            input: Default::default(),
            input_indices_0: Default::default(),
            input_indices_: Default::default(),
            do_eval: Default::default(),
            do_eval_indices_: Default::default(),
            do_eval_indices_0: Default::default(),
            eval: Default::default(),
            eval_indices_0_1: Default::default(),
            eval_indices_0: Default::default(),
            scc0_duration: std::time::Duration::ZERO,
            scc1_duration: std::time::Duration::ZERO,
            scc2_duration: std::time::Duration::ZERO,
            scc3_duration: std::time::Duration::ZERO,
         };
         _self
      }
   };
}
