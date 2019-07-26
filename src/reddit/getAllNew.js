const superagent = require("superagent");

module.exports = async subreddits => {
	try {
		const { body } = await superagent.get("https://www.reddit.com/r/all/new.json?limit=100");

		return body.data.children.map(post => post.data).filter(post => subreddits.include(post.subreddit));
	} catch(err) {
		return [];
	}
};
