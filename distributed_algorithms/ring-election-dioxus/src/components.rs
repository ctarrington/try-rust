use crate::model::{Network, NetworkConnection};
use dioxus::prelude::*;

const PROCESS_COUNT: usize = 9;

#[derive(Props, PartialEq)]
pub struct NetworkRowProps {
    row_index: usize,
    row: [NetworkConnection; PROCESS_COUNT],
    processor_count: usize,
}

#[allow(non_snake_case)]
pub fn NetworkRow(cx: Scope<NetworkRowProps>) -> Element {
    let NetworkRowProps {
        row_index,
        row,
        processor_count,
    } = cx.props;
    let processor_count = *processor_count;

    let row_cells = (0..processor_count).map(|index| {
        let cell_contents = format!(" {:?} ", row[index]);
        rsx!(
        td {
            key: "{index}",
            "{cell_contents}"
        }
        )
    });

    cx.render(rsx!(
        tr {
            td { "{row_index}" }
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
    let NetworkProps {
        network,
        processor_count,
    } = cx.props;
    let processor_count = *processor_count;

    let header_cells = (0..processor_count).map(|index| {
        rsx!(
        td {
            key: "{index}",
            "{index}"
        }
        )
    });

    let header_row = rsx!(
        tr {
            td {"from"}
            header_cells
        }
    );

    let network_rows = (0..processor_count).map(|from| {
        rsx!(NetworkRow {
            row_index: from.clone(),
            row: network.get_connections_from(from),
            processor_count: processor_count,
        })
    });

    cx.render(rsx!(
        table {
            header_row
            network_rows
        }
    ))
}
