use candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct EncryptedNote {
    pub id: u128,
    pub encrypted_text: String,
}

#[derive(Default)]
pub struct Notes {
    pub notes: HashMap<Principal, Vec<EncryptedNote>>,
}

impl Notes {
    pub fn get_notes(&self, caller: Principal) -> Vec<EncryptedNote> {
        self.notes.get(&caller).cloned().unwrap_or_default()
    }

    pub fn add_note(&mut self, caller: Principal, encrypted_text: String) {
        let notes_of_caller = self.notes.entry(caller).or_default();
        let id = notes_of_caller.len() as u128;
        notes_of_caller.push(EncryptedNote { id, encrypted_text });
    }

    pub fn delete_note(&mut self, caller: Principal, id: u128) {
        if let Some(notes_of_caller) = self.notes.get_mut(&caller) {
            notes_of_caller.retain(|n| n.id != id); // 条件式がtrueのものだけ残します。
        }
    }

    pub fn update_note(&mut self, caller: Principal, new_note: EncryptedNote) {
        if let Some(current_note) = self
            .notes
            .get_mut(&caller)
            .and_then(|notes_of_caller| notes_of_caller.iter_mut().find(|n| n.id == new_note.id))
        {
            current_note.encrypted_text = new_note.encrypted_text;
        }
    }
}