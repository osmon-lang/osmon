use crate::Context as CContext;

use crate::{
    str,
    syntax::ast::{
        Elem, Expr, ExprKind, Function, NodeId, Stmt, StmtKind, StructArg, StructField, Type,
    },
};

use crate::syntax::interner::Name;
use std::{
    collections::{HashMap, VecDeque},
    ffi::CString,
};
