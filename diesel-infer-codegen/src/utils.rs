use syn::{Ty, PathSegment, MutTy};
use quote::Tokens;

pub fn get_fields_vec(fields: &[String]) -> Tokens {
    let mut inner = String::new();
    inner.push_str("vec![");
    for (index, field) in fields.iter().enumerate()  {
        if index != 0 {
            inner.push_str(", ");
        }
        inner.push_str(&format!("\"{}\"", field));
        inner.push_str(".to_string()");
    }
    inner.push_str("]");
    let mut tokens = Tokens::new();
    tokens.append(inner);
    tokens
}

pub fn get_values(fields: &[String], tys: &[Ty]) -> Tokens {
    let mut inner = String::new();
    inner.push_str("vec![");
    for (index, field) in fields.iter().enumerate() {
        if index != 0 {
            inner.push_str(", ");
        }
        inner.push_str("::diesel_infer::get_");
        let ty = get_type(&tys[index]);
        inner.push_str(&ty);
        inner.push_str("_sql(");
        if ty == "str" || ty == "vec" {
            inner.push_str("&");
        }
        inner.push_str("self.");
        inner.push_str(field);
        inner.push_str(")");
    }
    inner.push_str("]");
    let mut tokens = Tokens::new();
    tokens.append(inner);
    tokens
}

fn get_type(ty: &Ty) -> String {
    match ty {
        &Ty::Path(_, ref path) => {
            parse_path_segment(&path.segments)
        }
        &Ty::Rptr(_, ref mut_ty) => {
            parse_mut_ty(mut_ty)
        },
        _ => panic!("unsupported ty type!")
    }
}

fn parse_path_segment(segments: &[PathSegment]) -> String {
    let type_ = match segments[0].ident.as_ref() {
        "i32" => "i32",
        "i64" => "i64",
        "bool" => "bool",
        "Vec" => "vec",
        "String" => "str",
        "str" => "str",
        "u8" => "vec",
        _ => panic!("unsupported path type!")
    };
    type_.into()
}

fn parse_mut_ty(mut_ty: &MutTy) -> String {
    match mut_ty.ty {
        Ty::Path(_, ref path) => {
            parse_path_segment(&path.segments)
        }
        Ty::Slice(ref ty) => {
            get_type(ty.as_ref())
        }
        ref ty_ => panic!("unsupported mutty type! ty_= {:?} ", ty_)
    }
}
