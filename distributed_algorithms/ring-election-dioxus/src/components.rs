use dioxus::prelude::*;
use crate::model::{Network, NetworkConnection};

const PROCESS_COUNT: usize = 9;

#[derive(Props, PartialEq)]
pub struct NetworkRowProps {
    row: [NetworkConnection; PROCESS_COUNT],
    processor_count: usize,
}

#[allow(non_snake_case)]
pub fn NetworkRow(cx: Scope<NetworkRowProps>) -> Element {
    let NetworkRowProps {row, processor_count} = cx.props;
    let processor_count = *processor_count;

    let row_cells = (0..processor_count).map(|index| {
        let cell_contents = format!(" {:?} ", row[index]);
        rsx!(
            span {
                key: "{index}",
                "{cell_contents}"
            }
            )
    });

    cx.render( rsx!(
        div {
            row_cells
        }
    ))
}


#[derive(Props, PartialEq)]
pub struct NetworkProps {
    network: Network,
    processor_count: usize,
}

#[allow(non_snake_case)]
pub fn NetworkComponent(cx: Scope<NetworkProps>) -> Element {
    let NetworkProps {network, processor_count} = cx.props;
    let processor_count = *processor_count;

    let network_rows = (0..processor_count).map(|from| {
        rsx!(
            NetworkRow {
                row: network.get_connections_from(from),
                processor_count: processor_count,
            })
    });

    cx.render( rsx!(
        div {
            network_rows
        }
    ))
}
