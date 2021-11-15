use crate::error::MailError::NotWritable;
use crate::instruction::MailInstruction;
use crate::state::{Mail, MailAccount};
use borsh::BorshSerialize;
use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  pubkey::Pubkey,
};

pub struct Processor;
impl Processor {
    pub fn process(
      program_id: &Pubkey;
      accounts: &[AccountInfo],
      instruction_data: &[u8],
    ) -> ProgramResult {
    let instruction = MailInstruction::unpack(instruction_data)?;

    match instruction {
      MailInstruction::InitAccount => {
        msg!("Instruction: InitAccount");
        Self::process_init_account(accounts, program_id)
      }
      MailInstruction::SendMail { mail } => {
        msg!("Instruction: SendMail");
        Self::process_send_mail(accounts, mail, program_id)
      }
    }
  }
    fn process_init_account(
        account: &AccountInfo,
        program_id: &Pubkey
    ) -> ProgramResult {
        if !account.is_writable {
            return Err(NotWritable.into());
          }
        
          if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
          }
          let welcome_mail = Mail {
            id: String::from("00000000-0000-0000-0000-000000000000"),
            from_address: program_id.to_string(),
            to_address: account.key.to_string(),
            subject: String::from("Welcome to SolMail"),
            body: String::from("This is the start of your private messages on SolMail
            Lorem, ipsum dolor sit amet consectetur adipisicing elit. Quos ut labore, debitis assumenda, dolorem nulla facere soluta exercitationem excepturi provident ipsam reprehenderit repellat quisquam corrupti commodi fugiat iusto quae voluptates!"),
            sent_date: "11/15/2021, 3:58:02 PM"
          };
          
          let mail_account = MailAccount {
            inbox: vec![welcome_mail],
            sent: Vec::new(),
          };
          
          mail_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
    }

    fn process_send_mail(accounts: &[AccountInfo], mail: &Mail, program_id: &Pubkey) -> ProgramResult {
        let sender_account = &accounts[0];

        if !sender_account.is_writable {
        return Err(NotWritable.into());
        }

        if sender_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
        }

        let receiver_account = &accounts[1];

        if !receiver_account.is_writable {
        return Err(NotWritable.into());
        }

        if receiver_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
        }

        let sender_data = MailAccount::try_from_slice(&sender_account.data.borrow()[..]);
        sender_data.sent.push(mail.clone());
        sender_data.serialize(&mut &mut sender_account.data.borrow_mut()[..])?;

        receiver_data = MailAccount::try_from_slice(&receiver_account.data.borrow()[..])?;
        receiver_data.inbox.push(mail.clone());
        receiver_data.serialize(&mut &mut receiver_account.data.borrow_mut()[..])?;
        Ok(())
      }
}