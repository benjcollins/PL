use std::collections::HashMap;

use crate::{infer::{InferTyRef, Unify, unify}, symbols::Symbol};

pub type TyRef = InferTyRef<Ty>;
pub type IntTyRef = InferTyRef<IntTy>;
pub type StructTyRef = InferTyRef<StructTy>;

#[derive(Debug, Clone)]
pub enum Ty {
    Bool,
    Ref(TyRef),
    Int(IntTyRef),
    Struct(StructTyRef),
    Any,
}

#[derive(Debug, Clone)]
pub enum IntTy {
    Int(Int),
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Int {
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

#[derive(Debug, Clone)]
pub enum StructTy {
    Known(KnownStruct<TyRef>),
    WithFields(HashMap<Symbol, TyRef>),
}

#[derive(Debug, Clone)]
pub struct Field<T> {
    pub name: Symbol,
    pub ty: T,
}

#[derive(Debug, Clone)]
pub struct KnownStruct<T> {
    pub name: Symbol,
    pub fields: Vec<Field<T>>,
}

pub enum ConcreteTy {
    Bool,
    Ref(Box<ConcreteTy>),
    Int(Int),
    Struct(KnownStruct<ConcreteTy>),
}

impl Unify for Ty {
    type Concrete = ConcreteTy;

    fn unify(a: Ty, b: Ty) -> Result<Ty, ()> {
        Ok(match (a, b) {
            (Ty::Any, Ty::Any) => Ty::Any,
            (Ty::Any, ty) | (ty,  Ty::Any) => ty,
            (Ty::Bool, Ty::Bool) => Ty::Bool,
            (Ty::Ref(a), Ty::Ref(b)) => Ty::Ref(unify(&a, &b)?),
            (Ty::Int(a), Ty::Int(b)) => Ty::Int(unify(&a, &b)?),
            (Ty::Struct(a), Ty::Struct(b)) => Ty::Struct(unify(&a, &b)?),
            _ => Err(())?,
        })
    }
    fn concrete(&self) -> Self::Concrete {
        match self {
            Ty::Bool => ConcreteTy::Bool,
            Ty::Ref(ty) => ConcreteTy::Ref(Box::new(ty.concrete())),
            Ty::Int(ty) => ConcreteTy::Int(ty.concrete()),
            Ty::Struct(ty) => ConcreteTy::Struct(ty.concrete()),
            Ty::Any => todo!(),
        }
    }
}

impl Unify for IntTy {
    type Concrete = Int;

    fn unify(a: Self, b: Self) -> Result<Self, ()> {
        Ok(match (a, b) {
            (IntTy::Any, IntTy::Any) => IntTy::Any,
            (ty, IntTy::Any) | (IntTy::Any, ty) => ty,
            (IntTy::Int(a), IntTy::Int(b)) => if a == b {
                IntTy::Int(a)
            } else {
                Err(())?
            } 
        })
    }
    fn concrete(&self) -> Self::Concrete {
        match self {
            IntTy::Int(int) => *int,
            IntTy::Any => Int { signedness: Signedness::Signed, size: Size::B32 },
        }
    }
}

impl Unify for StructTy {
    type Concrete = KnownStruct<ConcreteTy>;
    
    fn unify(a: Self, b: Self) -> Result<Self, ()> {
        Ok(match (a, b) {
            (StructTy::Known(a), StructTy::Known(b)) => if a.name == b.name {
                StructTy::Known(a)
            } else {
                Err(())?
            }
            (StructTy::Known(s), StructTy::WithFields(required_fields)) |
            (StructTy::WithFields(required_fields), StructTy::Known(s)) => {
                for (name, ty) in required_fields {
                    let field = s.fields.iter().find(|field| field.name == name).unwrap();
                    unify(&field.ty, &ty)?;
                }
                StructTy::Known(s)
            }
            (StructTy::WithFields(mut a), StructTy::WithFields(b)) => {
                for (name, b_ty) in b {
                    match a.get(&name) {
                        Some(a_ty) => _ = unify(a_ty, &b_ty)?,
                        None => _ = a.insert(name, b_ty),
                    }
                }
                StructTy::WithFields(a)
            }
        })
    }
    fn concrete(&self) -> Self::Concrete {
        match self {
            StructTy::Known(s) => KnownStruct {
                name: s.name,
                fields: s.fields.iter().map(|field| Field { name: field.name, ty: field.ty.concrete() }).collect(),
            },
            StructTy::WithFields(_) => panic!(),
        }
    }
}

// impl fmt::Display for IntTy {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let signedness = match self.signedness {
//             Signedness::Signed => "i",
//             Signedness::Unsigned => "u",
//         };
//         let size = match self.size {
//             Size::B8 => "8",
//             Size::B16 => "16",
//             Size::B32 => "32",
//         };
//         write!(f, "{}{}", signedness, size)
//     }
// }

// impl fmt::Display for Ty {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Ty::Bool => write!(f, "bool"),
//             Ty::Ref(ty) => write!(f, "&{}", ty.ty())),
//             Ty::Int(int_ty) => write!(f, "{}", int_ty.concrete())),
//             Ty::Struct { .. } => todo!(),
//         }
//     }
// }