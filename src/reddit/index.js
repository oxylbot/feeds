const embedConverter = require("./embedConverter");
const getHot = require("./getHot");
const getNew = require("./getAllNew");

const subreddits = {
	hot: [],
	new: []
};

const subscriptions = {
	hot: [],
	new: []
};

module.exports = async (redis, bucket) => {
	const hotSubs = await redis.keys("hot:*");
	const newSubs = await redis.keys("new:*");

	subreddits.hot.push(...hotSubs.map(key => key.substring(key.indexOf(":") + 1)));
	subreddits.new.push(...newSubs.map(key => key.substring(key.indexOf(":") + 1)));

	setInterval(async () => {
		const newPosts = await getNew(subreddits.new);
		const exists = await redis.multi(newPosts.map(post => ["exists", `posts:${post.id}`])).exec();

		redis.multi();
		newPosts.filter((post, index) => exists[index]).forEach(post => {
			redis.set(`posts:${post.id}`, "", "EX", 3600);

			subscriptions.new[post.subreddit].forEach(channelId => {
				bucket.request("createChannelMessage", {
					channelId: channelId,
					content: "",
					embed: embedConverter(post)
				});
			});
		});

		await redis.exec();
	}, 30000);

	setInterval(async () => {
		subreddits.hot.forEach(async subreddit => {
			const hotPosts = await getHot(subreddit);
			const exists = await redis.multi(hotPosts.map(post => ["exists", `posts:${post.id}`])).exec();

			redis.multi();
			hotPosts.filter((post, index) => exists[index]).forEach(post => {
				redis.set(`posts:${post.id}`, "", "EX", 604800);

				subscriptions.hot[post.subreddit].forEach(channelId => {
					bucket.request("createChannelMessage", {
						channelId: channelId,
						content: "",
						embed: embedConverter(post)
					});
				});
			});

			await redis.exec();
		});
	}, 900000);
};

module.exports.message = async (redis, message) => {
	message = JSON.parse(message);
	// { "op": "drop/add", "type": "hot/new", "subreddit": "me_irl", "channelId": "...." }

	if(message.op === "drop") {
		await redis.set(`${message.type}:${message.subreddit}`);

		subreddits[message.type].splice(
			subreddits[message.type].indexOf(message.subreddit),
			1
		);
		subscriptions[message.type][message.subreddit].splice(
			subscriptions[message.type][message.subreddit].indexOf(message.channelId),
			1
		);
	} else if(message.op === "add") {
		await redis.del(`${message.type}:${message.subreddit}`);

		subreddits[message.type].push(message.subreddit);
		subscriptions[message.type][message.subreddit] = (subscriptions[message.type][message.subreddit] || [])
			.concat(message.channelId);
	}
};
