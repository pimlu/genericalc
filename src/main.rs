use std::collections::HashMap;
use std::fmt;

#[derive(Clone,PartialEq,Eq,Hash,Debug)]
struct Symbol {
    name: String,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
     }
}

struct Inputs {
    symbol_map: HashMap<Symbol, f64>,
}

trait Eval: Clone {
    fn eval(&self, inputs: &Inputs) -> f64;
}

trait Derive: Eval {
    type DerivT: Eval;
    fn deriv(&self, vs: &Symbol) -> Self::DerivT;
}

#[derive(Debug, Clone)]
struct Const {
    val: f64,
}

impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
     }
}

impl Eval for Const {
    fn eval(&self, _inputs: &Inputs) -> f64 {
        return self.val;
    }
}
impl Derive for Const {
    type DerivT = Const;
    fn deriv(&self, _vs: &Symbol) -> Self::DerivT {
        Const {
            val: 0.0
        }
    }
}

impl Eval for Symbol {
    fn eval(&self, inputs: &Inputs) -> f64 {
        if let Some(res) = inputs.symbol_map.get(self) {
            *res
        } else {
            panic!("Couldn't find symbol {:?}", self)
        }
    }
}

impl Derive for Symbol {
    type DerivT = Const;
    fn deriv(&self, vs: &Symbol) -> Self::DerivT {
        Const {
            val: if vs == self { 1.0 } else { 0.0 }
        }
    }
}

#[derive(Debug, Clone)]
struct DAdd<L, R> {
    lhs: L,
    rhs: R,
}

impl<L, R> fmt::Display for DAdd<L, R> where L: fmt::Display, R: fmt::Display{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} + {})", self.lhs, self.rhs)
     }
}

impl<L, R> Eval for DAdd<L, R> where L: Eval, R: Eval {
    fn eval(&self, inputs: &Inputs) -> f64 {
        self.lhs.eval(inputs) + self.rhs.eval(inputs)
    }
}

impl<L, R> Derive for DAdd<L, R> where L: Derive, R: Derive {
    type DerivT = DAdd<L::DerivT, R::DerivT>;

    fn deriv(&self, vs: &Symbol) -> Self::DerivT {
        DAdd {
            lhs: self.lhs.deriv(vs),
            rhs: self.rhs.deriv(vs)
        }
    }
}

#[derive(Debug, Clone)]
struct DMul<L, R> {
    lhs: L,
    rhs: R
}

impl<L, R> fmt::Display for DMul<L, R> where L: fmt::Display, R: fmt::Display{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} * {})", self.lhs, self.rhs)
     }
}

impl<L, R> Eval for DMul<L, R> where L: Eval, R: Eval {
    fn eval(&self, inputs: &Inputs) -> f64 {
        self.lhs.eval(inputs) * self.rhs.eval(inputs)
    }
}
impl<L, R> Derive for DMul<L, R> where L: Derive, R: Derive {
    type DerivT = DAdd<DMul<L, R::DerivT>, DMul<L::DerivT, R>>;

    fn deriv(&self, vs: &Symbol) -> Self::DerivT {
        let d_l = self.lhs.deriv(vs);
        let d_r = self.rhs.deriv(vs);
        DAdd {
            lhs: DMul {
                lhs: self.lhs.clone(),
                rhs: d_r
            },
            rhs: DMul {
                lhs: d_l,
                rhs: self.rhs.clone()
            }
        }
    }
}


fn main() {
    let x = Symbol { name: "x".to_string() };

    let xp1 = DAdd {
        lhs: x.clone(),
        rhs: Const { val: 1.0 }
    };
    let func = DMul {
        lhs: xp1.clone(),
        rhs: xp1
    };

    let ddx = func.deriv(&x);

    let inputs = Inputs {
        symbol_map: HashMap::from([(x, 2.0)]),
    };


    println!("f(x): {}", func);
    println!("f(2): {}", func.eval(&inputs));

    println!("f'(x): {}", ddx);
    println!("f'(2): {}", ddx.eval(&inputs));
}
