ERROR ydb::client_table: error=YdbStatusError(YdbStatusError { message: "Operation { id: \"\", ready: true, status: BadRequest, issues: [IssueMessage { position: None, message: \"contrib/ydb/core/kqp/session_actor/kqp_session_actor.cpp:902: Invalid value representation for type: Uint8, expected value case: 3, but current: 2\", end_position: None, issue_code: 0, severity: 1, issues: [] }], result: Some(Any { type_url: \"type.googleapis.com/Ydb.Table.ExecuteQueryResult\", value: [18, 28, 10, 26, 48, 49, 106, 56, 122, 107, 52, 102, 98, 107, 102, 110, 48, 53, 52, 51, 115, 57, 54, 122, 54, 56, 115, 122, 102, 97] }), metadata: None, cost_info: None }", operation_status: 400010, issues: [YdbIssue { issue_code: 0, message: "contrib/ydb/core/kqp/session_actor/kqp_session_actor.cpp:902: Invalid value representation for type: Uint8, expected value case: 3, but current: 2", issues: [], severity: Error }] })
Error: YdbStatusError(YdbStatusError { message: "Operation { id: \"\", ready: true, status: BadRequest, issues: [IssueMessage { position: None, message: \"contrib/ydb/core/kqp/session_actor/kqp_session_actor.cpp:902: Invalid value representation for type: Uint8, expected value case: 3, but current: 2\", end_position: None, issue_code: 0, severity: 1, issues: [] }], result: Some(Any { type_url: \"type.googleapis.com/Ydb.Table.ExecuteQueryResult\", value: [18, 28, 10, 26, 48, 49, 106, 56, 122, 107, 52, 102, 98, 107, 102, 110, 48, 53, 52, 51, 115, 57, 54, 122, 54, 56, 115, 122, 102, 97] }), metadata: None, cost_info: None }", operation_status: 400010, issues: [YdbIssue { issue_code: 0, message: "contrib/ydb/core/kqp/session_actor/kqp_session_actor.cpp:902: Invalid value representation for type: Uint8, expected value case: 3, but current: 2", issues: [], severity: Error }] })
