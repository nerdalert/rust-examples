// You can play around in this function.
fn main() {
    let is_follower = Raft::new(/* ... */);
    // Raft typically comes in groups of 3, 5, or 7. Just 1 for us. :)

    // Simulate this node timing out first.
    let is_candidate = Raft::<Candidate>::from(is_follower);

    // It wins! How unexpected.
    let is_leader = Raft::<Leader>::from(is_candidate);

    // Then it fails and rejoins later, becoming a Follower again.
    let is_follower_again = Raft::<Follower>::from(is_leader);

    // And goes up for election...
    let is_candidate_again = Raft::<Candidate>::from(is_follower_again);

    // But this time it fails!
    let is_follower_another_time = Raft::<Follower>::from(is_candidate_again);
}

// This is our state machine.
struct Raft<S> {
    // ... Shared Values
    state: S,
}

// The three cluster states a Raft node can be in

// If the node is the Leader of the cluster services requests and replicates its state.
struct Leader {
    // ... Specific State Values
}

// If it is a Candidate it is attempting to become a leader due to timeout or initialization.
struct Candidate {
    // ... Specific State Values
}

// Otherwise the node is a follower and is replicating state it receives.
struct Follower {
    // ... Specific State Values
}

// Raft starts in the Follower state
impl Raft<Follower> {
    fn new(/* ... */) -> Self {
        // ...
        Raft {
            // ...
            state: Follower { /* ... */ },
        }
    }
}

// The following are the defined transitions between states.

// When a follower timeout triggers it begins to campaign
impl From<Raft<Follower>> for Raft<Candidate> {
    fn from(val: Raft<Follower>) -> Raft<Candidate> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Candidate { /* ... */ },
        }
    }
}

// If it doesn't receive a majority of votes it loses and becomes a follower again.
impl From<Raft<Candidate>> for Raft<Follower> {
    fn from(val: Raft<Candidate>) -> Raft<Follower> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Follower { /* ... */ },
        }
    }
}

// If it wins it becomes the leader.
impl From<Raft<Candidate>> for Raft<Leader> {
    fn from(val: Raft<Candidate>) -> Raft<Leader> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Leader { /* ... */ },
        }
    }
}

// If the leader becomes disconnected it may rejoin to discover it is no longer leader
impl From<Raft<Leader>> for Raft<Follower> {
    fn from(val: Raft<Leader>) -> Raft<Follower> {
        // ... Logic prior to transition
        Raft {
            // ... attr: val.attr
            state: Follower { /* ... */ },
        }
    }
}

// -----------------------------------------------------------
// -----------------------------------------------------------
// -----------------------------------------------------------
// -----------------------------------------------------------

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=74375077674b32134d8d071197e58d76

// fn main() {
//     // The `<StateA>` is implied here. We don't need to add type annotations!
//     let in_state_a = StateMachine::new("Blah blah blah".into());

//     // This is okay here. But later once we've changed state it won't work anymore.
//     in_state_a.some_unrelated_value;
//     println!("Starting Value: {}", in_state_a.state.start_value);

//     // Transition to the new state. This consumes the old state.
//     // Here we need type annotations (since not all StateMachines are linear in their state).
//     let in_state_b = StateMachine::<StateB>::from(in_state_a);

//     // This doesn't work! The value is moved when we transition!
//     // in_state_a.some_unrelated_value;
//     // Instead, we can use the existing value.
//     in_state_b.some_unrelated_value;

//     println!("Interm Value: {:?}", in_state_b.state.interm_value);

//     // And our final state.
//     let in_state_c = StateMachine::<StateC>::from(in_state_b);

//     // This doesn't work either! The state doesn't even contain this value.
//     // in_state_c.state.start_value;

//     println!("Final state: {}", in_state_c.state.final_value);
// }

// // Here is our pretty state machine.
// struct StateMachine<S> {
//     some_unrelated_value: usize,
//     state: S,
// }

// // It starts, predictably, in `StateA`
// impl StateMachine<StateA> {
//     fn new(val: String) -> Self {
//         StateMachine {
//             some_unrelated_value: 0,
//             state: StateA::new(val)
//         }
//     }
// }

// // State A starts the machine with a string.
// struct StateA {
//     start_value: String,
// }
// impl StateA {
//     fn new(start_value: String) -> Self {
//         StateA {
//             start_value: start_value,
//         }
//     }
// }

// // State B goes and breaks up that String into words.
// struct StateB {
//     interm_value: Vec<String>,
// }
// impl From<StateMachine<StateA>> for StateMachine<StateB> {
//     fn from(val: StateMachine<StateA>) -> StateMachine<StateB> {
//         StateMachine {
//             some_unrelated_value: val.some_unrelated_value,
//             state: StateB {
//                 interm_value: val.state.start_value.split(" ").map(|x| x.into()).collect(),
//             }
//         }
//     }
// }

// // Finally, StateC gives us the length of the vector, or the word count.
// struct StateC {
//     final_value: usize,
// }
// impl From<StateMachine<StateB>> for StateMachine<StateC> {
//     fn from(val: StateMachine<StateB>) -> StateMachine<StateC> {
//         StateMachine {
//             some_unrelated_value: val.some_unrelated_value,
//             state: StateC {
//                 final_value: val.state.interm_value.len(),
//             }
//         }
//     }
// }

// -----------------------------------------------
// -----------------------------------------------
// -----------------------------------------------
// -----------------------------------------------

// struct JiraTicket {
//     ticket_id: String,
//     current_status: Box<JiraTicketStateMovement>
// }

// impl Debug for JiraTicket {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "##### Ticket {} is at {}", self.ticket_id, self.current_status.get_state())
//     }
// }

// impl JiraTicket {
//     fn new(p_ticket_id: String) -> Box<JiraTicket> {
//         Box::new(JiraTicket {
//             ticket_id: p_ticket_id,
//             current_status: Box::new(
//                 NewState {
//                     state_name: "NEW".to_string()
//                 }
//             )
//         })
//     }
// }

// impl State for JiraTicket {
//     fn get_state(&self) -> String {
//         unimplemented!()
//     }
// }

// impl JiraTicketStateMovement for JiraTicket {

//     fn to_new(&mut self) -> bool {
//         println!("   + {} -> {}", self.current_status.get_state(), "NEW");
//         let move_ok = self.current_status.to_new();
//         if move_ok {
//             self.current_status = Box::new(InProgressState {
//                 state_name: "NEW".to_string()
//             });
//         }
//         move_ok
//     }

//     fn to_in_progress(&mut self) -> bool {
//         println!("   + {} -> {}", self.current_status.get_state(), "IN PROGRESS");

//         let move_ok = self.current_status.to_in_progress();
//         if move_ok {
//             self.current_status = Box::new(InProgressState {
//                 state_name: "IN PROGRESS".to_string()
//             });
//         }
//         move_ok
//     }

//     fn to_done(&mut self) -> bool {
//         println!("   + {} -> {}", self.current_status.get_state(), "DONE");

//         let move_ok = self.current_status.to_done();
//         if move_ok {
//             self.current_status = Box::new(InProgressState {
//                 state_name: "DONE".to_string()
//             });
//         }
//         move_ok
//     }
// }

// trait State {
//     fn get_state(&self) -> String;
// }
// trait JiraTicketStateMovement: State {
//     fn to_new(&mut self) -> bool;
//     fn to_in_progress(&mut self) -> bool;
//     fn to_done(&mut self) -> bool;
// }

// struct NewState {
//     state_name: String
// }
// impl State for NewState {
//     fn get_state(&self) -> String{
//         self.state_name.clone()
//     }
// }
// impl JiraTicketStateMovement for NewState {
//     fn to_new(&mut self) -> bool {
//         println!("      * You are already in the NEW status");
//         false
//     }

//     fn to_in_progress(&mut self) -> bool {
//         true
//     }

//     fn to_done(&mut self) -> bool {
//         println!("      * Cannot move to DONE status");
//         false
//     }
// }

// struct InProgressState {
//     state_name: String
// }
// impl State for InProgressState {
//     fn get_state(&self) -> String{
//         self.state_name.clone()
//     }
// }
// impl JiraTicketStateMovement for InProgressState {
//     fn to_new(&mut self) -> bool {
//         println!("      * Cannot move to NEW status");
//         false
//     }

//     fn to_in_progress(&mut self) -> bool {
//         println!("      * You are already in the IN PROGRESS status");
//         false
//     }

//     fn to_done(&mut self)-> bool {
//         true
//     }
// }

// struct Donestate {
//     state_name: String
// }
// impl State for Donestate {
//     fn get_state(&self) -> String{
//         self.state_name.clone()
//     }
// }
// impl JiraTicketStateMovement for Donestate {
//     fn to_new(&mut self) -> bool {
//         println!("      * Cannot move to NEW status");
//         false
//     }

//     fn to_in_progress(&mut self) -> bool {
//         println!("      * Cannot move to IN PROGRESS status");
//         false
//     }

//     fn to_done(&mut self)-> bool {
//         println!("      * Cannot move to DONE status");
//         false
//     }
// }

// fn main() {

//     let mut jira_ticket = JiraTicket::new("TUR-1".to_string());
//     println!("{:?}", jira_ticket);
//     jira_ticket.to_in_progress();
//     println!("{:?}", jira_ticket);
//     jira_ticket.to_new();
//     println!("{:?}", jira_ticket);
//     jira_ticket.to_done();
//     println!("{:?}", jira_ticket);
// }
