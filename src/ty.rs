use std::fmt;

use crate::{infer::{InferTyRef, Unify, unify}, symbols::Symbol};

pub type TyRef = InferTyRef<Ty>;
pub type IntTyRef = InferTyRef<IntTy>;

#[derive(Debug, Clone)]
pub enum Ty {
    Bool,
    Ref(TyRef),
    Int(IntTyRef),
    Struct {
        name: Symbol,
        tys: Vec<TyRef>,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntTy {
    pub signedness: Signedness,
    pub size: Size,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signedness {
    Signed, Unsigned
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    B8, B16, B32
}

impl<'a> Unify for Ty {
    fn unify(a: &Ty, b: &Ty) -> Result<(), ()> {
        match (a, b) {
            (Ty::Bool, Ty::Bool) => (),
            (Ty::Ref(a), Ty::Ref(b)) => _ = unify(&a, &b)?,
            (Ty::Int(a), Ty::Int(b)) => _ = unify(&a, &b)?,
            (Ty::Struct { name: a, .. }, Ty::Struct { name: b, .. }) if a == b => (),
            _ => Err(())?,
        };
        Ok(())
    }
}

impl Unify for IntTy {
    fn unify(_: &IntTy, _: &IntTy) -> Result<(), ()> {
        Ok(())
    }
}

impl fmt::Display for IntTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signedness = match self.signedness {
            Signedness::Signed => "i",
            Signedness::Unsigned => "u",
        };
        let size = match self.size {
            Size::B8 => "8",
            Size::B16 => "16",
            Size::B32 => "32",
        };
        write!(f, "{}{}", signedness, size)
    }
}

pub struct TyOption(pub Option<Ty>);

impl fmt::Display for TyOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(ty) => write!(f, "{}", ty),
            None => write!(f, "any?"),
        }
    }
}

pub struct IntTyOption(pub Option<IntTy>);

impl fmt::Display for IntTyOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(ty) => write!(f, "{}", ty),
            None => write!(f, "int?"),
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Bool => write!(f, "bool"),
            Ty::Ref(ty) => write!(f, "&{}", TyOption(ty.concrete())),
            Ty::Int(int_ty) => write!(f, "{}", IntTyOption(int_ty.concrete())),
            Ty::Struct { .. } => todo!(),
        }
    }
}