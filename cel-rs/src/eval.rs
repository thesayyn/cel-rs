use core::panic;
use std::rc::Rc;
use ordered_hash_map::OrderedHashMap;
use crate::parser::{ArithmeticOp, Expression, Member, RelationOp};
use crate::{value::Value, Context};

pub trait Bag {
   fn unpack(self) -> Value;
}

impl Bag for Value {
    fn unpack(self) -> Value {
        self
    }  
}

#[derive(Default)]
pub struct Eval {}

impl Eval {

    fn eval_member(&self, expr: Expression, member: Member, ctx: &mut Context) -> impl Bag {
        let v = self.eval(expr, ctx).unpack();
        match member {
            crate::parser::Member::Attribute(attr) => {
                if let Value::Map(v) = v {
                    let value = v.get(&Value::String(attr)).expect("TODO: unknown map key");
                    return value.to_owned()
                }
                if let Some(val) = ctx.resolve(&attr) {
                    if let Value::Function(f, _) = val {
                        return Value::Function(f, Some(Rc::new(v)));
                    } 
                    return  val
                }
                
                panic!("unknown attribute {}", attr)
            },
            crate::parser::Member::FunctionCall(mut rargs) => {
                let mut args = Vec::with_capacity(rargs.len());
                rargs.reverse();
                for arg in rargs {
                    args.push(self.eval(arg, ctx).unpack());
                }
                
                if let Value::Function(f, bound) = v {
                    if let Some(b) = bound {
                        args.push((*b).clone());
                        args.reverse()
                    }
                    return (f.overloads.first().unwrap().func)(args)
                }

                panic!("is not a func")
            },
            crate::parser::Member::Index(i) => {
                let i = self.eval(*i, ctx).unpack();
                if let Value::Map(v) = v {
                    let value = v.get(&i).expect("TODO: unknown map key");
                    return value.to_owned()
                } 
                Value::Null
            },
            crate::parser::Member::Fields(_) => todo!("Fields"),
        }
    }
    fn eval_map(&self, entries: Vec<(Expression, Expression)>, ctx: &mut Context) -> Value {
        let mut map = OrderedHashMap::with_capacity(entries.len());
        for (kexpr, vexpr) in entries {
            let k = self.eval(kexpr, ctx).unpack();
            let v = self.eval(vexpr, ctx).unpack();
            map.insert(k, v);
        }
        Value::Map(Rc::new(map))
    }

    fn eval_list(&self, elems: Vec<Expression>, ctx: &mut Context) -> Value {
        let mut list = Vec::with_capacity(elems.len());
        for expr in elems {
            let v = self.eval(expr, ctx).unpack();
            list.push(v);
        }
        Value::List(Rc::new(list))
    }

    pub fn eval(&self, expr: Expression, ctx: &mut Context) -> impl Bag {
        match expr {
            Expression::Atom(atom) => Value::from(atom),
            Expression::Relation(left, op, right) => {
                let left = self.eval(*left, ctx).unpack();
                let right = self.eval(*right, ctx).unpack();
                let result = match op {
                    RelationOp::Equals => left.eq(&right),
                    RelationOp::LessThan => todo!("lt"),
                    RelationOp::LessThanEq => todo!("lte"),
                    RelationOp::GreaterThan => todo!("gt"),
                    RelationOp::GreaterThanEq => todo!("gte"),
                    RelationOp::NotEquals => todo!("ne"),
                    RelationOp::In => todo!("in"),
                };
                Value::Bool(result)
            }
            Expression::Arithmetic(left, op, right) => {
                let left = self.eval(*left, ctx).unpack();
                let right = self.eval(*right, ctx).unpack();
                match op {
                    ArithmeticOp::Add => left + right,
                    ArithmeticOp::Subtract => left - right,
                    ArithmeticOp::Divide => left / right,
                    ArithmeticOp::Multiply => left * right,
                    ArithmeticOp::Modulus => todo!("modulus"),
                }
            }
            Expression::Ternary(_, _, _) => todo!(),
            Expression::Or(_, _) => todo!(),
            Expression::And(_, _) => todo!(),
            Expression::Unary(_, _) => todo!(),
            Expression::Member(expr, member) => self.eval_member(*expr, *member, ctx).unpack(),
            Expression::List(elems) => self.eval_list(elems, ctx),
            Expression::Map(entries) => self.eval_map(entries, ctx),
            Expression::Ident(r) => {
                let val  = ctx.resolve(&r);
                if let Some(val) = val {
                    return val
                }
                panic!("unknown attribute {}", &r)
            },
        }
    }
}
