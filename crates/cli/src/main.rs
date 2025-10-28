    //! The CLI Runner Crate
    //!
    //! This is the main entry point for the headless/server version of the
    //! application. Its sole responsibility is to initialize the configuration
    //! and start the main service layer.

    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        println!("CLI starting the service...");
        sentiric_service::run().await
    }