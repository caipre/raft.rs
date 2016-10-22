protobuf: src/msg/protobuf.rs

etc/raft.proto:
	@mkdir -p etc
	@curl --silent --show-error --fail --output $@ \
		'https://raw.githubusercontent.com/astrieanna/raft.jl/master/proto/raft_rpc.proto'

src/msg/protobuf.rs: etc/raft.proto
	@protoc --rust_out $(dir $@) $<
	@mv $(dir $@)/raft.rs $@
