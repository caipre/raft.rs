macro_rules! pass { () => ({}) }

macro_rules! dbg {
    ($v:path) => ( format!("{:?}", $v) );
    ($v:ident) => ( format!("{:?}", $v) );
    ($v:expr) => ( format!("{:?}", $v) );
}

macro_rules! raftmsg {
    ($id:expr, $msg:ident) => ({
        RaftMsg {
            xtype: ExchangeType::from(&$msg),
            xpart: ExchangePart::from(&$msg),
            sender: $id,
            data: MsgData::from($msg),
        }
    })
}

macro_rules! from {
    ($e:ty, $p:path, $t:ty) => (
        impl<'a> From<&'a $t> for $e {
            fn from(_: &'a $t) -> $e {
                $p
            }
        }
    )
}

macro_rules! fromdata {
    ($e:ty, $p:path, $t:ty) => (
        impl From<$t> for $e {
            fn from(data: $t) -> $e {
                $p(data)
            }
        }
    )
}

macro_rules! reply
    { ($self_:ident, $val:expr) => (($self_.election.term, $val).into()) }

macro_rules! append_entries {
    ($self_:ident, $entries:expr) => ({
        AppendEntriesRequest {
            term: $self_.election.term,
            leaderId: $self_.identity.id,
            prevLogIndex: 0, // FIXME
            prevLogTerm: 0, // FIXME
            entries: $entries,
            leaderCommit: $self_.log.cindex,
        }
    })
}

macro_rules! heartbeat {
    ($self_:ident) => ({
        let msg = append_entries!($self_, vec![]);
        raftmsg!($self_.identity.id, msg)
    })
}

macro_rules! request_votes {
    ($self_:ident) => ({
        let msg = {
            let entry = $self_.log.entries.last();
            RequestVotesRequest {
                term: $self_.election.term,
                candidateId: $self_.identity.id,
                lastLogIndex: entry.map_or(0, |e| e.index),
                lastLogTerm: entry.map_or(0, |e| e.term),
            }
        };
        raftmsg!($self_.identity.id, msg)
    })
}

macro_rules! bindport
    { ($port:expr) => { format!("127.0.0.1:{}", $port).as_str() } }

macro_rules! localport
    { ($socket:expr) => { $socket.local_addr.map(|addr| addr.port()).unwrap() } }

