use anyhow::Result;
use ethers::prelude::*;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

abigen!(
    UniswapV3,
    r#"[
        event Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick)
    ]"#
);

const CONTRACT_V3_ADDR: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
const INFURA_WS_URL: &str = "wss://mainnet.infura.io/ws/v3";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <infura_api_key>", args[0]);
        return Ok(());
    }

    let infura_api_key = &args[1];
    let url = format!("{}/{}", INFURA_WS_URL, infura_api_key);
    let client = Arc::new(Provider::<Ws>::connect(&url).await?);
    let contract = UniswapV3::new(Address::from_str(CONTRACT_V3_ADDR)?, Arc::clone(&client));

    let events = contract.event::<SwapFilter>();
    let mut stream = events.stream().await?.with_meta();
    let mut tasks = Vec::new();

    while let Some(Ok((event, meta))) = stream.next().await {
        let provider = Arc::clone(&client);
        let task = tokio::spawn(async move {
            let timestamp = provider
                .get_block(meta.block_number)
                .await?
                .map(|block| block.timestamp)
                .unwrap_or_default();

            println!(
                r#"
            ==========================================================
            USDC <-> WETH Swap:
            Timestamp: {:?}
            Block Number: {:?}
            Block Hash: {:?}
            Transaction Hash: {:?}
            Transaction Index: {:?}
            Log Index: {:?}
            Sender: {:?}
            Recipient: {:?}
            USDC: {:?}
            WETH: {:?}
            "#,
                timestamp,
                meta.block_number,
                meta.block_hash,
                meta.transaction_hash,
                meta.transaction_index,
                meta.log_index,
                event.sender,
                event.recipient,
                event.amount_0,
                event.amount_1
            );
            anyhow::Result::<()>::Ok(())
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Error printing swap event: {:?}", e);
        }
    }

    Ok(())
}
