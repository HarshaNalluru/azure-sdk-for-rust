// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell::ignore Uncategorized

use azure_core_test::recorded;
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{ConsumerClient, OpenReceiverOptions, StartPosition};
use futures::{pin_mut, StreamExt};
use std::{env, error::Error, time::Duration};
use tokio::time::timeout;
use tracing::{info, trace};

mod common;

#[recorded::test(live)]
async fn consumer_new() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let _client = ConsumerClient::builder()
        .with_application_id("test_new")
        .open(
            host.as_str(),
            eventhub.as_str(),
            DefaultAzureCredential::new()?,
        )
        .await?;

    Ok(())
}

#[recorded::test(live)]
async fn consumer_new_with_error() -> Result<(), Box<dyn Error>> {
    common::setup();
    trace!("test_new_with_error");
    let eventhub = env::var("EVENTHUB_NAME")?;
    let result = ConsumerClient::builder()
        .with_application_id("test_new")
        .open(
            "invalid_host",
            eventhub.as_str(),
            DefaultAzureCredential::new()?,
        )
        .await;
    assert!(result.is_err());
    match result {
        Err(err) => {
            info!("Error: {:?}", err);
            assert_eq!(*err.kind(), azure_core::error::ErrorKind::Io);
            if matches!(err.kind(), azure_core::error::ErrorKind::Io) {
                // Dig into the I/O error to determine the type.
                let e = err
                    .source()
                    .unwrap()
                    .downcast_ref::<std::io::Error>()
                    .unwrap();
                info!("IO Error: {}", e);
                info!("IO Error Kind: {:?}", e.kind());
                let e = e.source();
                info!("IO Error Source: {:?}", e);
            } else {
                panic!("Expected IO Error");
            }
        }
        _ => panic!("Expected Error (cannot hit due to previous assert)"),
    }

    Ok(())
}

#[recorded::test(live)]
async fn consumer_open() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let _client = ConsumerClient::builder()
        .with_application_id("test_open")
        .open(
            host.as_str(),
            eventhub.as_str(),
            azure_identity::DefaultAzureCredential::new()?,
        )
        .await?;

    Ok(())
}
#[recorded::test(live)]
async fn consumer_close() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let client = ConsumerClient::builder()
        .with_application_id("test_open")
        .open(
            host.as_str(),
            eventhub.as_str(),
            azure_identity::DefaultAzureCredential::new()?,
        )
        .await?;
    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn consumer_get_properties() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    let client = ConsumerClient::builder()
        .with_application_id("test_open")
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let properties = client.get_eventhub_properties().await?;
    info!("Properties: {:?}", properties);
    assert_eq!(properties.name, eventhub);

    Ok(())
}

#[recorded::test(live)]
async fn consumer_get_partition_properties() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    let client = ConsumerClient::builder()
        .with_application_id("test_open")
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let properties = client.get_eventhub_properties().await?;

    for partition_id in properties.partition_ids {
        let partition_properties = client
            .get_partition_properties(partition_id.as_str())
            .await?;
        info!("Partition properties: {:?}", partition_properties);
        assert_eq!(partition_properties.id, partition_id);
    }

    Ok(())
}

#[recorded::test(live)]
async fn receive_lots_of_events() -> Result<(), Box<dyn Error>> {
    common::setup();

    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    info!("Establishing credentials.");

    let credential = DefaultAzureCredential::new()?;

    info!("Creating client.");
    let client = ConsumerClient::builder()
        .with_application_id("test_open")
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    let receiver = client
        .open_receiver_on_partition(
            "0",
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: azure_messaging_eventhubs::StartLocation::Earliest,
                    ..Default::default()
                }),
                // Timeout for individual receive operations.
                receive_timeout: Some(Duration::from_secs(5)),
                ..Default::default()
            }),
        )
        .await?;

    info!("Creating event receive stream.");
    let event_stream = receiver.stream_events();

    pin_mut!(event_stream); // Needed for iteration.

    let mut count = 0;

    const TEST_DURATION: std::time::Duration = Duration::from_secs(10);
    info!("Receiving events for {:?}.", TEST_DURATION);

    // Read events from the stream for a bit of time.

    let result = timeout(TEST_DURATION, async {
        while let Some(event) = event_stream.next().await {
            match event {
                Ok(_event) => {
                    //                    info!("Received the following message:: {:?}", event);
                    count += 1;
                }
                Err(err) => {
                    info!("Error while receiving message: {:?}", err);
                }
            }
        }
    })
    .await;

    info!("Received {count} messages in {TEST_DURATION:?}. Timeout: {result:?}");

    Ok(())
}
