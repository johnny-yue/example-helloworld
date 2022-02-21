use anyhow::Result;
mod solana;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use solana_sdk::transaction::Transaction;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct GreetingAccount {
    pub text: Vec<u8>,
}

fn main() -> Result<()> {
    let config = solana::load_config()?;

    println!("url - {:?}", config.json_rpc_url);

    let client = solana::connect(&config)?;

    let program_keypair = solana::get_program_keypair(&client)?;
    let program_id = program_keypair.pubkey();
    println!("program id: {}", program_id);

    // let program_instance_account =
    //     solana::get_program_instance_account(&client, &config.keypair, &program_keypair)?;

    let payer_account = client.get_account(&config.keypair.pubkey())?;
    println!("payer_account - {:?}", payer_account);

    static SEED: &str = "helloa";
    let greeted_pubkey = Pubkey::create_with_seed(&config.keypair.pubkey(), SEED, &program_id)?;
    println!("greeted_pubkey - {:?}", greeted_pubkey);

    let greeting = GreetingAccount {
        text: "hello kkk".as_bytes().to_vec(),
    };

    let instruction = Instruction {
        program_id: program_id,
        accounts: vec![AccountMeta::new(greeted_pubkey, false)],
        data: greeting.try_to_vec()?,
    };

    println!("instruction - {:?}", instruction);

    let mut tx = Transaction::new_with_payer(&[instruction], Some(&config.keypair.pubkey()));
    let blockhash = client.get_recent_blockhash()?.0;
    tx.try_sign(&[&config.keypair], blockhash)?;
    let sig = client.send_and_confirm_transaction_with_spinner(&tx)?;

    Ok(())
}
