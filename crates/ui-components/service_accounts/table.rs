#![allow(non_snake_case)]
use db::ServiceAccount;
use dioxus::prelude::*;
use primer_rsx::*;

#[derive(Props, PartialEq)]
pub struct TableProps {
    service_accounts: Vec<ServiceAccount>,
}

pub fn ServiceAccountTable(cx: Scope<TableProps>) -> Element {
    cx.render(rsx!(
        Box {
            BoxHeader {
                title: "Service Accounts"
            }
            BoxBody {
                DataTable {
                    table {
                        thead {
                            th { "Service Account Name" }
                            th { "Vault" }
                            th { "Environment" }
                            th { "Updated" }
                            th { "Created" }
                            th { "Action" }
                        }
                        tbody {
                            cx.props.service_accounts.iter().map(|service_account| rsx!(
                                tr {
                                    if let Some(vault_name) = &service_account.vault_name {
                                        cx.render(rsx!(
                                            td {
                                                id: "service-account-view-{service_account.id}",
                                                "{service_account.account_name}"
                                            }
                                            td {
                                                "{vault_name}"
                                            }
                                        ))
                                    } else {
                                        cx.render(rsx!(
                                            td {
                                                "{service_account.account_name}"
                                            }
                                            td {
                                                id: "service-account-row-{service_account.id}",
                                                a {
                                                    "Connect to Vault"
                                                }
                                            }
                                        ))
                                    }
                                    if let Some(env_name) = &service_account.environment_name {
                                        cx.render(rsx!(
                                            td {
                                                Label {
                                                    "{env_name}"
                                                }
                                            }
                                        ))
                                    } else {
                                        cx.render(rsx!(
                                            td {
                                            }
                                        ))
                                    }
                                    td {
                                    }
                                    td {
                                    }
                                    td {
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
