/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export",
  reactStrictMode: true,
  webpack: (config, { isServer }) => {
    const wasmModuleFilename = isServer
      ? "./../static/wasm/[modulehash].wasm"
      : "static/wasm/[modulehash].wasm";
    config.output.webassemblyModuleFilename = wasmModuleFilename;
    config.experiments.asyncWebAssembly = true;

    return config;
  },
};

module.exports = nextConfig;
