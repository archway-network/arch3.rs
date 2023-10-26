use anyhow::Result;
use archway_proto::archway;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "https://grpc.mainnet.archway.io:443";
    let channel = Channel::from_static(addr).connect().await?;

    let mut client = archway::rewards::v1::query_client::QueryClient::new(channel);

    let request = archway::rewards::v1::QueryContractMetadataRequest {
        contract_address: "archway1qxggkw5v33yefppsckd0ttentdhwuqwn2gppnw5dwldtekytvm3szhqvez"
            .to_string(),
    };

    let response = client.contract_metadata(request).await?;
    let metadata = response.into_inner();

    println!("{:?}", metadata);

    Ok(())
}
