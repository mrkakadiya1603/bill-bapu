import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  transpilePackages: ["@restrosync/ui", "@restrosync/shared"],
  eslint: {
    ignoreDuringBuilds: true,
  },
};

export default nextConfig;
