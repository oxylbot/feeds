module.exports = async ({ reddit }) => ({
	reddit: await require("./reddit")(reddit.subs, reddit.type)
});
