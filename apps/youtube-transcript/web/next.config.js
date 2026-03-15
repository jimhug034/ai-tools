const createNextIntlPlugin = require("next-intl/plugin");

const withNextIntl = createNextIntlPlugin("./src/i18n.ts");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  webpack: (config, { isServer }) => {
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        net: false,
        tls: false,
        dns: false,
      };
    }
    if (isServer) {
      config.externals = config.externals || [];
      config.externals.push({
        undici: "commonjs undici",
        "youtubei.js": "commonjs youtubei.js",
      });
    }
    return config;
  },
};

module.exports = withNextIntl(nextConfig);
