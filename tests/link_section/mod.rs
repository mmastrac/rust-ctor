use clitest_lib::clitest;

clitest!(
    basic,
    r#"
set RUSTFLAGS "";
cd "link_section/basic";
defer {
    $ cargo clean --quiet
}
$ cargo build --quiet
*
$ cargo run --quiet
! LINK_SECTION: Section { name: "%{DATA}LINK_SECTION%{DATA}", start: %{BASE16NUM}, end: %{BASE16NUM}, byte_len: 40 }
! TYPED_LINK_SECTION: TypedSection { name: "%{DATA}TYPED_LINK%{DATA}", start: %{BASE16NUM}, end: %{BASE16NUM}, len: 2, stride: 4 }
! address of TYPED_LINK_SECTION[0]: %{BASE16NUM}
! address of TYPED_LINK_SECTION[1]: %{BASE16NUM}
! CODE_SECTION: TypedSection { name: "%{DATA}FN_ARRAY%{DATA}", start: %{BASE16NUM}, end: %{BASE16NUM}, len: 3, stride: 8 }
! [%{BASE16NUM}, %{BASE16NUM}, %{BASE16NUM}]
! f: %{BASE16NUM}
! link_section_function
! f: %{BASE16NUM}
! linked_function
! f: %{BASE16NUM}
! linked_function_2
! DEBUGGABLES: [1, 2, %{BASE16NUM}]
"#
);
