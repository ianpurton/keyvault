#![allow(non_snake_case)]
use db::Audit;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct TableProps<'a> {
    audits: &'a Vec<Audit>,
}

pub fn AuditTable<'a>(cx: Scope<'a, TableProps<'a>>) -> Element {
    cx.render(rsx!(
        Box {
            BoxHeader {
                title: "Audit Trail"
            }
            BoxBody {
                DataTable {
                    table {
                        thead {
                            th { "When" }
                            th { "User" }
                            th { "Access Type" }
                            th { "Action" }
                            th { "Description" }
                        }
                        tbody {
                            cx.props.audits.iter().map(|audit| rsx!(
                                tr {
                                    td {
                                    }
                                    td {
                                        "{audit.email}"
                                    }
                                    td {
                                    }
                                    td {
                                    }
                                    td {
                                        "{audit.description}"
                                    }
                                }
                            ))
                        }
                    }
                }
            }
        }
    ))
}
