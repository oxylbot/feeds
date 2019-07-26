const superagent = require("superagent");

module.exports = async subreddit => {
	try {
		const { body } = await superagent.get(`https://www.reddit.com/r/${subreddit}/hot.json`);

		return body.data.children.map(post => post.data).filter(post => !post.stickied);
	} catch(err) {
		return [];
	}
};
