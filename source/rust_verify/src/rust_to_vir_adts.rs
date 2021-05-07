use crate::rust_to_vir_base::{
    check_generics, def_id_to_vir_path, get_mode, hack_get_def_name, ty_to_vir,
};
use crate::util::spanned_new;
use crate::{unsupported, unsupported_unless};
use rustc_hir::{Crate, EnumDef, Generics, ItemId, VariantData};
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;
use std::rc::Rc;
use vir::ast::{DatatypeX, Ident, KrateX, Mode, Variant, VirErr};
use vir::ast_util::{ident_binder, str_ident};
use vir::def::variant_ident;

fn check_variant_data<'tcx>(
    tcx: TyCtxt<'tcx>,
    _krate: &'tcx Crate<'tcx>,
    name: &Ident,
    variant_data: &'tcx VariantData<'tcx>,
) -> Variant {
    ident_binder(
        name,
        &(match variant_data {
            VariantData::Struct(fields, recovered) => {
                unsupported_unless!(!recovered, "recovered_struct", variant_data);
                Rc::new(
                    fields
                        .iter()
                        .map(|field| {
                            ident_binder(
                                &str_ident(&field.ident.as_str()),
                                &(
                                    ty_to_vir(tcx, field.ty),
                                    get_mode(Mode::Exec, tcx.hir().attrs(field.hir_id)),
                                ),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            }
            VariantData::Tuple(fields, _variant_id) => Rc::new(
                fields
                    .iter()
                    .map(|field| {
                        ident_binder(
                            &str_ident(&field.ident.as_str()),
                            &(
                                ty_to_vir(tcx, field.ty),
                                get_mode(Mode::Exec, tcx.hir().attrs(field.hir_id)),
                            ),
                        )
                    })
                    .collect::<Vec<_>>(),
            ),
            VariantData::Unit(_) => {
                unsupported!("unit_adt", variant_data);
            }
        }),
    )
}

pub fn check_item_struct<'tcx>(
    tcx: TyCtxt<'tcx>,
    krate: &'tcx Crate<'tcx>,
    vir: &mut KrateX,
    span: Span,
    id: &ItemId,
    variant_data: &'tcx VariantData<'tcx>,
    generics: &'tcx Generics<'tcx>,
) -> Result<(), VirErr> {
    check_generics(generics)?;
    let name = hack_get_def_name(tcx, id.def_id.to_def_id());
    let path = def_id_to_vir_path(tcx, id.def_id.to_def_id());
    let variant_name = variant_ident(&name, &name);
    let variants = Rc::new(vec![check_variant_data(tcx, krate, &variant_name, variant_data)]);
    vir.datatypes.push(spanned_new(span, DatatypeX { path, variants }));
    Ok(())
}

pub fn check_item_enum<'tcx>(
    tcx: TyCtxt<'tcx>,
    krate: &'tcx Crate<'tcx>,
    vir: &mut KrateX,
    span: Span,
    id: &ItemId,
    enum_def: &'tcx EnumDef<'tcx>,
    generics: &'tcx Generics<'tcx>,
) -> Result<(), VirErr> {
    check_generics(generics)?;
    let name = Rc::new(hack_get_def_name(tcx, id.def_id.to_def_id()));
    let path = def_id_to_vir_path(tcx, id.def_id.to_def_id());
    let variants = Rc::new(
        enum_def
            .variants
            .iter()
            .map(|variant| {
                let rust_variant_name = variant.ident.as_str();
                let variant_name = str_ident(
                    format!("{}{}{}", name, vir::def::VARIANT_SEPARATOR, rust_variant_name)
                        .as_str(),
                );
                check_variant_data(tcx, krate, &variant_name, &variant.data)
            })
            .collect::<Vec<_>>(),
    );
    vir.datatypes.push(spanned_new(span, DatatypeX { path, variants }));
    Ok(())
}