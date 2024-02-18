use std::rc::Rc;
use ordered_hash_map::OrderedHashMap;
use parser::{ArithmeticOp, Expression, Member, RelationOp};
use crate::{value::Value, Context};

pub trait Bag {
   fn unpack(self) -> Value;
}

/// Indexer permits random access of elements by index ```a[b()]```.
pub trait Indexer<T> where T: Bag {
    fn get(val: T) -> T;
}

impl Bag for Value {
    fn unpack(self) -> Value {
        self
    }  
}

pub struct Eval {
    ctx: Context,
}

impl Eval {
    pub fn new(ctx: Context) -> Self {
        Eval { ctx }
    }

    fn eval_member(&self, expr: Expression, member: Member) -> impl Bag {
        let v = self.eval(expr).unpack();
        match member {
            parser::Member::Attribute(_) => todo!("Attribute"),
            parser::Member::FunctionCall(_) => todo!("FunctionCall"),
            parser::Member::Index(i) => {
                let i = self.eval(*i).unpack();
                if let Value::Map(v) = v {
                    let value = v.get(&i).expect("TODO: unknown map key");
                    return value.to_owned()
                } 
                Value::Null
                
            },
            parser::Member::Fields(_) => todo!("Fields"),
        }
    }
    fn eval_map(&self, entries: Vec<(Expression, Expression)>) -> Value {
        let mut map = OrderedHashMap::with_capacity(entries.len());
        for (kexpr, vexpr) in entries {
            let k = self.eval(kexpr).unpack();
            let v = self.eval(vexpr).unpack();
            println!("k: {}, v: {}", &k, &v);
            map.insert(k, v);
        }
        Value::Map(Rc::new(map))
    }

    pub fn eval(&self, expr: Expression) -> impl Bag {
        match expr {
            Expression::Atom(atom) => Value::from(atom),
            Expression::Relation(left, op, right) => {
                let left = self.eval(*left).unpack();
                let right = self.eval(*right).unpack();
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
                let left = self.eval(*left).unpack();
                let right = self.eval(*right).unpack();
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
            Expression::Member(expr, member) => self.eval_member(*expr, *member).unpack(),
            Expression::List(_) => todo!(),
            Expression::Map(entries) => self.eval_map(entries),
            Expression::Ident(r) => Value::String(r),
        }
    }
}
