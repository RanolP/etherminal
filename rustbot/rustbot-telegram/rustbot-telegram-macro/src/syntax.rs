struct TelegramApiFunction {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    init: Expr,
}

impl Parse for TelegramApiFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![static]>()?;
        input.parse::<Token![ref]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let init: Expr = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(TelegramApiFunction {
            visibility,
            name,
            ty,
            init,
        })
    }
}
