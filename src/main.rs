use cosmos::{Address, CosmosNetwork};

#[derive(serde::Serialize)]
struct Record {
    tapper: Address,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum QueryMsg {
    Tappers { start_after: Option<Address> },
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
struct TappersResp {
    tappers: Vec<Address>,
}

const FAUCET_ADDR: &str = "sei1czf3age49j99yrkxynmetysh3p2x8tp429mv23jme7zj8hytwhesnqngza";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut builder = CosmosNetwork::SeiTestnet.builder();
    builder.grpc_url = "https://sei-grpc.kingnodes.com".to_owned();
    let cosmos = builder.build().await?;
    let mut csv = csv::Writer::from_path("sei-tappers.csv")?;

    let mut start_after = None;
    let mut total = 0;
    let contract = cosmos.make_contract(FAUCET_ADDR.parse()?);

    loop {
        let mut retries = 0;
        let TappersResp { tappers } = loop {
            match contract.query(QueryMsg::Tappers { start_after }).await {
                Ok(x) => break x,
                Err(e) => {
                    eprintln!("Error while querying, retries == {retries}: {e:?}");
                    anyhow::ensure!(retries <= 20, "Too many retries, giving up");
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    retries += 1;
                }
            }
        };
        match tappers.last() {
            None => {
                csv.flush()?;
                println!("Final total of tappers: {}", total);
                break Ok(());
            }
            Some(last) => start_after = Some(*last),
        }

        for tapper in tappers {
            total += 1;
            if total % 1000 == 0 {
                println!("Total so far: {total}");
            }
            csv.serialize(Record { tapper })?;
        }
        csv.flush()?;
    }
}
