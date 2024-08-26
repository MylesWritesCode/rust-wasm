module.exports = {
	reactStrictMode: true,
	webpack: (
		config,
		{ _buildId, _dev, _isServer, _defaultLoaders, _nextRuntime, _webpack },
	) => {
		config.experiments = {
			...config.experiments,
			syncWebAssembly: true,
		};

		return config;
	},
};
