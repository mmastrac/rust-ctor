use clitest_lib::clitest;

clitest!(basic, r#"
set RUSTFLAGS "";
cd "link_section/basic";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
$ cargo run --quiet
! LINK_SECTION: Section { name: "__TEXT,LINK_SECTION", start: %{BASE16NUM}, end: %{BASE16NUM}, byte_len: 40 }
! link_section_function
! TYPED_LINK_SECTION: TypedSection { name: "__DATA,TYPED_LINKIXOI_M", start: 0x100c74268, end: %{BASE16NUM}, len: 2, stride: 4 }
! CODE_SECTION: TypedSection { name: "__DATA,FN_ARRAY", start: %{BASE16NUM}, end: %{BASE16NUM}, len: 3, stride: 8 }
! [%{BASE16NUM}, %{BASE16NUM}, %{BASE16NUM}]
! f: %{BASE16NUM}
! f: %{BASE16NUM}
! linked_function
! f: %{BASE16NUM}
! linked_function_2
! DEBUGGABLES: [1, 2, %{BASE16NUM}]
"#);
