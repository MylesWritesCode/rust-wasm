module.exports = {
	reactStrictMode: true,
	webpack: (
		config,
		{ _buildId, _dev, _isServer, _defaultLoaders, _nextRuntime, _webpack },
	) => {
		config.experiments = {
			...config.experiments,
			// This is required to be able to use wasm files in the project.
			// Unfortunately, Nextjs doesn't have the greatest support for WASM. With
			// Vite, all we had to do is import some plugins, and it worked out of
			// the box. For now, we'll have to do it like this.
			syncWebAssembly: true,
			// Currently causes issues with Next.js. It says the target environment
			// is not async, so enabling this will cause runtime errors.
			//
			// The generated code contains 'async/await' because this module is
			// using "asyncWebAssembly". However, your target environment does not
			// appear to support 'async/await'. As a result, the code may not run as
			// expected or may cause runtime errors. asyncWebAssembly: true,
			//
			// Not sure how to get around it, but that's not the main focus of this
			// project, so I'll leave it for now. I spent way too much time on logs
			// anyway.
			// asyncWebAssembly: true,
		};

		return config;
	},
};
