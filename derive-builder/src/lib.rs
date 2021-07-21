use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let fields: Vec<(String, syn::Type)> = if let syn::Data::Struct(data_struct) = ast.data {
        if let syn::Fields::Named(fields_named) = data_struct.fields {
            fields_named
                .named
                .into_iter()
                .map(|field| (field.ident.unwrap().to_string(), field.ty))
                .collect()
        } else {
            panic!("Unnamed fields are not supported")
        }
    } else {
        panic!("#[derive(Builder)] supports only structs")
    };

    let base_name = ast.ident.to_string();
    let base_ident = ident(&base_name);

    let builder_name = base_name + "Builder";
    let builder_ident = ident(&builder_name);

    let builder_method_defs = fields
        .iter()
        .fold(quote! {}, |mut acc, (field_name, field_type)| {
            let ident = ident(field_name);
            acc.extend(quote! {
                fn #ident(mut self, #ident: #field_type) -> #builder_ident {
                    self.draft.#ident = #ident;
                    self
                }
            });
            acc
        });

    proc_macro::TokenStream::from(quote! {
        impl #base_ident {
            fn builder() -> #builder_ident {
                #builder_ident { draft: Default::default() }
            }
        }

        struct #builder_ident {
            draft: #base_ident,
        }

        impl #builder_ident {
            #builder_method_defs

            pub fn build(self) -> #base_ident {
                self.draft
            }
        }
    })
}

fn ident<T: ToString>(content: T) -> syn::Ident {
    syn::Ident::new(&content.to_string(), proc_macro2::Span::call_site())
}
