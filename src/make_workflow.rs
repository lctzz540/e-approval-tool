use crate::{utils, workflow};
use std::error::Error;

pub fn make_workflow() -> Result<(), Box<dyn Error>> {
    let request: [i32; 4] = [4, 2, 1, 3];
    let mut workflowout = workflow::Root::default();

    workflowout.notification_config =
        match workflow::read_json_file_to_notification_config("notification_config.json") {
            Ok(noti_config) => noti_config,
            Err(err) => {
                eprintln!("Error reading JSON file: {}", err);
                std::process::exit(1);
            }
        };

    let mut root_of_relatives = workflow::Rela::default();
    root_of_relatives.roles = Some(workflow::Roles::default());
    root_of_relatives.id = -3;
    root_of_relatives.type_field = "ticket".to_string();
    root_of_relatives.is_done = false;
    root_of_relatives.status = "Opened".to_string();
    workflowout.relatives.push(root_of_relatives);

    let mut autorow = 0;
    let mut connect_from = -3;
    for row in 0..request.len() {
        for node in 0..request[row] {
            let nodeid = (row as i64 + autorow as i64 + 1) * 10 + node as i64 + 1;
            workflowout.relatives.push(add_node(
                -nodeid,
                &format!("node at {}.{}", row + 1, node + 1),
                if row == 0 { "Responding" } else { "Waiting" },
            ));
            workflowout
                .relationships
                .push(add_edge(connect_from, -nodeid, false));
            if row + 1 == request.len() {
                workflowout.relationships.push(add_edge(-nodeid, -1, true));
                continue;
            }
            if request[row + 1] > 1 {
                workflowout.relationships.push(add_edge(
                    -nodeid,
                    -(row as i64 + 1 + autorow + 1) * 10,
                    false,
                ))
            } else {
                let single_node = (row as i64 + autorow as i64 + 2) * 10 + 1;
                workflowout
                    .relationships
                    .push(add_edge(-nodeid, single_node, false));
                if connect_from != single_node {
                    connect_from = (row as i64 + autorow as i64 + 2) * 10 + 1;
                }
            }
        }
        if request[row] > 1 && row != request.len() - 1 {
            autorow += 1;
            let autoid = (row as i64 + 1 + autorow) * 10;
            workflowout.relatives.push(add_auto_move(-autoid));
            connect_from = -autoid;
        }
    }

    let mut end_of_relatives = workflow::Rela::default();
    end_of_relatives.summary = Some("User Confirm Result".to_string());
    end_of_relatives.roles = Some(workflow::Roles::default());
    end_of_relatives.id = -1;
    end_of_relatives.type_field = "phase".to_string();
    end_of_relatives.is_done = false;
    end_of_relatives.status = "Waiting".to_string();
    end_of_relatives.phase_type = Some("Close".to_string());
    workflowout.relatives.push(end_of_relatives);

    match utils::workflow_to_json(&workflowout) {
        Ok(json_string) => {
            if let Err(err) = utils::write_json_to_file("workflowout.json", &json_string) {
                eprintln!("Error exporting JSON to file: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Error converting to JSON: {}", err);
        }
    }

    Ok(())
}

fn add_auto_move(id: i64) -> workflow::Rela {
    let mut auto_relalive = workflow::Rela::default();
    auto_relalive.id = id;
    auto_relalive.owner = Some("system".to_string());
    auto_relalive.summary = Some("Auto move to next step".to_string());
    auto_relalive.is_notify = Some(true);
    auto_relalive.auto_finish = Some(true);
    auto_relalive.sla = Some(workflow::Sla {
        res: 0.01,
        fix: 0.02,
    });
    auto_relalive.prevent_feeder = Some(true);
    auto_relalive.type_field = "phase".to_string();
    auto_relalive.is_done = false;
    auto_relalive.phase_type = Some("Implement".to_string());
    auto_relalive.autocreate = Some(false);
    auto_relalive.role_config = Some(workflow::RoleConfig::default());
    auto_relalive.status = "Responding".to_string();

    auto_relalive
}

fn add_node(id: i64, summary: &str, status: &str) -> workflow::Rela {
    let mut node = workflow::Rela::default();
    node.id = id;
    node.is_done = false;
    node.status = status.to_string();
    node.type_field = "phase".to_string();
    node.summary = Some(summary.to_string());
    node.sla = Some(workflow::Sla {
        res: 2 as f64,
        fix: 16 as f64,
    });
    node.autocreate = Some(false);
    node.phase_type = Some("Implement".to_string());

    node
}

fn add_edge(fromid: i64, toid: i64, is_last_edge: bool) -> workflow::Relationship {
    let id_str = format!("-{}{}", fromid.abs(), toid.abs());
    let id = match id_str.parse::<i64>() {
        Ok(parsed_value) => parsed_value,
        Err(_) => {
            println!("Failed to parse as i64");
            0
        }
    };
    let mut rela_out = workflow::Relationship::default();

    rela_out.autocreate = false;
    rela_out.display = Some(false);
    rela_out.from = fromid;
    rela_out.to = toid;
    rela_out.id = id;
    if is_last_edge {
        rela_out.button_config = Some(workflow::ButtonConfig {
            cancel: workflow::Cancel {
                enable: true,
                text: "Cancel".to_string(),
            },
            approve: workflow::Approve {
                enable: true,
                text: "Approve".to_string(),
            },
            request_update: workflow::RequestUpdate {
                enable: false,
                text: "Request Update".to_string(),
            },
        })
    }

    rela_out
}
