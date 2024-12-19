//#![allow(dead_code)]
// we use smart contracts crates and VecDeque
use smart_contract_macros::smart_contract;

use smart_contract::log;
use smart_contract::payload::Parameters;

use std::collections::VecDeque;

//maximum number of messages can be stored in the chat
const MAX_CHATTING_CAPA: usize = 100;
//maximum number of characters allowed in a message
const MAX_MESSAGE_CHARACT: usize = 300;

//the sender is identified by a public key of 32 bytes
struct Message {
    sender: [u8; 32],
    message: String,
}

// the chat is a VecDeque containing the messages
struct Chat {
    chatting: VecDeque<Message>,
}

impl Chat {
    //this function ensure the number of message below 301
    fn remove_old_mess(&mut self) {
        while self.chatting.len() > MAX_CHATTING_CAPA {
            self.chatting.pop_front();
        }
    }
    //transform the key to a string
    fn hex_to_str(tab: [u8; 32]) -> String {
        let str: Vec<String> = tab.iter().map(|b| format!("{:02x}", b)).collect();
        str.join("")
    }
}

#[smart_contract]
impl Chat {
    // create a new chat Parameters is a type defined in smart_contract crates is used for store the input provided from the outside to the smart contract
    fn init(_parameters: &mut Parameters) -> Self {
        Self {
            chatting: VecDeque::new(),
        }
    }
    //send a message after check if it is not empty or too long
    fn send_message(&mut self, parameters: &mut Parameters) -> Result<(), String> {
        let mess = Message {
            sender: parameters.sender,
            message: parameters.read(),
        };

        if mess.message.len() > MAX_MESSAGE_CHARACT {
            return Err(format!(
                "The number of message characters can not go over {}.",
                MAX_MESSAGE_CHARACT
            ));
        }

        if mess.message.is_empty() {
            return Err("Message is empty.".to_string());
        }

        self.chatting.push_back(mess);
        self.remove_old_mess();
        Ok(())
    }
    // allowing to display all chat messages (sender and purpose) up to a limit of 300
    fn get_messages(&mut self, _params: &mut Parameters) -> Result<(), String> {
        let messages: Vec<String> = self
            .chatting
            .iter()
            .rev()
            .map(|chat| format!("<{}> {}", Chat::hex_to_str(chat.sender), chat.message))
            .collect();

        log(&messages.join("\n"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_hex() {
        let id = [0x5b; 32];
        let id_hex = Chat::hex_to_str(id);
        assert_eq!(
            id_hex,
            "5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b5b"
        )
    }

    #[test]
    fn test_remove_old_mess() {
        let mut chat = Chat {
            chatting: VecDeque::new(),
        };
        for i in 0..110 {
            chat.chatting.push_back(Message {
                sender: [0; 32],
                message: format!("Message {}", i),
            });
        }
        chat.remove_old_mess();
        assert_eq!(chat.chatting.len(), MAX_CHATTING_CAPA);
        assert_eq!(chat.chatting.front().unwrap().message, "Message 10");
    }
}
