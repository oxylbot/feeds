const path = require("path");
const protobuf = require("protobufjs");
const Redis = require("ioredis");

const reddit = require("./reddit/index");

const redis = new Redis({
	port: +process.env.REDIS_SERVICE_PORT,
	host: "redis",
	family: 4,
	db: +process.env.REDIS_DATABASE,
	maxRetriesPerRequest: null,
	reconnectOnError(error) {
		return error.message.startsWith("connect ETIMEDOUT");
	}
});

redis.on("error", error => {
	if(error.message.startsWith("connect ETIMEDOUT")) {
		redis.connect();
	}
});

const BucketClient = require("./sockets/bucketClient");

const bucketClient = new BucketClient();

async function init() {
	const rpcProto = await protobuf.load(path.resolve(__dirname, "..", "bucket-proto", "rpcWrapper.proto"));
	const discordProto = await protobuf.load(path.resolve(__dirname, "..", "bucket-proto", "service.proto"));

	bucketClient.start({
		discord: discordProto,
		rpc: rpcProto
	});

	await redis.subscribe("reddit");
	await reddit(redis);

	redis.on("message", (channel, message) => {
		if(channel === "reddit") reddit.message(redis, message);
	});
}

init();
