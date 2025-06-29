use bs58;
use solana_client::rpc_client::RpcClient;
use solana_program::{hash::hash, pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::{
    message::Message,
    signature::{Keypair, Signer, read_keypair_file},
    system_program,
    transaction::Transaction,
};
use std::io::{self, BufRead};
use std::str::FromStr;
const RPC_URL: &str =
    "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

#[cfg(test)]
mod tests {
    use solana_sdk::signature::Signer;
    use solana_sdk::signer::keypair::Keypair;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }
    #[test]
    fn submit_rs() {}
}
// 2ZkvMc2MT3sAp1MLT4JeM9V5KRy7wQPS98fBERJFX4Yt

#[test]
fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
} //D4RJ87U8r4vA1VrpMi1GCqT6a7R6tMEjB8QYvubLvytA

#[test]
fn wallet_to_base58() {
    println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
    let stdin = io::stdin();
    let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("Your Base58-encoded private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}
#[test]
fn claim_airdrop() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    // we'll establish a connection to Solana devnet using the const we defined above
    let client = RpcClient::new(RPC_URL);
    // We're going to claim 2 devnet SOL tokens (2 billion lamports)
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(sig) => {
            println!("Success! Check your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
        }
        Err(err) => {
            println!("Airdrop failed: {}", err);
        }
    }
}

#[test]
fn transfer_sol() {
    // Load your devnet keypair from file
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Generate a signature from the keypair
    let pubkey = keypair.pubkey();
    let message_bytes = b"I verify my Solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    // Verify the signature using the public key
    if sig.verify(pubkey.as_ref(), message_bytes) {
        println!("Signature verified");
    } else {
        println!("Verification failed");
    }

    // Step 4: Define the destination (Turbin3) address
    let to_pubkey = Pubkey::from_str("FpaNW2AwxFxfen9jbscZabsfhn5n1ZXnjDVniq7jNdfq").unwrap();
    // Step 5: Connect to devnet
    let rpc_client = RpcClient::new(RPC_URL);
    // Step 6: Fetch recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    // Step 8: Send the transaction and print tx
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]
fn transfer_all() {
    // Load your devnet keypair from file
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

    // Step 4: Define the destination (Turbin3) address
    let to_pubkey = Pubkey::from_str("FpaNW2AwxFxfen9jbscZabsfhn5n1ZXnjDVniq7jNdfq").unwrap();
    // Step 5: Connect to devnet
    let rpc_client = RpcClient::new(RPC_URL);
    // Step 6: Fetch recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    // Step 2: Get current balance
    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");
    // Step 3: Build a mock transaction to calculate fee
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );
    // Step 4: Estimate transaction fee
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");
    // Step 5: Create final transaction with balance minus fee
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );
    // This ensures we leave zero lamports behind.
    // Step 6: Send transaction and verify
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send final transaction");
    println!(
        "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}

#[test]
fn submit_rs() {
    // Step 1: Create a Solana RPC client
    let rpc_client = RpcClient::new(RPC_URL);

    // Step 2: Load your signer keypair
    // Make sure to use the correct path for your wallet file. This keypair will sign and pay for the transaction.
    let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

    // Step 3: Define program and account public keys
    // Specify all the public keys of the program and accounts your instruction will interact with.
    let mint = Keypair::new();
    let turbin3_prereq_program =
        Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
    let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
    let mpl_core_program =
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
    let system_program = system_program::id();

    // Step 4: Get the PDA (Program Derived Address)
    // We already created this PDA earlier in our TypeScript enrollment. Now we’re replicating the same logic in Rust.
    // The byte array of the string “prereqs”
    // The byte array of the signer's public key
    // These are combined and used along with the program ID to create a deterministic address for the account:
    let signer_pubkey = signer.pubkey();
    let seeds = &[b"prereqs", signer_pubkey.as_ref()];
    let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
    let seeds_authority = &[b"collection", collection.as_ref()];
    let (authority, _bump) = Pubkey::find_program_address(seeds_authority, &turbin3_prereq_program);

    // Step 5: Prepare the instruction data (discriminator)
    // The discriminator uniquely identifies the instruction your program expects.
    // From the IDL, the submit_rs instruction discriminator is:
    let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

    // Step 6: Define the accounts metadata
    let accounts = vec![
        AccountMeta::new(signer.pubkey(), true),     // user signer
        AccountMeta::new(prereq_pda, false),         // PDA account
        AccountMeta::new(mint.pubkey(), true),       // mint keypair
        AccountMeta::new(collection, false),         // collection
        AccountMeta::new_readonly(authority, false), // authority (PDA)
        AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
        AccountMeta::new_readonly(system_program, false), // system program
    ];
    // Use new for accounts that the instruction writes to and new_readonly for accounts that are read-only. The true flag indicates the account must sign the transaction.
    // Step 7: Get the recent blockhash
    // We need a recent blockhash to build the transaction:
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    // Step 8: Build the instruction
    // Construct the instruction by specifying the program ID, accounts, and instruction data.
    let instruction = Instruction {
        program_id: turbin3_prereq_program,
        accounts,
        data,
    };
    // Step 9: Create and sign the transaction
    // Create a transaction containing the instruction and sign it with the necessary keypairs.
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.pubkey()),
        &[&signer, &mint],
        blockhash,
    );
    // Step 10: Send and confirm the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    println!(
        "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
