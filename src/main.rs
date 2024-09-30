    use std::{env, str::FromStr};

    use tracing::{info, Level};

    use ydb::{ydb_params, ClientBuilder, Query, Row, ServiceAccountCredentials};
    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        init_logs();
    
        let connection_string =
            env::var("YDB_CONNECTION_STRING").map_err(|_| "YDB_CONNECTION_STRING not set")?;

        let client = ClientBuilder::new_from_connection_string(connection_string)?
            .with_credentials(ServiceAccountCredentials::from_env()?)
            .client()?;

        info!("Waiting for client");
        client.wait().await?;
        
        let table_client =  client.table_client();

        table_client.retry_execute_scheme_query("CREATE TABLE test3 (id Uint64 NOT NULL, name Utf8, age UInt8, description Utf8, PRIMARY KEY (id))").await?;
        
        let res = table_client.retry_transaction(|mut t| async move {
        
        let desc: Option<String> = None;
        let res = t.query(ydb::Query::from(r#"
            DECLARE $id as Uint64;
            DECLARE $name as Text;
            DECLARE $age as Uint8;
            DECLARE $descr as Text?;

            INSERT INTO test3 (id, name, age, description) VALUES ( $id, $name, $age, $descr)
        "#).with_params(
            ydb_params!(
                "$id" => 3u64,
                "$name" => "test".to_owned(),
                "$age" => 33u8,
                "$descr" => desc
            )
        )).await?;
        t.commit().await?;

        Ok(res)   
        
        }).await?;
        info!("res: {:?}", res);

        let rows: Vec<Row> = table_client
            .retry_transaction(|mut t| async move {
                Ok(
                    t.query(Query::new("SELECT * FROM test3"))
                        .await?
                        .into_only_result()?
                        .rows()
                        .collect(),
                )
            })
            .await?;
    
        for mut row in rows {
            let id: Option<u64> = row.remove_field_by_name("id")?.try_into()?;
            let name: Option<String> = row.remove_field_by_name("name")?.try_into()?;
            let age: Option<u8> = row.remove_field_by_name("age")?.try_into()?;
            println!("row id '{}' with name '{}' age: '{}'", id.unwrap(), name.unwrap(), age.unwrap());
        }

        Ok(())
    }


    fn init_logs() {
        let level = env::var("RUST_LOG").unwrap_or("INFO".to_string());
        let log_level = Level::from_str(&level).unwrap();
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(log_level)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("Error setting subscriber");
    }
