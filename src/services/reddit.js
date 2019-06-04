const { get } = require("axios");

module.exports = async (sub, type) => {
	const redd_data = [];

	const response = await get(`https://www.reddit.com/r/${sub.join("+")}/${type}/.json`);

	const { data } = response.data;

	for(let i = 0; i < data.children.length; i++) {
		const curr_index = data.children[i].data;

		const post = {
			title: curr_index.title,
			url: curr_index.url,
			get preview () {
				let preview;

				if(curr_index.secure_media !== null) {
					if(curr_index.secure_media.reddit_video) {
						return curr_index.secure_media.scrubber_media_url;
					} else if(curr_index.secure_media.type) {
						switch(curr_index.secure_media.type) {
							case "youtube.com":
								preview = curr_index.secure_media.oembed.thumbnail_url;
								break;
						}
					}
				} else if(curr_index.thumbnail.startsWith("https://") || curr_index.thumbnail.startsWith("http://")) {
					preview = curr_index.thumbnail;
				}

				return preview;
			},
			author: curr_index.author,
			created_at: curr_index.created
		};

		redd_data.push(post);
	}

	return redd_data;
};
