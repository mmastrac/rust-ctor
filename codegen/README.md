# ctor codegen

This project does the codegen to ensure that ctor builds without dependencies. We use
the `quote!` macro to generate a `TokenStream`, and then dump that into a source file.
