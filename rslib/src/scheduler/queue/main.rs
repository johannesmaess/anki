// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use super::CardQueues;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MainQueueEntry {
    pub id: CardId,
    pub mtime: TimestampSecs,
    pub kind: MainQueueEntryKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MainQueueEntryKind {
    New,
    Review,
    InterdayLearning,
}

impl CardQueues {
    fn reduce_counts(&mut self, entry: &MainQueueEntry) -> () {
        match entry.kind {
            MainQueueEntryKind::New => self.counts.new -= 1,
            MainQueueEntryKind::Review => self.counts.review -= 1,
            MainQueueEntryKind::InterdayLearning => {
                // the bug causing learning counts to go below zero should
                // hopefully be fixed at this point, but ensure we don't wrap
                // if it isn't
                self.counts.learning = self.counts.learning.saturating_sub(1)
            }
        };
    }

    /// Remove the head of the main queue, and update counts.
    pub(super) fn pop_main(&mut self) -> Option<MainQueueEntry> {
        self.main.pop_front().map(|head| {
            self.reduce_counts(&head);
            head
        })
    }

    /// Add an undone entry to the top of the main queue.
    pub(super) fn push_main(&mut self, entry: MainQueueEntry) {
        match entry.kind {
            MainQueueEntryKind::New => self.counts.new += 1,
            MainQueueEntryKind::Review => self.counts.review += 1,
            MainQueueEntryKind::InterdayLearning => self.counts.learning += 1,
        };
        self.main.push_front(entry);
    }

    /// todo doc
    pub(super) fn remove_main_learning_card(
        &mut self,
        card_id: CardId,
    ) -> Option<MainQueueEntry> {
        if let Some(position) = self.main.iter().position(|e| e.id == card_id) {
            let entry = self.main.remove(position).unwrap();
            println!("main: removed.");
            self.reduce_counts(&entry);
            Some(entry)
        } else {
            println!("main: not found.");
            None
        }
    }
}
