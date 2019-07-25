const path = require("path");
const protobuf = require("protobufjs");

const BucketClient = require("./sockets/bucketClient");

const bucketClient = new BucketClient();

async function init() {
	const rpcProto = await protobuf.load(path.resolve(__dirname, "..", "protobuf", "rpcWrapper.proto"));
	const discordProto = await protobuf.load(path.resolve(__dirname, "..", "protobuf", "discordapi", "service.proto"));

	bucketClient.start({
		discord: discordProto,
		rpc: rpcProto
	});
}

init();
