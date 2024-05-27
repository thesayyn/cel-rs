use std::collections::HashMap;
use std::rc::Rc;

use crate::parser::{Atom, Expression, Member, RelationOp};
use crate::value::value::Val;
use crate::Context;

#[derive(Default)]
pub struct Eval {}

impl Eval {
    fn eval_function(
        &self,
        name: Rc<String>,
        receiver: Option<Val>,
        argexprs: Vec<Expression>,
        ctx: &mut Context,
    ) -> Val {
        let mut args = Vec::with_capacity(argexprs.len() + 1);

        if let Some(expr) = receiver {
            args.push(expr)
        }

        for expr in argexprs {
            args.push(self.eval(expr, ctx));
        }

        if let Some(func) = ctx.resolve_function(&name) {
            return (func.overloads.first().unwrap().func)(args)
        }
        Val::new_error("unknown func".to_string())
    }
    fn eval_member(&self, expr: Box<Expression>, member: Box<Member>, ctx: &mut Context) -> Val {
        let v = self.eval(*expr, ctx);
        match *member {
            crate::parser::Member::Attribute(attr) => todo!(),
            crate::parser::Member::FunctionCall(name, argexprs) => {
                self.eval_function(name, Some(v), argexprs, ctx)
            }
            crate::parser::Member::Index(i) => todo!(),
            crate::parser::Member::Fields(_) => todo!(),
        }
    }

    pub fn eval(&self, expr: Expression, ctx: &mut Context) -> Val {
        match expr {
            Expression::GlobalFunctionCall(name, argexprs) => {
                self.eval_function(name, None, argexprs, ctx)
            }
            Expression::Arithmetic(_, _, _) => todo!(),
            Expression::Relation(left, op, right) => {
                let l = self.eval(*left, ctx);
                let r = self.eval(*right, ctx);
                Val::new_bool(match op {
                    RelationOp::Equals => l.eq(&r),
                    RelationOp::LessThan => l.lt(&r),
                    RelationOp::LessThanEq => l.le(&r),
                    RelationOp::GreaterThan => l.gt(&r),
                    RelationOp::GreaterThanEq => l.ge(&r),
                    RelationOp::NotEquals => l.ne(&r),
                    RelationOp::In => todo!("in"),
                })
            }
            Expression::Ternary(_, _, _) => todo!(),
            Expression::Or(_, _) => todo!(),
            Expression::And(_, _) => todo!(),
            Expression::Unary(_, _) => todo!(),
            Expression::Member(expr, member) => self.eval_member(expr, member, ctx),
            Expression::List(values) => self.eval_list(values, ctx),
            Expression::Map(entries) => self.eval_map(entries, ctx),
            Expression::Atom(atom) => self.eval_atom(atom, ctx),
            Expression::Ident(ident) => ctx
                .resolve_variable(&ident)
                .unwrap_or(&Val::new_error(format!("unknown variable {}", ident)))
                .to_owned(),
        }
    }

    fn eval_map(&self, entries: Vec<(Expression, Expression)>, ctx: &mut Context) -> Val {
        let mut map = HashMap::with_capacity(entries.len());
        for (kexpr, vexpr) in entries {
            let k = self.eval(kexpr, ctx);
            let v = self.eval(vexpr, ctx);
            map.insert(k, v);
        }
        Val::new_map(Rc::new(map))
    }

    fn eval_list(&self, elems: Vec<Expression>, ctx: &mut Context) -> Val {
        let mut list = Vec::with_capacity(elems.len());
        for expr in elems {
            let v = self.eval(expr, ctx);
            list.push(v);
        }
        Val::new_list(Rc::new(list))
    }

    pub fn eval_atom(&self, atom: Atom, ctx: &mut Context) -> Val {
        match atom {
            Atom::Int(i) => Val::new_int(i),
            Atom::UInt(u) => Val::new_uint(u),
            Atom::Float(f) => Val::new_double(f),
            Atom::String(s) => Val::new_string(s),
            Atom::Bytes(b) => Val::new_bytes(b),
            Atom::Bool(b) => Val::new_bool(b),
            Atom::Null => Val::new_null(),
        }
    }

    // pub fn eval(&self, expr: Expression, ctx: &mut Context) -> impl Bag {
    //     match expr {
    //         Expression::Atom(atom) => Value::from(atom),
    //         Expression::Relation(left, op, right) => {
    //             let left = self.eval(*left, ctx).unpack();
    //             let right = self.eval(*right, ctx).unpack();
    //             let result = match op {
    //                 RelationOp::Equals => left.eq(&right),
    //                 RelationOp::LessThan => todo!("lt"),
    //                 RelationOp::LessThanEq => todo!("lte"),
    //                 RelationOp::GreaterThan => todo!("gt"),
    //                 RelationOp::GreaterThanEq => todo!("gte"),
    //                 RelationOp::NotEquals => todo!("ne"),
    //                 RelationOp::In => todo!("in"),
    //             };
    //             Value::Bool(result)
    //         }
    //         Expression::Arithmetic(left, op, right) => {
    //             let left = self.eval(*left, ctx).unpack();
    //             let right = self.eval(*right, ctx).unpack();
    //             match op {
    //                 ArithmeticOp::Add => left + right,
    //                 ArithmeticOp::Subtract => left - right,
    //                 ArithmeticOp::Divide => left / right,
    //                 ArithmeticOp::Multiply => left * right,
    //                 ArithmeticOp::Modulus => todo!("modulus"),
    //             }
    //         }
    //         Expression::Ternary(_, _, _) => todo!(),
    //         Expression::Or(_, _) => todo!(),
    //         Expression::And(_, _) => todo!(),
    //         Expression::Unary(_, _) => todo!(),
    //         Expression::Member(expr, member) => self.eval_member(*expr, *member, ctx).unpack(),
    //         Expression::List(elems) => self.eval_list(elems, ctx),
    //         Expression::Map(entries) => self.eval_map(entries, ctx),
    //         Expression::Ident(r) => {
    //             let val  = ctx.resolve(&r);
    //             if let Some(val) = val {
    //                 return val
    //             }
    //             panic!("unknown attribute {}", &r)
    //         },
    //         Expression::FunctionCall(_) => todo!(),
    //     }
    // }
}
