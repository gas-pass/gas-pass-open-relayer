/**
 * GAS PASS - Advanced Solana Gas Solution
 * Main client demonstration
 */

import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { Program, Provider, web3 } from '@project-serum/anchor';
import fs from 'fs';
import BN from 'bn.js';

// Import GAS PASS program interface
import idl from './idl/gaspass.json' assert { type: 'json' };
import { GasPassError } from '../program-rust/src/error.js';

// Configuration
const PROGRAM_ID = new PublicKey('your_program_id_here');
const CLUSTER_URL = clusterApiUrl('devnet');

// Initialize connection
const connection = new Connection(CLUSTER_URL, 'confirmed');

// Provider setup
const provider = new Provider(connection, window.solana, {
  preflightCommitment: 'processed',
});

// Program instance
const program = new Program(idl, PROGRAM_ID, provider);

console.log('🚀 GAS PASS - Advanced Solana Gas Solution');
console.log('Initializing transaction execution...');

/**
 * Main execution function
 */
async function main() {
  try {
    // Establish connection to the cluster
    await connection.connect();
    console.log('✅ Connected to Solana cluster');

    // Connect provider
    await provider.connect();
    console.log('✅ Provider connected');

    // Load program
    await program.load();
    console.log('✅ GAS PASS program loaded');

    // Initialize the program
    await program.initialize();
    console.log('✅ Program initialized');

    // Topup account
    await program.topup();
    console.log('✅ Account topped up');

    // Submit transaction
    await program.submitTx();
    console.log('✅ Transaction submitted');

    // Print state
    await program.printState();
    console.log('✅ State printed');

    console.log('🎉 GAS PASS execution completed successfully!');
  } catch (error) {
    console.error('❌ Error during execution:', error);
    throw error;
  }
}

// Execute main function
main().catch(console.error);

export default main;
