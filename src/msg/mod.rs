extern crate itertools;
extern crate protobuf;

use itertools::Itertools;

use self::protobuf;

enum RaftMessage {
    Rpc,
    Reply,
}

//// identify

struct Identify {
    serverId: ServerId,
}

//// requests

enum Rpc {
    AppendEntries(AppendEntries),
    RequestVote(RequestVote),
}

struct AppendEntries {
    term: usize,
    leaderId: ServerId,
    prevLogIndex: usize,
    prevLogTerm: usize,
    entries: Vec<LogEntry>,
    leaderCommit: usize,
}

struct RequestVote {
    term: usize,
    candidateId: ServerId,
    lastLogIndex: usize,
    lastLogTerm: usize,
}

impl<T> From<T> for AppendEntries
    where T: protobuf::AppendEntries
{
    fn from(proto: T) -> AppendEntries {
        let term = proto.get_term();
        let leaderId = proto.get_leaderId();
        let prevLogIndex = proto.get_prevLogIndex();
        let prevLogTerm = proto.get_prevLogTerm();
        let leaderCommit = proto.get_leaderCommit();

        let entries = {
            let entries = proto.get_entry();
            entries.into_iter().collect_vec();
        };

        AppendEntries {
            term: term,
            leaderId: leaderId,
            prevLogIndex: prevLogIndex,
            prevLogTerm: prevLogTerm,
            leaderCommit: leaderCommit,
        }
    }
}

impl<T> From<T> for RequestVote
    where T: protobuf::RequestVote
{
    fn from(proto: T) -> RequestVote {
        let term = proto.get_term();
        let candidateId = proto.get_candidateId();
        let lastLogIndex = proto.get_lastLogIndex();
        let lastLogTerm = proto.get_lastLogTerm();

        RequestVoteReply {
            term: term,
            candidateId: candidateId,
            lastLogIndex: lastLogIndex,
            lastLogTerm: lastLogTerm,
        }
    }
}

//// replies

enum Reply {
    AppendEntries(AppendEntriesReply),
    RequestVote(RequestVoteReply),
}

struct AppendEntriesReply {
    term: usize,
    success: bool,
}
struct RequestVoteReply {
    term: usize,
    voteGranted: bool,
}

impl<T> From<T> for AppendEntriesReply
    where T: protobuf::AppendEntriesReply
{
    fn from(proto: T) -> AppendEntriesReply {
        let term = proto.get_term();
        let success = proto.get_success();

        AppendEntriesReply {
            term: term,
            success: success,
        }
    }
}

impl<T> From<T> for RequestVoteReply
    where T: protobuf::RequestVoteReply
{
    fn from(proto: T) -> RequestVoteReply {
        let term = proto.get_term();
        let voteGranted = proto.get_voteGranted();

        RequestVoteReply {
            term: term,
            voteGranted: voteGranted,
        }
    }
}
