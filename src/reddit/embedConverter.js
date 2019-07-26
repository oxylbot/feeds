module.exports = post => {
	const embed = {
		author: {
			name: post.subreddit_name_prefixed,
			url: `https://reddit.com/u/${post.author}`
		},
		timestamp: new Date(post.created * 1000),
		title: post.title,
		url: `https://reddit.com${post.permalink}`,
		footer: { text: "u/bdstel" }
	};

	if(post.selftext) {
		embed.description = post.selftext;
		embed.thumbnail = { url: "https://www.redditstatic.com/icon.png" };
	}

	if(/\.(jpe?g|png|gifv?)$/.test(post.url)) embed.image = { url: post.url };
	return embed;
};
