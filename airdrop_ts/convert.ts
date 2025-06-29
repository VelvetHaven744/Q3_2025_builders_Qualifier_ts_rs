import { Keypair } from '@solana/web3.js';
import bs58 from 'bs58';
import promptSync from 'prompt-sync';
import wallet from "./dev-wallet.json";

function base58_to_wallet() {
    const prompt = promptSync();
    console.log('Enter your Base58 encoded wallet:');
    const base58 = prompt(''); // waits for user input

    try {
        const decodedWallet = bs58.decode(base58);
        console.log('Decoded wallet (Uint8Array):', decodedWallet);
        console.log('Decoded wallet (Array):', Array.from(decodedWallet));
    } catch (err) {
        console.error('Invalid Base58 input:', (err as Error).message);
    }
}

function wallet_to_base58() { 
    console.log("\nConverting wallet to base58:");

    // Encode secret key to Base58
    const base58 = bs58.encode(Buffer.from(wallet));
    console.log("Secret Key:", base58);

    // Create keypair from secret key
    const keypair = Keypair.fromSecretKey(Uint8Array.from(wallet));
    console.log("Public Key:", keypair.publicKey.toBase58());
}

// Get command line arguments
const args = process.argv.slice(2);
const command = args[0];

// Execute based on command line argument
if (command === 'base58_to_wallet') {
    base58_to_wallet();
} else if (command === 'wallet_to_base58') {
    wallet_to_base58();
} else {
    console.log('Usage: ts-node converter.ts [base58_to_wallet|wallet_to_base58]');
    console.log('Available commands:');
    console.log('  base58_to_wallet  - Convert Base58 string to wallet array');
    console.log('  wallet_to_base58  - Convert wallet array to Base58 string');
}