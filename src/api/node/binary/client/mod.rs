pub struct GetInformationClient {}

impl GetInformationClient {
    async fn get_block_header(&self) -> Result<BinaryResponseAndRequest, Error>;
}

pub struct GetClient {
    information: GetInformationClient,
}

pub struct Client {
    get: GetClient,
}
